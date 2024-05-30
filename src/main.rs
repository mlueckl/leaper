use std::env::{args, current_dir};
use std::process;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        println!("Too many or no argument(s) provided!");
        process::exit(0);
    }

    let needle = String::from(&args[1]);
    let mut directory = leap::Dirs::new(current_dir().unwrap(), needle);
    directory = leap::dir_get_current_entries(directory);

    let found = directory.find();

    if found.as_os_str().is_empty() == false {
        leap::bash(format!("cd {} || exit 1\n$SHELL", found.display()));
        println!("Leaping to {}", found.display());
    } else {
        println!("{} not found", &args[1]);
    }
}
