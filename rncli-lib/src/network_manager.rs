//! Main NetworkManager interface

use crate::error::{Error, Result};
use crate::models::{ConnectionState, ConnectionType, DeviceType, NetworkingStatus};
use crate::connection::ConnectionManager;
use crate::device::DeviceManager;
use crate::wifi::WiFiManager;
use std::process::Command;

/// Main NetworkManager interface
pub struct NetworkManager {
    use_sudo: bool,
    connection_manager: ConnectionManager,
    device_manager: DeviceManager,
    wifi_manager: WiFiManager,
}

impl NetworkManager {
    /// Create a new NetworkManager instance
    pub fn new(use_sudo: bool) -> Self {
        Self {
            use_sudo,
            connection_manager: ConnectionManager::new(use_sudo),
            device_manager: DeviceManager::new(use_sudo),
            wifi_manager: WiFiManager::new(use_sudo),
        }
    }

    /// Create with automatic sudo detection (sudo if not root and available)
    pub fn with_auto_sudo() -> Self {
        let use_sudo = !is_root() && can_sudo();
        Self::new(use_sudo)
    }

    /// Get the current status of networking
    pub async fn status(&self) -> Result<NetworkingStatus> {
        let output = self.run_nmcli(&["general", "status"])
            .map_err(|e| Error::DBusError(format!("Failed to get status: {}", e)))?;

        // Parse nmcli output
        let lines: Vec<&str> = output.lines().collect();
        if lines.len() < 2 {
            return Err(Error::DBusError("Invalid status output".to_string()));
        }

        // Example output:
        // STATE    CONNECTIVITY  WIFI-HW  WIFI     WWAN-HW  WWAN
        // connected  full          enabled  enabled  enabled  enabled
        
        let networking_enabled = lines[1].contains("connected") || lines[1].contains("asleep");
        let wifi_enabled = lines[1].contains("enabled");

        Ok(NetworkingStatus {
            networking_enabled,
            wifi_enabled,
            wireless_hardware_enabled: true,
            cellular_enabled: true,
        })
    }

    /// Enable or disable networking
    pub async fn set_networking(&self, enabled: bool) -> Result<()> {
        let state = if enabled { "on" } else { "off" };
        self.run_nmcli(&["networking", state])
            .map_err(|e| Error::DBusError(format!("Failed to set networking: {}", e)))?;
        Ok(())
    }

    /// Enable or disable WiFi
    pub async fn set_wifi(&self, enabled: bool) -> Result<()> {
        let state = if enabled { "on" } else { "off" };
        self.run_nmcli(&["radio", "wifi", state])
            .map_err(|e| Error::DBusError(format!("Failed to set wifi: {}", e)))?;
        Ok(())
    }

    /// Get connection manager
    pub fn connections(&self) -> &ConnectionManager {
        &self.connection_manager
    }

    /// Get mutable connection manager
    pub fn connections_mut(&mut self) -> &mut ConnectionManager {
        &mut self.connection_manager
    }

    /// Get device manager
    pub fn devices(&self) -> &DeviceManager {
        &self.device_manager
    }

    /// Get mutable device manager
    pub fn devices_mut(&mut self) -> &mut DeviceManager {
        &mut self.device_manager
    }

    /// Get WiFi manager
    pub fn wifi(&self) -> &WiFiManager {
        &self.wifi_manager
    }

    /// Get mutable WiFi manager
    pub fn wifi_mut(&mut self) -> &mut WiFiManager {
        &mut self.wifi_manager
    }

    /// Run an nmcli command
    pub fn run_nmcli(&self, args: &[&str]) -> std::io::Result<String> {
        let mut cmd = if self.use_sudo {
            let mut c = Command::new("sudo");
            c.arg("nmcli");
            c
        } else {
            Command::new("nmcli")
        };

        // Add arguments and set JSON output for parseable results
        cmd.args(args);

        let output = cmd.output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("nmcli failed: {}", stderr),
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Run an nmcli command with JSON output
    pub fn run_nmcli_json(&self, args: &[&str]) -> std::io::Result<String> {
        let mut new_args = args.to_vec();
        new_args.push("-t");
        new_args.push("-m");
        self.run_nmcli(&new_args)
    }
}

/// Check if running as root
fn is_root() -> bool {
    std::env::var("EUID")
        .ok()
        .and_then(|id| id.parse::<u32>().ok())
        .map(|id| id == 0)
        .unwrap_or_else(|| {
            // Fallback: check if UID is 0
            unsafe { libc::getuid() == 0 }
        })
}

/// Check if sudo is available and user can run it without password
fn can_sudo() -> bool {
    Command::new("sudo")
        .arg("-n")
        .arg("true")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_manager_creation() {
        let nm = NetworkManager::new(false);
        assert!(!nm.use_sudo);
    }
}
