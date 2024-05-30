use std::fs::read_dir;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn bash(cmd: String) {
    let mut command = Command::new("bash");
    command.arg("--login").arg("-c").arg(cmd);

    command
        .spawn()
        .expect("Failed to spawn command")
        .wait_with_output()
        .expect("Failed to wait for command");
}

fn dir_collect_entries(input_dir: &Path) -> io::Result<Vec<PathBuf>> {
    let entries = read_dir(input_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    Ok(entries)
}

pub fn dir_get_current_entries(mut current_dir: Dirs) -> Dirs {
    if let Ok(collection) = dir_collect_entries(current_dir.base_path.as_path()) {
        current_dir.entries.extend(collection);
    };

    current_dir
}
#[derive(Debug)]
pub struct Dirs {
    base_path: PathBuf,
    entries: Vec<PathBuf>,
    needle: String,
    dirs_followed: bool,
}

impl Dirs {
    pub fn new(base_path: PathBuf, needle: String) -> Self {
        Self {
            base_path,
            entries: Vec::new(),
            needle,
            dirs_followed: false,
        }
    }

    fn _follow_dirs(&mut self) {
        let mut new_entries = Vec::new();
        let mut had_dirs = false;

        for e in self.entries.drain(..) {
            if e.is_dir() {
                if let Ok(dir_entries) = dir_collect_entries(e.as_path()) {
                    new_entries.extend(dir_entries);
                }
                had_dirs = true;
            } else {
                new_entries.push(e);
            }
        }

        if had_dirs == false {
            self.dirs_followed = true;
        }

        self.entries = new_entries;
    }

    fn _exists(&self) -> PathBuf {
        for entry in &self.entries {
            if entry.ends_with(&self.needle) {
                return entry.to_owned();
            }
        }

        PathBuf::new()
    }

    pub fn find(mut self) -> PathBuf {
        while self.dirs_followed == false {
            let exists = self._exists();
            if exists.as_os_str().is_empty() == false {
                return exists;
            }

            self._follow_dirs();
        }

        PathBuf::new()
    }
}
