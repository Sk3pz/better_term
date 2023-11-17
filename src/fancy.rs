/// Generates a gradient from start to end with size steps
/// # Example
/// ```
/// use better_term::fancy::gradient;
///
/// // prints a gradient from red to green with 10 steps
/// let gradient = gradient((255, 0, 0), (0, 255, 0), 10);
/// for color in gradient {
///    println!("{}Hello, world!", color);
/// }
/// ```
pub fn gradient(start: (u8, u8, u8), end: (u8, u8, u8), size: usize) -> Vec<crate::Color> {
    // Calculate the step size for each color channel
    let step_r = (end.0 as i16 - start.0 as i16) as f64 / size as f64;
    let step_g = (end.1 as i16 - start.1 as i16) as f64 / size as f64;
    let step_b = (end.2 as i16 - start.2 as i16) as f64 / size as f64;

    // Generate the gradient
    let mut gradient = Vec::new();
    for i in 0..size {
        let r = (start.0 as f64 + i as f64 * step_r).round() as u8;
        let g = (start.1 as f64 + i as f64 * step_g).round() as u8;
        let b = (start.2 as f64 + i as f64 * step_b).round() as u8;
        gradient.push(crate::Color::RGB(r, g, b));
    }

    gradient
}

/// Prints text with a gradient, accounting for multiple lines
pub fn print_gradient<S: Into<String>>(text: S, gradient_start: (u8, u8, u8), gradient_end: (u8, u8, u8)) {
    let text = text.into();
    // handle multiline as well
    let split = text.split('\n').collect::<Vec<&str>>();
    let length = split.iter().map(|s| s.len()).max().unwrap();

    let gradient = gradient(gradient_start, gradient_end, length);

    for (i, line) in text.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let color_index = (i + j) % gradient.len();
            print!("{}{}", gradient[color_index], c);
        }
        println!();
    }
}