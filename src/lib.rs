use std::fmt::Display;
use std::fmt;

/// Default ansi colors
/// # Example
/// ```
/// use better_term::Color;
///
/// // prints Hello, world! in green and red
/// println!("{}Hello, {}world!", Color::BrightGreen, Color::BrightRed);
/// ```
#[derive(PartialEq, Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightPurple,
    BrightCyan,
    BrightWhite,
    Fixed(u8),
    RGB(u8,u8,u8),
    Hex(u32),
    Reset,
}

impl Color {
    /// Convert color to a style with the foreground set to this color
    pub fn to_style_fg(self) -> Style {
        Style::new().fg(self)
    }
    /// Convert color to a style with the background set to this color
    pub fn to_style_bg(self) -> Style {
        Style::new().bg(self)
    }

    /// convert a hexadecimal value to an RGB color value (hex example: 0x000000 for black)
    pub fn hex_to_rgb(hex: u32) -> Color {
        Color::RGB(((hex >> (16u8)) & 0xFF) as u8, ((hex >> (8u8)) & 0xFF) as u8, ((hex) & 0xFF) as u8)
    }

    /// Convert to the ansi value within a string (for output with the ansi char)
    pub fn as_fg(&self) -> String {
        match *self {
            Color::Black  => String::from("30"),
            Color::Red    => String::from("31"),
            Color::Green  => String::from("32"),
            Color::Yellow => String::from("33"),
            Color::Blue   => String::from("34"),
            Color::Purple => String::from("35"),
            Color::Cyan   => String::from("36"),
            Color::White  => String::from("37"),
            Color::BrightBlack  => String::from("90"),
            Color::BrightRed    => String::from("91"),
            Color::BrightGreen  => String::from("92"),
            Color::BrightYellow => String::from("93"),
            Color::BrightBlue   => String::from("94"),
            Color::BrightPurple => String::from("95"),
            Color::BrightCyan   => String::from("96"),
            Color::BrightWhite  => String::from("97"),
            Color::Fixed(u) => String::from(format!("38;5;{}", &u).to_string()),
            Color::RGB(r,g,b) => String::from(format!("38;2;{};{};{}", &r,&g,&b).to_string()),
            Color::Hex(hex) => Color::hex_to_rgb(hex).as_fg(),
            Color::Reset => String::from("37"),
        }
    }
    /// Convert to the ansi value within a string (for output with the ansi char
    pub fn as_bg(&self) -> String {
        match *self {
            Color::Black  => String::from("40"),
            Color::Red    => String::from("41"),
            Color::Green  => String::from("42"),
            Color::Yellow => String::from("43"),
            Color::Blue   => String::from("44"),
            Color::Purple => String::from("45"),
            Color::Cyan   => String::from("46"),
            Color::White  => String::from("47"),
            Color::BrightBlack  => String::from("100"),
            Color::BrightRed    => String::from("101"),
            Color::BrightGreen  => String::from("102"),
            Color::BrightYellow => String::from("103"),
            Color::BrightBlue   => String::from("104"),
            Color::BrightPurple => String::from("105"),
            Color::BrightCyan   => String::from("106"),
            Color::BrightWhite  => String::from("107"),
            Color::Fixed(u) => String::from(format!("48;5;{}", &u).to_string()),
            Color::RGB(r,g,b) => String::from(format!("48;2;{};{};{}", &r,&g,&b).to_string()),
            Color::Hex(hex) => Color::hex_to_rgb(hex).as_bg(),
            Color::Reset => String::from("40"),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_style_fg().fmt(f)
    }
}

/// A way to style output, by setting flags within this struct and then outputting it with ("... {} ...", style)
///
/// # Example
/// ```
/// use better_term::Style;
///
/// // prints out Hello world! underlined and bold
/// let style = Style::default().underline().bold();
///
/// println!("{}Hello, world!", style);
/// ```
#[derive(PartialEq, Clone, Copy)]
pub struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
    overwrite: bool,
    bold: bool,
    dim: bool,
    italic: bool,
    underline: bool,
    blink: bool,
    invert: bool,
    hide: bool,
    strikethrough: bool
}

impl Style {
    /// Creates a new Style with default values
    pub fn new() -> Style {
        Style::default()
    }

    /// sets the foreground
    pub fn fg(self, c: Color) -> Style {
        Style { fg: Some(c), .. self }
    }

    /// clears the foreground
    pub fn clear_fg(self) -> Style {
        Style { fg: None, .. self }
    }

    /// sets a new background
    pub fn bg(self, c: Color) -> Style {
        Style { bg: Some(c), .. self }
    }

    /// clear the background
    pub fn clear_bg(self) -> Style {
        Style { bg: None, .. self }
    }

    /// this will overwrite previous styles with default values
    pub fn overwrite(self) -> Style {
        Style { overwrite: true, .. self }
    }

    /// Set the output to bold
    pub fn bold(self) -> Style {
        Style { bold: true, .. self }
    }

    /// Set the output to be dim
    pub fn dim(self) -> Style {
        Style { dim: true, .. self }
    }

    /// Set the output to be italic
    pub fn italic(self) -> Style {
        Style { italic: true, .. self }
    }

    /// Set the output to be underlined
    pub fn underline(self) -> Style {
        Style { underline: true, .. self }
    }

    /// Set the output to blink
    /// This is not supported in most terminals
    pub fn blink(self) -> Style {
        Style { blink: true, .. self }
    }

    /// Inverts the current colors (bg and fg) through ansi (does not change fg and bg values)
    pub fn invert(self) -> Style {
        Style { invert: true, .. self }
    }

    /// hides the text (it's still there, just hidden.)
    pub fn hide(self) -> Style {
        Style { hide: true, .. self }
    }

    /// sets the text to be strike-through
    pub fn strikethrough(self) -> Style {
        Style { strikethrough: true, .. self }
    }

    #[doc(hidden)]
    fn gen(&self) -> String {
        let mut s = String::from("\x1b[");
        let mut has_written = false;

        {
            let mut write_c = |c| {
                if has_written { s += ";"; }
                has_written = true;
                s += c;
            };

            if self.overwrite { write_c("0") }

            if self.bold          { write_c("1"); }
            if self.dim           { write_c("2"); }
            if self.italic        { write_c("3"); }
            if self.underline     { write_c("4"); }
            if self.blink         { write_c("5"); }
            if self.invert        { write_c("7"); }
            if self.hide          { write_c("8"); }
            if self.strikethrough { write_c("9"); }
        }

        if let Some(bg) = self.bg {
            if has_written { s += ";"; }
            has_written = true;
            s += bg.as_bg().as_str();
        }

        if let Some(fg) = self.fg {
            if has_written { s += ";"; }
            s += fg.as_fg().as_str();
        }

        s += "m";

        s
    }
}

impl Default for Style {
    /// Get the default values for a Style
    fn default() -> Self {
        Style {
            fg: None,
            bg: None,
            overwrite: false,
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            blink: false,
            invert: false,
            hide: false,
            strikethrough: false,
        }
    }
}

impl Display for Style {
    #[doc(hidden)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.gen())
    }
}

#[doc(hidden)]
const RAINBOW: [Color; 6] = [Color::BrightRed, Color::Yellow, Color::BrightYellow, Color::BrightGreen, Color::Cyan, Color::BrightPurple];

/// This function takes a string and makes all the characters rainbow
///
/// # Example:
/// ```
/// use better_term::rainbowify;
///
/// // this will print "Hello, World!" in rainbow colors in most terminals
/// println!("{}", rainbowify("Hello, World!"));
/// ```
pub fn rainbowify<S: Into<String>>(s: S) -> String {
    let mut new_str = String::new();
    let chars = s.into().chars().collect::<Vec<char>>();
    let mut i: u8 = 0;
    for x in 0..chars.len() {
        let c = chars.get(x).unwrap();
        new_str += &format!("{}{}", RAINBOW[i as usize], c);
        if c >= &(33 as char) && c <= &(126 as char)  {
            i+=1;
            if i == 6 { i = 0; }
        }
    }
    new_str
}

/// this will reset all colors and styles in the console for future output
///
/// # Example:
/// ```
/// use better_term::{flush_styles, rainbowify};
///
/// // this will print in rainbow colors
/// println!("{}", rainbowify("This is rainbow!"));
///
/// // clear all colors and styles from the terminal to ensure the next output is normal
/// flush_styles();
/// println!("This is normal!");
/// ```
pub fn flush_styles() {
    print!("{}", Style::default());
}