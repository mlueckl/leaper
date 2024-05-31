use clap::{command, Arg, ArgMatches};
use std::fs::read_dir;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn cli_handler() -> ArgMatches {
    let match_results: ArgMatches = command!()
        .about("A simple CLI tool to quickly leap to a directory")
        .arg(Arg::new("target").required(true))
        .arg(
            Arg::new("up")
                .action(clap::ArgAction::SetTrue)
                .short('u')
                .long("up")
                .help("Leap upwards to Parent directories without following any subfolders"),
        )
        .arg(
            Arg::new("path")
                .action(clap::ArgAction::SetTrue)
                .short('p')
                .long("path")
                .help("Return Path without leaping"),
        )
        .get_matches();

    match_results
}

pub fn bash(cmd: String) {
    let mut command = Command::new("bash");
    command.arg("--login").arg("-c").arg(cmd);

    command
        .spawn()
        .expect("Failed to spawn command")
        .wait_with_output()
        .expect("Failed to wait for command");
}

fn dir_collect_entries(mut input_dir: &Path, upward: bool) -> io::Result<Vec<PathBuf>> {
    if input_dir.parent().is_none() {
        return Ok(Vec::new());
    }

    input_dir = match upward {
        true => input_dir.parent().unwrap(),
        false => input_dir,
    };

    let entries = read_dir(input_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    Ok(entries)
}

#[derive(Debug)]
pub struct Dirs {
    base_path: PathBuf,
    entries: Vec<PathBuf>,
    target: String,
    upward: bool,
    dirs_followed: bool,
}

impl Dirs {
    pub fn new(base_path: PathBuf, target: String, upward: bool) -> Self {
        Self {
            base_path,
            entries: Vec::new(),
            target,
            upward,
            dirs_followed: false,
        }
    }

    pub fn dir_get_current_entries(&mut self) {
        if let Ok(collection) = dir_collect_entries(self.base_path.as_path(), self.upward) {
            self.entries.extend(collection);
        };
    }

    fn _follow_dirs(&mut self) {
        let mut new_entries = Vec::new();
        let mut had_dirs = false;

        for e in self.entries.drain(..) {
            if e.is_dir() {
                match self.upward {
                    true => {
                        if let Ok(dir_entries) =
                            dir_collect_entries(e.parent().unwrap(), self.upward)
                        {
                            new_entries.extend(dir_entries);
                            had_dirs = true;
                            break;
                        }
                    }
                    false => {
                        if let Ok(dir_entries) = dir_collect_entries(e.as_path(), self.upward) {
                            new_entries.extend(dir_entries);
                        }
                    }
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
            if entry.ends_with(&self.target) {
                if entry.is_file() {
                    return entry.parent().unwrap().to_path_buf();
                }
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
