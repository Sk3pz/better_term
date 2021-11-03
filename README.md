# better_term
A rust crate designed to allow easy styling of terminal output using standard ANSI escape codes.

## Usage
### Style
A struct used to style output
```rust
use better_term::Style;

// prints out Hello world! underlined and bold
let style = Style::default().underline().bold();

println!("{}Hello, world!", style);
```

### Color
A struct used to be simple for just changing colors
```rust
use better_term::Color;

// prints Hello, world! in green and red
println!("{}Hello, {}world!", Color::BrightGreen, Color::BrightRed);
```

### Flushing Styles and Colors
It may be useful to reset all the changes you have made, and go back to the default output style. the `flush_styles()` function is meant for this.
```rust
use better_term::{flush_styles, rainbowify};

// prints the text in rainbow colors
println!("{}", rainbowify("This is rainbow!!!!!"));

// clear all colors and styles to reset to default
flush_styles();
```