use std::io::{stdin, stdout, Write};
use crate::{Color, flush_styles};

#[cfg(test)]
mod tests {
    use crate::{Color, flush_styles, read_input, Style, yesno_prompt};

    #[test]
    fn test1() {
        let input = read_input!();
        println!("{}", input);

        let input2 = yesno_prompt!("Is this a question? ");
        println!("You were {}{}!", if input2 { format!("{}correct", Color::Green) }
        else { format!("{}wrong", Color::Red) }, Style::reset());
        flush_styles();
    }
}

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

#[cfg(feature = "io")]
#[macro_export]
/// Returns a string from stdin with the prompt given
macro_rules! read_input {
    () =>  { crate::io::_read_input(format!("> "))};
    ($($arg:tt)*) =>  { crate::io::_read_input(format!("{}", format_args!($($arg)*)))};
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

#[cfg(feature = "io")]
#[macro_export]
/// Returns true if the user types 'y', 'Y', or "yes" to the prompt
/// Returns false if the user types 'n', 'N', or "no" to the prompt
/// repeats prompt until user enters a valid input
macro_rules! yesno_prompt {
    () =>  { crate::io::_yn_prompt(format!("> "))};
    ($($arg:tt)*) =>  { crate::io::_yn_prompt(format!("{}", format_args!($($arg)*)))};
}