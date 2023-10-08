use crate::utils::get_current_shell::get_current_shell;
use anyhow::Result;
use std::process::{Command, Stdio};
use system_shutdown::shutdown;

pub fn zenity() -> Result<()> {
    let shell = get_current_shell()?;
    let mut child = Command::new(shell)
        .arg("-c")
        .arg("for i in $(seq 1.00 0.5 100.00); do echo \"$i\"; sleep 0.3; done")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start child process");

    let mut zenity = Command::new("zenity")
        .arg("--progress")
        .arg("--title=Shutdown")
        .arg("--text=Your computer is shutting down...")
        .arg("--percentage=0")
        .arg("--auto-close")
        .stdin(child.stdout.take().unwrap())
        .spawn()
        .expect("Failed to start zenity command");

    let zenity_status = zenity.wait().expect("Failed to wait for zenity command");

    if zenity_status.code() == Some(1) {
        Command::new("zenity")
            .arg("--error")
            .arg("--text=Shutdown aborted.")
            .spawn()
            .expect("Failed to start error dialog");
    } else {
        match shutdown() {
            Ok(_) => (),
            Err(error) => eprintln!("Failed to shut down: {}", error),
        }
    }
    Ok(())
}
