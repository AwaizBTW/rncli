//! Output formatting for different formats (text, JSON, CSV)

use colored::Colorize;
use serde_json::json;
use std::collections::HashMap;

pub struct OutputFormatter {
    format: OutputFormat,
    use_color: bool,
}

#[derive(Debug, Clone, Copy)]
enum OutputFormat {
    Text,
    Json,
    Csv,
}

impl OutputFormat {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "json" => OutputFormat::Json,
            "csv" => OutputFormat::Csv,
            _ => OutputFormat::Text,
        }
    }
}

impl OutputFormatter {
    pub fn new(format: &str, use_color: bool) -> Self {
        Self {
            format: OutputFormat::from_str(format),
            use_color,
        }
    }

    /// Print status message
    pub fn status(&self, title: &str, value: &str) {
        match self.format {
            OutputFormat::Text => {
                let title = if self.use_color {
                    title.bold().cyan().to_string()
                } else {
                    title.to_string()
                };
                println!("{}: {}", title, value);
            }
            OutputFormat::Json => {
                let json = json!({ "status": title, "value": value });
                println!("{}", json.to_string());
            }
            OutputFormat::Csv => {
                println!("{},{}", title, value);
            }
        }
    }

    /// Print success message
    pub fn success(&self, message: &str) {
        match self.format {
            OutputFormat::Text => {
                if self.use_color {
                    println!("{} {}", "✓".green().bold(), message.green());
                } else {
                    println!("[✓] {}", message);
                }
            }
            OutputFormat::Json => {
                println!("{}", json!({ "status": "success", "message": message }));
            }
            OutputFormat::Csv => {
                println!("success,{}", message);
            }
        }
    }

    /// Print error message
    pub fn error(&self, message: &str) {
        match self.format {
            OutputFormat::Text => {
                if self.use_color {
                    eprintln!("{} {}", "✗".red().bold(), message.red());
                } else {
                    eprintln!("[✗] {}", message);
                }
            }
            OutputFormat::Json => {
                eprintln!("{}", json!({ "status": "error", "message": message }));
            }
            OutputFormat::Csv => {
                eprintln!("error,{}", message);
            }
        }
    }

    /// Print warning message
    pub fn warning(&self, message: &str) {
        match self.format {
            OutputFormat::Text => {
                if self.use_color {
                    println!("{} {}", "⚠".yellow().bold(), message.yellow());
                } else {
                    println!("[!] {}", message);
                }
            }
            OutputFormat::Json => {
                println!("{}", json!({ "status": "warning", "message": message }));
            }
            OutputFormat::Csv => {
                println!("warning,{}", message);
            }
        }
    }

    /// Print a table with headers
    pub fn table(&self, headers: Vec<&str>, rows: Vec<Vec<String>>) {
        match self.format {
            OutputFormat::Text => {
                self.print_text_table(&headers, &rows);
            }
            OutputFormat::Json => {
                let mut json_rows = Vec::new();
                for row in rows {
                    let mut obj = serde_json::Map::new();
                    for (i, header) in headers.iter().enumerate() {
                        if i < row.len() {
                            obj.insert(header.to_string(), serde_json::Value::String(row[i].clone()));
                        }
                    }
                    json_rows.push(serde_json::Value::Object(obj));
                }
                println!("{}", serde_json::to_string_pretty(&json_rows).unwrap());
            }
            OutputFormat::Csv => {
                println!("{}", headers.join(","));
                for row in rows {
                    println!("{}", row.join(","));
                }
            }
        }
    }

    /// Print text-based table
    fn print_text_table(&self, headers: &[&str], rows: &[Vec<String>]) {
        if rows.is_empty() {
            println!("No data to display");
            return;
        }

        // Calculate column widths
        let mut widths = vec![0; headers.len()];
        for (i, header) in headers.iter().enumerate() {
            widths[i] = header.len();
        }

        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(cell.len());
                }
            }
        }

        // Print header
        let header_line = headers
            .iter()
            .enumerate()
            .map(|(i, h)| format!("{:<width$}", h, width = widths[i]))
            .collect::<Vec<_>>()
            .join(" | ");

        if self.use_color {
            println!("{}", header_line.bold().cyan());
        } else {
            println!("{}", header_line);
        }

        // Print separator
        let separator = widths
            .iter()
            .map(|w| "-".repeat(w + 1))
            .collect::<Vec<_>>()
            .join("|");
        println!("{}", separator);

        // Print rows
        for row in rows {
            let row_line = row
                .iter()
                .enumerate()
                .map(|(i, cell)| format!("{:<width$}", cell, width = widths[i]))
                .collect::<Vec<_>>()
                .join(" | ");
            println!("{}", row_line);
        }
    }

    /// Print key-value pairs
    pub fn key_value(&self, data: &HashMap<String, String>) {
        match self.format {
            OutputFormat::Text => {
                for (key, value) in data {
                    let key_str = if self.use_color {
                        key.bold().cyan().to_string()
                    } else {
                        key.clone()
                    };
                    println!("{}: {}", key_str, value);
                }
            }
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&data).unwrap());
            }
            OutputFormat::Csv => {
                println!("key,value");
                for (key, value) in data {
                    println!("{},{}", key, value);
                }
            }
        }
    }

    /// Print list of items
    pub fn list(&self, title: &str, items: Vec<&str>) {
        match self.format {
            OutputFormat::Text => {
                if self.use_color {
                    println!("{}", title.bold().cyan());
                } else {
                    println!("{}", title);
                }
                for item in items {
                    println!("  • {}", item);
                }
            }
            OutputFormat::Json => {
                println!("{}", json!({ title: items }));
            }
            OutputFormat::Csv => {
                println!("{}", title);
                for item in items {
                    println!("{}", item);
                }
            }
        }
    }
}
