# rncli - The Rust-Powered Network Manager CLI

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)

A modern, fast, and user-friendly command-line interface for managing Linux network connections. Built with Rust for reliability and performance.

## Features

- ✅ **Connection Management**: List, activate, deactivate, and modify network connections
- ✅ **Device Management**: View device status, reapply settings, and manage interfaces
- ✅ **WiFi Control**: Scan, connect, disconnect, and manage WiFi networks
- ✅ **Network Status**: Real-time network status monitoring
- ✅ **Website Blocking**: Simple domain blocking via `/etc/hosts`
- ✅ **Multiple Output Formats**: Text, JSON, and CSV output
- ✅ **Cross-Platform**: Linux support with extensible architecture

## Installation

### From Source

```bash
git clone https://github.com/AwaizBTW/rncli.git
cd rncli
cargo build --release
sudo cp target/release/rncli /usr/local/bin/
```

## Quick Start

### View Network Status

```bash
rncli status
```

### List Connections

```bash
rncli connections list
```

### Activate a Connection

```bash
rncli connections activate "Wired connection 1"
```

### WiFi Management

```bash
# Scan for networks
rncli wifi scan

# Connect to WiFi
rncli wifi connect "MyNetwork" --password "mypassword"

# Enable/disable WiFi
rncli wifi radio on
rncli wifi radio off
```

### Block/Unblock Websites

```bash
# Block a domain
sudo rncli block block example.com

# Unblock a domain
sudo rncli block unblock example.com

# List blocked domains
sudo rncli block list
```

## Output Formats

All commands support multiple output formats:

```bash
# Text format (default)
rncli connections list

# JSON format
rncli connections list --output json

# CSV format
rncli connections list --output csv
```

## Global Options

```bash
-v, --verbose          Enable verbose output
-o, --output <FORMAT>  Output format: text, json, csv (default: text)
--no-color            Disable colored output
--sudo                Force use of sudo
```

## Project Structure

```
rncli/
├── rncli-lib/          # Core library
│   ├── src/
│   │   ├── lib.rs
│   │   ├── error.rs
│   │   ├── models.rs
│   │   ├── network_manager.rs
│   │   ├── connection.rs
│   │   ├── device.rs
│   │   ├── wifi.rs
│   │   └── blocking.rs
│   └── Cargo.toml
├── rncli-cli/          # CLI interface
│   ├── src/
│   │   ├── main.rs
│   │   ├── formatter.rs
│   │   ├── cli.rs
│   │   └── commands/
│   │       ├── mod.rs
│   │       ├── status.rs
│   │       ├── connections.rs
│   │       ├── devices.rs
│   │       ├── wifi.rs
│   │       ├── networking.rs
│   │       ├── blocking.rs
│   │       └── info.rs
│   └── Cargo.toml
├── Cargo.toml          # Workspace configuration
├── README.md           # This file
└── LICENSE             # Apache 2.0 License
```

## Development

### Build

```bash
cargo build
```

### Test

```bash
cargo test --all
```

### Lint

```bash
cargo clippy --all
```

### Format

```bash
cargo fmt --all
```

## Security

- **No Hardcoded Secrets**: All sensitive data is securely handled
- **Input Validation**: Comprehensive validation of all user inputs
- **Safe File Operations**: Atomic writes prevent data corruption
- **Privilege Management**: Automatic sudo detection without escalation vulnerabilities
- **Secure Password Handling**: Uses `rpasswrd` for secure password input

## Requirements

- Linux system with NetworkManager
- Rust 1.70+
- Sudo access for network management operations

## Documentation

Full API documentation is available via:

```bash
cargo doc --all --open
```

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Author

Awaiz Azam - [@AwaizBTW](https://github.com/AwaizBTW)

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- CLI parsing with [Clap](https://github.com/clap-rs/clap)
- Async runtime by [Tokio](https://tokio.rs/)
- Serialization with [Serde](https://serde.rs/)

## Support

For issues, questions, or suggestions, please open an [issue on GitHub](https://github.com/AwaizBTW/rncli/issues).
