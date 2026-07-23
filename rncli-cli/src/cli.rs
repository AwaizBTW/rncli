//! CLI utilities and helpers

use std::io::{self, Write};

/// Prompt user for confirmation
pub fn confirm(message: &str) -> io::Result<bool> {
    print!("{} [y/N]: ", message);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(input.trim().to_lowercase() == "y")
}

/// Display a warning message
pub fn warn(message: &str) {
    eprintln!("⚠ Warning: {}", message);
}

/// Display an info message
pub fn info(message: &str) {
    println!("ℹ {}", message);
}
