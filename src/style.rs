use std::fmt::Display;
use std::fmt;

/// Default ansi colors
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
    Hex(u32)
}

impl Color {
    /// Convert color to a style with the foreground set to this color
    pub fn to_style_fg(&self) -> Style {
        Style::new().fg(*self)
    }
    /// Convert color to a style with the background set to this color
    pub fn to_style_bg(&self) -> Style {
        Style::new().bg(*self)
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
            Color::Hex(hex) => Color::hex_to_rgb(hex).as_fg()
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
            Color::Hex(hex) => Color::hex_to_rgb(hex).as_bg()
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_style_fg().fmt(f)
    }
}

/// A way to style output, by setting flags within this struct and then outputting it with ("... {} ...", style)
#[derive(PartialEq, Clone, Copy)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub overwrite: bool,
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub blink: bool,
    pub invert: bool,
    pub hide: bool,
    pub strikethrough: bool
}

impl Style {
    /// Creates a new Style with default values
    pub fn new() -> Style {
        Style::default()
    }

    /// sets the foreground
    pub fn fg(&self, c: Color) -> Style {
        Style { fg: Some(c), .. *self }
    }

    /// clears the foreground
    pub fn clear_fg(&self) -> Style {
        Style { fg: None, .. *self }
    }

    /// sets a new background
    pub fn bg(&self, c: Color) -> Style {
        Style { bg: Some(c), .. *self }
    }

    /// clear the background
    pub fn clear_bg(&self) -> Style {
        Style { bg: None, .. *self }
    }

    /// creates a new Style from the current style with overwriting enabled (designed for chaining)
    /// overwriting is to overwrite previous colors and styles, if on, it will set all unset values back to default,
    /// otherwise the style will only change what it is set to change
    pub fn overwrite(&self) -> Style {
        Style { overwrite: true, .. *self }
    }

    /// Set the output to bold
    pub fn bold(&self) -> Style {
        Style { bold: true, .. *self }
    }

    /// Set the output to be dim
    pub fn dim(&self) -> Style {
        Style { dim: true, .. *self }
    }

    /// Set the output to be italic
    pub fn italic(&self) -> Style {
        Style { italic: true, .. *self }
    }

    /// Set the output to be underlined
    pub fn underline(&self) -> Style {
        Style { underline: true, .. *self }
    }

    /// Set the output to blink (untested - can't find a terminal that works)
    pub fn blink(&self) -> Style {
        Style { blink: true, .. *self }
    }

    /// Inverts the current colors (bg and fg) through ansi (does not change fg and bg values)
    pub fn invert(&self) -> Style {
        Style { invert: true, .. *self }
    }

    /// hides the text (it's still there, just hidden.)
    pub fn hide(&self) -> Style {
        Style { hide: true, .. *self }
    }

    /// sets the text to be strike-through
    pub fn strikethrough(&self) -> Style {
        Style { strikethrough: true, .. *self }
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