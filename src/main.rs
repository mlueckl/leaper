use clap::{command, Arg, ArgMatches};
use std::env::current_dir;

fn cli_handler() -> ArgMatches {
    let match_results: ArgMatches = command!()
        .about("A simple CLI tool to quickly leap to a directory")
        .arg(Arg::new("target").required(true))
        .arg(
            Arg::new("up")
                .action(clap::ArgAction::SetTrue)
                .short('u')
                .long("up")
                .help("Leap upwards to Parent directories"),
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

fn main() {
    let cli = cli_handler();

    let target = cli.get_one::<String>("target").unwrap().to_owned();
    let mut directory = leaper::Dirs::new(current_dir().unwrap(), target.to_string());
    directory = leaper::dir_get_current_entries(directory);

    let found = directory.find();

    if found.as_os_str().is_empty() == false {
        leaper::bash(format!("cd {} || exit 1\n$SHELL", found.display()));
        println!("Leaping to {}", found.display());
    } else {
        println!("{} not found", cli.get_one::<String>("target").unwrap());
    }
}
