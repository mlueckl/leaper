use clap::{command, Arg, ArgMatches};
use std::env::current_dir;
use std::fs::{read_dir, File, OpenOptions, ReadDir};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Handle and return CL arguments
pub fn args_handler() -> ArgMatches {
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

/// Initialize File with options
pub fn init_file(write: bool, create: bool, append: bool, path: &Path) -> io::Result<File> {
    OpenOptions::new()
        .write(write)
        .create(create)
        .append(append)
        .open(&path)
}

pub fn write_to_file(content: &str) -> io::Result<()> {
    let path = Path::new(current_dir().unwrap().as_path()).join("found.txt");

    if let Ok(mut file) = init_file(true, true, true, path.as_path()) {
        file.write(format!("{}\r", content).as_bytes())?;
    }

    Ok(())
}

/// Reads entries in provided directory
pub fn get_read_dir(input_dir: &Path, go_upward: bool) -> io::Result<ReadDir> {
    let dir = if go_upward {
        input_dir
            .parent()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Dir has no parent"))?
    } else {
        input_dir
    };

    read_dir(dir)
}

/// Extracts the found entries in directory
pub fn extract_entries(dir_entries: ReadDir) -> io::Result<Vec<PathBuf>> {
    dir_entries
        .filter_map(|entry| match entry {
            Ok(e) => Some(Ok(e.path())),
            Err(err) => Some(Err(err)),
        })
        .collect()
}

/// Return all entries for given location
pub fn get_entries(path: &Path, go_upward: bool) -> io::Result<Vec<PathBuf>> {
    match get_read_dir(path, go_upward) {
        Ok(dir_entries) => extract_entries(dir_entries),
        Err(err) => return Result::Err(err),
    }
}

fn find_target(entries: &[PathBuf], target: &str) -> Option<PathBuf> {
    for e in entries {
        if e.to_string_lossy().ends_with(target) {
            return Some(e.to_path_buf());
        }
    }

    None
}

/// Find target and return if found
pub fn find(mut entries: Vec<PathBuf>, target: &str, go_upward: bool) -> Option<PathBuf> {
    let mut found = find_target(&entries, target);

    while found.is_none() {
        if let Some(unsearched_entries) = follow(&entries, go_upward) {
            found = find_target(&unsearched_entries, target);
            entries = unsearched_entries;
        } else {
            // No new entries
            return None;
        }
    }

    if found.is_some() {
        return Some(found.unwrap().to_path_buf());
    }

    None
}

/// Follow directories and return unsearched entries
pub fn follow(entries: &[PathBuf], go_upward: bool) -> Option<Vec<PathBuf>> {
    let mut unsearched_entries: Vec<PathBuf> = Vec::new();

    // Ignore directories and files from entries, already established that target was not found
    for e in entries {
        // Follow directories and add their elements to unsearched_entries
        if e.is_dir() {
            if go_upward {
                if e.parent().is_some() {
                    return Some(vec![e.parent().unwrap().to_path_buf()]);
                }
            }

            if let Ok(sub_entries) = get_entries(e.as_path(), go_upward) {
                for se in sub_entries {
                    unsearched_entries.push(se);
                }
            }
        }
    }

    if unsearched_entries.is_empty() {
        return None;
    }

    Some(unsearched_entries)
}
