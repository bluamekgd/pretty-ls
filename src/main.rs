use std::fs;
use std::path::PathBuf;
use comfy_table::*;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use clap::Parser;
use ignored::is_ignored;
use phf::phf_map;

#[derive(Parser, Debug)]
#[command(about, version)]

struct Args {
    #[arg(default_value = ".")]
    path: PathBuf,

    #[arg(short, long, help = "Show hidden files")]
    all: bool,

    #[arg(short, long, help = "Hide files ignored by Git")]
    gitignore: bool,
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
    "lock" => "",

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
        files.push(name.clone());
    }

    let mut table = Table::new();
    table
        .load_style(UTF8_FULL_CONDENSED.with_rounded_corners())
        .set_header(vec!["#", "File", "#"]);

    for (i, file) in files.iter().enumerate() {
        table.add_row(vec![(i + 1).to_string(), file.to_string(), (i + 1).to_string()]);
    }

    println!("{table}");
    Ok(())
}
