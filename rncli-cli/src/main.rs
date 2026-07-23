use clap::{Parser, Subcommand};
use colored::*;
use rncli_lib::NetworkManager;
use std::io::{self, Write};

mod cli;
mod formatter;
mod commands;

use commands::*;
use formatter::OutputFormatter;

/// Modern Network Management, Powered by Rust
#[derive(Parser)]
#[command(name = "rncli")]
#[command(version = "0.1.0")]
#[command(about = "Modern Network Management, Powered by Rust", long_about = None)]
#[command(author = "Awaiz Azam")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Disable colored output
    #[arg(long)]
    no_color: bool,

    /// Force use of sudo
    #[arg(long)]
    sudo: bool,

    /// Output format: text, json, or csv
    #[arg(short, long, default_value = "text")]
    output: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show network status
    Status,

    /// Manage connections
    #[command(subcommand)]
    Connections(ConnectionsCmd),

    /// Manage devices
    #[command(subcommand)]
    Devices(DevicesCmd),

    /// Manage WiFi
    #[command(subcommand)]
    WiFi(WiFiCmd),

    /// Enable or disable networking
    Networking {
        #[command(subcommand)]
        command: NetworkingCmd,
    },

    /// Block or unblock websites
    #[command(subcommand)]
    Block(BlockCmd),

    /// Show configuration and diagnostics
    Info,
}

#[derive(Subcommand)]
enum ConnectionsCmd {
    /// List all connections
    List,

    /// Show active connections
    Active,

    /// Activate a connection
    Activate {
        /// Connection name or UUID
        connection: String,

        /// Specific device to activate on
        #[arg(short, long)]
        device: Option<String>,
    },

    /// Deactivate a connection
    Deactivate {
        /// Connection name or UUID
        connection: String,
    },

    /// Show connection details
    Show {
        /// Connection name or UUID
        connection: String,
    },

    /// Delete a connection
    Delete {
        /// Connection name or UUID
        connection: String,
    },

    /// Edit a connection
    Edit {
        /// Connection name or UUID
        connection: String,
    },
}

#[derive(Subcommand)]
enum DevicesCmd {
    /// List all devices
    List,

    /// Show device details
    Show {
        /// Interface name
        interface: String,
    },

    /// Reapply connection settings to a device
    Reapply {
        /// Interface name
        interface: String,
    },

    /// Disconnect device
    Disconnect {
        /// Interface name
        interface: String,
    },
}

#[derive(Subcommand)]
enum WiFiCmd {
    /// List available WiFi networks
    List,

    /// Scan for WiFi networks
    Scan {
        /// Specific interface to scan
        #[arg(short, long)]
        interface: Option<String>,
    },

    /// Connect to a WiFi network
    Connect {
        /// SSID of the network
        ssid: String,

        /// Password (will prompt if not provided)
        #[arg(short, long)]
        password: Option<String>,
    },

    /// Disconnect from WiFi
    Disconnect {
        /// Specific interface to disconnect
        #[arg(short, long)]
        interface: Option<String>,
    },

    /// Forget a WiFi network
    Forget {
        /// SSID to forget
        ssid: String,
    },

    /// Enable or disable WiFi
    #[command(subcommand)]
    Radio(WiFiRadioCmd),
}

#[derive(Subcommand)]
enum WiFiRadioCmd {
    /// Enable WiFi
    On,

    /// Disable WiFi
    Off,
}

#[derive(Subcommand)]
enum NetworkingCmd {
    /// Enable networking
    On,

    /// Disable networking
    Off,
}

#[derive(Subcommand)]
enum BlockCmd {
    /// Block a website
    Block {
        /// Domain or URL (e.g., example.com or https://example.com)
        target: String,
    },

    /// Unblock a website
    Unblock {
        /// Domain or URL
        target: String,
    },

    /// List blocked websites
    List,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let cli = Cli::parse();

    // Initialize NetworkManager
    let nm = if cli.sudo {
        NetworkManager::new(true)
    } else {
        NetworkManager::with_auto_sudo()
    };

    let formatter = OutputFormatter::new(&cli.output, !cli.no_color);

    // Handle commands
    match cli.command {
        Commands::Status => {
            status::handle(&nm, &formatter, cli.verbose).await?;
        }
        Commands::Connections(cmd) => {
            connections::handle(&nm, cmd, &formatter, cli.verbose).await?;
        }
        Commands::Devices(cmd) => {
            devices::handle(&nm, cmd, &formatter, cli.verbose).await?;
        }
        Commands::WiFi(cmd) => {
            wifi::handle(&nm, cmd, &formatter, cli.verbose).await?;
        }
        Commands::Networking { command } => {
            networking::handle(&nm, command, &formatter, cli.verbose).await?;
        }
        Commands::Block(cmd) => {
            blocking::handle(&nm, cmd, &formatter, cli.verbose).await?;
        }
        Commands::Info => {
            info::handle(&formatter, cli.verbose).await?;
        }
    }

    Ok(())
}

/// Prompt user for a password securely
pub fn prompt_password(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    
    let password = rpasswrd::read_password()?;
    Ok(password)
}
