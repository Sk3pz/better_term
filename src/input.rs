use std::io::{stdin, stdout, Write};
use crate::{Color, flush_styles, read_input};

pub(crate) fn _read_input(prompt: String) -> String {
    print!("{}", prompt);
    let r = stdout().flush();
    if r.is_err() {
        panic!("Error flusing output: {}", r.unwrap_err());
    }
    let mut buffer = String::new();
    let r2 = stdin().read_line(&mut buffer);
    if r2.is_err() {
        panic!("Error in reading input: {}", r.unwrap_err());
    }
    buffer.replace("\n", "").replace("\r", "")
}

pub(crate) fn _yn_prompt(p: String) -> bool {
    loop {
        let input = read_input!("{} (Y or N): ", p);
        match input.to_ascii_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                println!("{}Warning: The input can only be Y or N!", Color::Yellow);
                flush_styles();
            }
        }
    }
}