use std::env::current_dir;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn collect_dir_entries(input_dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(input_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    Ok(entries)
}

fn get_current_dir_entries() -> Vec<PathBuf> {
    let cdir = current_dir().unwrap();
    let e = collect_dir_entries(cdir.as_path());

    e.unwrap()
}

fn main() {
    let current = get_current_dir_entries();
    println!("{:?}", current);
}
