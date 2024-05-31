use std::env::current_dir;

fn main() {
    let cli = leaper::cli_handler();

    let target = cli.get_one::<String>("target").unwrap().to_owned();
    let mut directory = leaper::Dirs::new(
        current_dir().unwrap(),
        target.to_string(),
        cli.get_flag("up"),
    );
    directory.dir_get_current_entries();

    let target_path = directory.find();

    if target_path.as_os_str().is_empty() == false {
        match cli.get_flag("path") {
            true => println!("Target Path: {}", target_path.display()),
            false => {
                leaper::bash(format!("cd {} || exit 1\n$SHELL", target_path.display()));
                println!("Leaping to {}", target_path.display());
            }
        }
    } else {
        println!("{} not found", cli.get_one::<String>("target").unwrap());
    }
}
