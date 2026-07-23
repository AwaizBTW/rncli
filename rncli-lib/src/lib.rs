//! # rncli-lib
//!
//! A modern, idiomatic Rust library for NetworkManager interaction on Linux.
//! Provides a clean, high-level interface for network management operations.

pub mod error;
pub mod models;
pub mod network_manager;
pub mod connection;
pub mod device;
pub mod wifi;
pub mod blocking;

pub use error::{Error, Result};
pub use network_manager::NetworkManager;
pub use models::{Connection, Device, DeviceType, ConnectionType};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
