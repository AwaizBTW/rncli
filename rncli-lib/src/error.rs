//! Error handling for rncli library

use std::fmt;
use thiserror::Error;

/// Result type for rncli operations
pub type Result<T> = std::result::Result<T, Error>;

/// Comprehensive error type for NetworkManager operations
#[derive(Error, Debug)]
pub enum Error {
    #[error("D-Bus error: {0}")]
    DBusError(String),

    #[error("Connection not found: {0}")]
    ConnectionNotFound(String),

    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    #[error("Network disabled")]
    NetworkingDisabled,

    #[error("WiFi disabled")]
    WiFiDisabled,

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Connection already active: {0}")]
    ConnectionActive(String),

    #[error("Connection already inactive: {0}")]
    ConnectionInactive(String),

    #[error("WiFi scan failed: {0}")]
    WiFiScanFailed(String),

    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Blocking configuration failed: {0}")]
    BlockingConfigFailed(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Operation not supported: {0}")]
    NotSupported(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl Error {
    /// Create a user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            Error::ConnectionNotFound(name) => {
                format!("Connection '{}' not found. Use 'rncli connections' to list available connections.", name)
            }
            Error::DeviceNotFound(name) => {
                format!("Device '{}' not found. Use 'rncli devices' to list available devices.", name)
            }
            Error::NetworkingDisabled => {
                "Networking is currently disabled. Enable it with 'rncli networking enable'.".to_string()
            }
            Error::WiFiDisabled => {
                "WiFi is currently disabled. Enable it with 'rncli wifi enable'.".to_string()
            }
            Error::PermissionDenied(msg) => {
                format!("Permission denied: {}. You may need to run with 'sudo'.", msg)
            }
            Error::InvalidConfiguration(msg) => {
                format!("Invalid configuration: {}", msg)
            }
            Error::ConnectionActive(name) => {
                format!("Connection '{}' is already active.", name)
            }
            Error::ConnectionInactive(name) => {
                format!("Connection '{}' is not active.", name)
            }
            _ => self.to_string(),
        }
    }

    /// Check if this is a permission-related error
    pub fn is_permission_error(&self) -> bool {
        matches!(self, Error::PermissionDenied(_))
    }

    /// Check if running with sudo might help
    pub fn needs_elevation(&self) -> bool {
        self.is_permission_error()
    }
}
