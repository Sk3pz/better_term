pub mod style;

use std::process::Command;

pub fn system<S: Into<String>>(cmd: S) {
    Command::new(cmd.into()).status().expect("Command failed");
}