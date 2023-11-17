#[cfg(feature = "input")]
use std::io::{stdin, stdout, Write};
use crate::{Color, flush_styles};

#[cfg(feature = "input")]
#[doc(hidden)]
pub fn _read_input(prompt: String) -> String {
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

#[cfg(feature = "input")]
#[doc(hidden)]
pub fn _yn_prompt(p: String) -> bool {
    loop {
        let input = _read_input(format!("{} [Y/n]: ", p));
        match input.to_ascii_lowercase().as_str() {
            // "y" | "yes" | "" => return true,
            "n" | "no" => return false,
            _ => return true,
        }
    }
}

#[cfg(feature = "input")]
#[macro_export]
/// Returns a string from stdin with the prompt given
/// # Example
/// ```
/// use better_term::read_input;
///
/// // by default read_input() will prompt with nothing
/// let input = read_input!();
/// // specify a custom prompt with the same formatting as format!()
/// let input2 = read_input!("Enter a second value: ");
macro_rules! read_input {
    () =>  { $crate::_read_input(format!(""))};
    ($($arg:tt)*) =>  { $crate::_read_input(format!("{}", format_args!($($arg)*)))};
}

#[cfg(feature = "input")]
#[macro_export]
/// Works the same way as read_input!()
/// Returns true if the user types 'y', 'Y', or "yes" to the prompt
/// Returns false if the user types 'n', 'N', or "no" to the prompt
/// defaults to yes if the user presses enter
macro_rules! yesno_prompt {
    () =>  { $crate::_yn_prompt("")};
    ($($arg:tt)*) =>  { $crate::_yn_prompt(format!("{}", format_args!($($arg)*)))};
}