# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-07-23

### Added
- Initial release
- Connection management (list, activate, deactivate, delete)
- Device management (list, show, reapply, disconnect)
- WiFi management (scan, connect, disconnect, forget)
- Website blocking via /etc/hosts
- Multiple output formats (text, JSON, CSV)
- Colored terminal output
- Network status monitoring
- Automatic sudo detection
- Secure password input handling
- Comprehensive error handling
- Full API documentation
- CLI command-line interface

### Security
- No hardcoded secrets
- Input validation for all user inputs
- Atomic file operations for blocking
- Safe command execution
- Privilege escalation protection
