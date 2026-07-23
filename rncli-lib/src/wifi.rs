//! WiFi management

use crate::error::{Error, Result};
use crate::models::WiFiNetwork;

/// WiFi manager
pub struct WiFiManager {
    use_sudo: bool,
}

impl WiFiManager {
    pub fn new(use_sudo: bool) -> Self {
        Self { use_sudo }
    }

    /// Scan available WiFi networks
    pub async fn scan(&self, interface: Option<&str>) -> Result<Vec<WiFiNetwork>> {
        // TODO: Implement D-Bus or nmcli WiFi scan
        Err(Error::WiFiScanFailed("Scan not yet implemented".to_string()))
    }

    /// Connect to a WiFi network
    pub async fn connect(&self, ssid: &str, password: Option<&str>) -> Result<()> {
        if ssid.is_empty() {
            return Err(Error::InvalidInput("SSID cannot be empty".to_string()));
        }

        // TODO: Implement D-Bus WiFi connection
        Ok(())
    }

    /// Disconnect from WiFi
    pub async fn disconnect(&self, interface: Option<&str>) -> Result<()> {
        // TODO: Implement D-Bus WiFi disconnection
        Ok(())
    }

    /// List available WiFi networks (cached from last scan)
    pub async fn list(&self) -> Result<Vec<WiFiNetwork>> {
        // TODO: Return cached results from last scan
        Ok(Vec::new())
    }

    /// Forget a WiFi network
    pub async fn forget(&self, ssid: &str) -> Result<()> {
        if ssid.is_empty() {
            return Err(Error::InvalidInput("SSID cannot be empty".to_string()));
        }

        // TODO: Implement D-Bus WiFi forget
        Ok(())
    }

    /// Get WiFi capability information
    pub async fn get_capabilities(&self, interface: Option<&str>) -> Result<String> {
        // TODO: Implement capability detection
        Ok("802.11ac, 802.11n".to_string())
    }

    /// Set WiFi power state
    pub async fn set_power(&self, enabled: bool) -> Result<()> {
        // TODO: Implement D-Bus radio power control
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wifi_manager_creation() {
        let wm = WiFiManager::new(false);
        assert!(!wm.use_sudo);
    }

    #[test]
    fn test_invalid_ssid() {
        let wm = WiFiManager::new(false);
        let result = futures::executor::block_on(wm.connect("", None));
        assert!(result.is_err());
    }
}
