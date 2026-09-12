use std::fs;
use std::path::PathBuf;
use comfy_table::*;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use clap::Parser;
use ignored::is_ignored;
use phf::phf_map;
use std::time::{SystemTime, Duration};
use std::os::unix::fs::{PermissionsExt, MetadataExt, FileTypeExt};
use users::{get_user_by_uid, get_group_by_gid};

#[derive(Parser, Debug)]
#[command(about, version)]

struct Args {
    #[arg(default_value = ".")]
    path: PathBuf,

    #[arg(short, long, help = "Show hidden files")]
    all: bool,

    #[arg(short, long, help = "Hide files ignored by Git")]
    gitignore: bool,

    #[arg(short, long, help = "Show longer information")]
    long: bool,
}

static ICONS: phf::Map<&'static str, &'static str> = phf_map! {

    // Programming languages
    "rs" => "",
    "py" => "",
    "js" => "",
    "ts" => "",
    "jsx" => "",
    "tsx" => "",
    "c" => "",
    "h" => "",
    "cpp" => "",
    "java" => "",
    "kt" => "",
    "go" => "",
    "lua" => "",
    "rb" => "",
    "php" => "",
    "sh" => "",
    "bash" => "",
    "nix" => "󱄅",

    // Web
    "html" => "",
    "css" => "",
    "scss" => "",

    // Fonts
    "otf" => "",
    "ttf" => "",
    "woff2" => "",
    "woff" => "",

    // Data / config
    "json" => "",
    "yaml" => "",
    "yml" => "",
    "toml" => "",
    "xml" => "󰗀",
    "lock" => "",

    // Documentation
    "md" => "󰈙",
    "txt" => "󰈙",

    // Documents
    "pdf" => "󰈙",
    "docx" => "󰈙",
    "rtf" => "󰈙",
    "odt" => "󰈙",
    "epub" => "",
    "pptx" => "󰐨",
    "xlsx" => "󰧷",

    // Archives
    "zip" => "",
    "7z" => "",
    "rar" => "",
    "tar" => "",
    "gz" => "",
    "xz" => "",
    "bz2" => "",
    "zst" => "",

    // Images
    "png" => "",
    "jpg" => "",
    "jpeg" => "",
    "gif" => "",
    "svg" => "",
    "webp" => "",
    "ico" => "",
    "bmp" => "",
    "avif" => "",

    // Audio / video
    "mp3" => "",
    "wav" => "",
    "ogg" => "",
    "flac" => "",
    "m4a" => "",
    "opus" => "",
    "mp4" => "",
    "mkv" => "",
    "mov" => "",

    // Disk images
    "iso" => "",
    "img" => "",
};

static SPECIAL_ICONS: phf::Map<&'static str, &'static str> = phf_map! {
    "Dockerfile" => "",
    "Makefile" => "",
    ".gitignore" => "",
    ".env" => "",
};

fn file_kind(path: &std::path::Path) -> &'static str {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            let file_type = metadata.file_type();

            if file_type.is_symlink() {
                if fs::metadata(path).is_ok() {
                    "symlink"
                } else {
                    "broken symlink"
                }
            } else if file_type.is_dir() {
                "dir"
            } else if file_type.is_file() {
                "file"
            } else if file_type.is_fifo() {
                "fifo"
            } else if file_type.is_socket() {
                "socket"
            } else if file_type.is_char_device() {
                "char device"
            } else if file_type.is_block_device() {
                "block device"
            } else {
                "unknown"
            }
        }
        Err(_) => "unknown",
    }
}

fn icon_for(path: &std::path::Path, name: &str, kind: &str) -> &'static str {
    match kind {
        "dir" => return "󰉋",
        "symlink" => return "",
        "broken symlink" => return "",
        "fifo" => return "",
        "socket" => return "",
        "char device" => return "",
        "block device" => return "",
        _ => {}
    }

    if let Some(icon) = SPECIAL_ICONS.get(name) {
        return icon;
    }

    path.extension()
        .and_then(|ext| ext.to_str())
        .and_then(|ext| ICONS.get(ext))
        .copied()
        .unwrap_or("󰈔")
}

fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];

    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1000.0 && unit < UNITS.len() - 1 {
        size /= 1000.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{size:.0} {}", UNITS[unit])
    } else {
        format!("{size:.1} {}", UNITS[unit])
    }
}

fn relative_time(time: SystemTime) -> String {
    let elapsed = SystemTime::now()
        .duration_since(time)
        .unwrap_or(Duration::ZERO);

    let seconds = elapsed.as_secs();

    match seconds {
        0..=59 => format!("{seconds} sec ago"),
        60..=3599 => format!("{} min ago", seconds / 60),
        3600..=86399 => format!("{} hr ago", seconds / 3600),
        86400..=2591999 => format!("{} days ago", seconds / 86400),
        2592000..=31535999 => format!("{} months ago", seconds / 2592000),
        _ => format!("{} years ago", seconds / 31536000),
    }
}

fn permissions_string(path: &std::path::Path) -> std::io::Result<String> {
    let mode = fs::metadata(path)
        .or_else(|_| fs::symlink_metadata(path))?
        .permissions()
        .mode();

    let file_type = match file_kind(path) {
        "dir" => "d",
        "symlink" | "broken symlink" => "l",
        "fifo" => "p",
        "socket" => "s",
        "char device" => "c",
        "block device" => "b",
        _ => "-",
    };

    let mut result = format!("\x1b[33m{file_type}\x1b[0m");

    for shift in [6, 3, 0] {
        let read = (mode >> shift) & 4 != 0;
        let write = (mode >> shift) & 2 != 0;
        let execute = (mode >> shift) & 1 != 0;

        result.push_str(&format!(
            "\x1b[32m{}\x1b[0m",
            if read { "r" } else { "-" }
        ));

        result.push_str(&format!(
            "\x1b[34m{}\x1b[0m",
            if write { "w" } else { "-" }
        ));

        result.push_str(&format!(
            "\x1b[35m{}\x1b[0m",
            if execute { "x" } else { "-" }
        ));
    }

    Ok(result)
}

fn main() -> std::io::Result<()> {

        let args = Args::parse();
        let directory = args.path;

        let mut files = Vec::new();

        for entry in fs::read_dir(&directory)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let path = entry.path();

        if !args.all && name.starts_with(".") {
            continue;
        }

        if args.gitignore && is_ignored!(&path) {
            continue;
        }

        files.push((name, path));
    }

    let mut table = Table::new();

    let mut headers = vec![
        Cell::new("#").fg(Color::Green),
        Cell::new("name").fg(Color::Green).set_alignment(CellAlignment::Center),
        Cell::new("type").fg(Color::Green).set_alignment(CellAlignment::Center),
        Cell::new("size").fg(Color::Green).set_alignment(CellAlignment::Center),
        Cell::new("modified").fg(Color::Green).set_alignment(CellAlignment::Center),
    ];

    if args.long {
        headers.insert(
            1,
            Cell::new("permissions")
                .fg(Color::Green)
                .set_alignment(CellAlignment::Center),
        );
        headers.insert(
            2,
            Cell::new("owner")
                .fg(Color::Green)
                .set_alignment(CellAlignment::Center),
        );
        headers.insert(
            3,
            Cell::new("group")
                .fg(Color::Green)
                .set_alignment(CellAlignment::Center),
        );
    }

    headers.push(Cell::new("#").fg(Color::Green));

    table
        .load_style(UTF8_FULL_CONDENSED.with_rounded_corners())
        .set_header(headers);

    for (i, (file, path)) in files.iter().enumerate() {
        let metadata = match fs::metadata(path).or_else(|_| fs::symlink_metadata(path)) {
            Ok(m) => m,
            Err(_) => continue,
        };

        let kind = file_kind(path);
        let icon = icon_for(path, file, kind);

        let owner = get_user_by_uid(metadata.uid())
            .map(|u| u.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| metadata.uid().to_string());

        let group = get_group_by_gid(metadata.gid())
            .map(|g| g.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| metadata.gid().to_string());

        let size = human_size(metadata.len());
        let modified = relative_time(metadata.modified()?);

        let mut row = vec![
            Cell::new((i + 1).to_string()).fg(Color::Green),
        ];

        if args.long {
            row.push(Cell::new(permissions_string(path)?));
            row.push(Cell::new(owner));
            row.push(Cell::new(group));
        }

        row.push(match kind {
            "dir" => Cell::new(format!("{icon} {file}")).fg(Color::Blue),
            "symlink" => Cell::new(format!("{icon} {file}")).fg(Color::Cyan),
            "broken symlink" => Cell::new(format!("{icon} {file}")).fg(Color::Red),
            "fifo" | "socket" | "char device" | "block device" => {
                Cell::new(format!("{icon} {file}")).fg(Color::Yellow)
            }
            _ => Cell::new(format!("{icon} {file}")),
        });

        row.push(Cell::new(kind));
        row.push(Cell::new(size).fg(Color::Cyan));
        row.push(Cell::new(modified).fg(Color::Magenta));
        row.push(Cell::new((i + 1).to_string()).fg(Color::Green));

        table.add_row(row);
    }

    table.column_mut(0)
        .unwrap()
        .set_cell_alignment(CellAlignment::Right);

    println!("{table}");
    Ok(())
}
