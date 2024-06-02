use clap::{command, Arg, ArgMatches};
use std::fs::read_dir;
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

/// Return all entries for given location
pub fn get_entries(path: &Path, is_upward: bool) -> Option<Vec<PathBuf>> {
    dir_collect_entries(path, is_upward)
}

/// Find target and return if found
pub fn find(mut entries: Vec<PathBuf>, target: &str, is_upward: bool) -> Option<PathBuf> {
    let mut found = find_target(&entries, target);

    while found.is_none() {
        if let Some(unsearched_entries) = follow(&entries, is_upward) {
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
pub fn follow(entries: &[PathBuf], is_upward: bool) -> Option<Vec<PathBuf>> {
    let mut unsearched_entries: Vec<PathBuf> = Vec::new();

    // Ignore directories and files from entries, already established that target was not found
    for e in entries {
        // Follow directories and add their elements to unsearched_entries
        if e.is_dir() {
            if is_upward {
                if e.parent().is_some() {
                    return Some(vec![e.parent().unwrap().to_path_buf()]);
                }
            }

            if let Some(sub_entries) = get_entries(e.as_path(), is_upward) {
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

fn find_target(entries: &[PathBuf], target: &str) -> Option<PathBuf> {
    for e in entries {
        if e.to_string_lossy().ends_with(target) {
            return Some(e.to_path_buf());
        }
    }

    None
}

fn dir_collect_entries(mut input_dir: &Path, upward: bool) -> Option<Vec<PathBuf>> {
    if input_dir.parent().is_none() {
        return None;
    }

    input_dir = match upward {
        true => input_dir.parent().unwrap(),
        false => input_dir,
    };

    let mut entries = Vec::new();

    match read_dir(input_dir) {
        Ok(dirs) => {
            for entry in dirs {
                match entry {
                    Ok(e) => {
                        entries.push(e.path());
                    }
                    Err(err) => println!("{}", err),
                }
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    Some(entries)
}
