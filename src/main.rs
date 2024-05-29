use std::env::current_dir;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

struct Dirs {
    base_path: PathBuf,
    directories: Vec<PathBuf>,
    files: Vec<PathBuf>,
}

impl Dirs {
    fn new(base_path: PathBuf) -> Self {
        Self {
            base_path,
            directories: Vec::new(),
            files: Vec::new(),
        }
    }

    fn parse(&mut self, entries: Vec<PathBuf>) {
        for e in entries {
            match e.is_dir() {
                true => self.directories.push(e),
                false => self.files.push(e),
            }
        }
    }
}

fn dir_collect_entries(input_dir: &Path) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(input_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    Ok(entries)
}

fn dir_get_current_entries() -> Dirs {
    let mut current_dir = Dirs::new(current_dir().unwrap());
    let collection = dir_collect_entries(current_dir.base_path.as_path());
    current_dir.parse(collection.unwrap());

    current_dir
}

fn main() {
    let current_dir = dir_get_current_entries();
    println!("Directories: {:?}", current_dir.directories);
    println!("Files: {:?}", current_dir.files);
}
