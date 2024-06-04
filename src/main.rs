use leaper::{find, get_entries};
use std::env::current_dir;

fn main() {
    // Handle CL arguments
    let args = leaper::args_handler();
    let target = args.get_one::<String>("target").unwrap();
    let is_upward = args.get_flag("up");

    // Get directories & files of current location
    if let Ok(entries) = get_entries(current_dir().unwrap().as_path(), is_upward) {
        if let Some(found) = find(entries, target, is_upward) {
            if found.is_file() {
                println!("{}", found.parent().unwrap().display());
            } else {
                println!("{}", found.display());
            }
        } else {
            println!("");
        }
    }
}
