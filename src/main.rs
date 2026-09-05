use std::fs;
use std::env;
use std::path::PathBuf;
use comfy_table::*;
use comfy_table::presets::UTF8_FULL;

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().skip(1).collect();
    let path_str = args.first().map(|s| s.as_str()).unwrap_or(".");
    let directory = PathBuf::from(path_str);

    let mut files = Vec::new();

    for entry in fs::read_dir(&directory)? {
        let entry = entry?;
        files.push(entry.file_name().to_string_lossy().into_owned());
    }

    // Indexes list (like the numbers for the files)
    let indexes: Vec<_> = (1..=files.len()).collect();
    println!("{:?}", indexes);

    let mut table = Table::new();
    table
        .load_style(UTF8_FULL.with_rounded_corners())
        .set_header(vec!["#", "File"]);

    for (i, file) in files.iter().enumerate() {
        table.add_row(vec![(i + 1).to_string(), file.to_string()]);
    }

    println!("{table}");
    println!("{:?}", directory);
    println!("{:?}", files);
    Ok(())
}
