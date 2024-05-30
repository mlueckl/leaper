use std::env::{args, current_dir};
use std::fs::read_dir;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

struct Dirs {
    base_path: PathBuf,
    entries: Vec<PathBuf>,
}

impl Dirs {
    fn new(base_path: PathBuf) -> Self {
        Self {
            base_path,
            entries: Vec::new(),
        }
    }
}

fn dir_collect_entries(input_dir: &Path) -> io::Result<Vec<PathBuf>> {
    let entries = read_dir(input_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    Ok(entries)
}

fn dir_get_current_entries() -> Dirs {
    let mut current_dir = Dirs::new(current_dir().unwrap());
    let collection = dir_collect_entries(current_dir.base_path.as_path());
    current_dir.entries.extend(collection.unwrap());

    current_dir
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        println!("Too many or no argument(s) provided!");
        process::exit(0);
    }

    let needle = &args[2];
    let directory = dir_get_current_entries();

    println!("Searching for {}", needle);

    for entry in &directory.entries {
        if entry.ends_with(needle) {
            println!("Leaping to {}", entry.display());
            leap::bash(format!("cd {} || exit 1\n$SHELL", entry.display()))
        }
    }
}
