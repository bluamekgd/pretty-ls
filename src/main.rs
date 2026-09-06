use std::fs;
use std::path::PathBuf;
use comfy_table::*;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use clap::Parser;
use ignored::is_ignored;

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

fn main() -> std::io::Result<()> {

    let args = Args::parse();
    let directory = args.path;

    let mut files = Vec::new();

    for entry in fs::read_dir(&directory)? {
        let entry = entry?;
        let fname = entry.file_name().to_string_lossy().into_owned();
        let fpath = entry.path();

        if !args.all && fname.starts_with(".") {
            continue;
        }
        if args.gitignore && is_ignored!(&fpath) {
            continue;
        }
        files.push(fname.clone());
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
