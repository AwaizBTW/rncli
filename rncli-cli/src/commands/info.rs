//! Info command handler

use crate::formatter::OutputFormatter;
use std::collections::HashMap;

pub async fn handle(
    formatter: &OutputFormatter,
    _verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut data = HashMap::new();
    
    data.insert("Application".to_string(), "rncli".to_string());
    data.insert("Version".to_string(), rncli_lib::VERSION.to_string());
    data.insert("Description".to_string(), "Modern Network Management, Powered by Rust".to_string());
    data.insert("Author".to_string(), "Awaiz Azam".to_string());
    data.insert("Repository".to_string(), "https://github.com/AwaizBTW/rncli".to_string());
    data.insert("License".to_string(), "Apache License 2.0".to_string());
    
    formatter.key_value(&data);
    formatter.success("Information displayed");
    
    Ok(())
}
