use leaper::{find, get_entries, write_to_file};
use std::env::current_dir;

fn main() {
    // Handle CL arguments
    let args = leaper::args_handler();
    let target = args.get_one::<String>("target").unwrap();
    let go_upward = args.get_flag("up");

    // Get directories & files of current location
    if let Ok(entries) = get_entries(current_dir().unwrap().as_path(), go_upward) {
        if let Some(found) = find(entries, target, go_upward) {
            write_to_file(&found.display().to_string()).unwrap();

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
