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
