//! Device management

use crate::error::{Error, Result};
use crate::models::{Device, DeviceType};

/// Device manager
pub struct DeviceManager {
    use_sudo: bool,
}

impl DeviceManager {
    pub fn new(use_sudo: bool) -> Self {
        Self { use_sudo }
    }

    /// List all network devices
    pub async fn list_all(&self) -> Result<Vec<Device>> {
        // TODO: Implement D-Bus or nmcli parsing
        Ok(Vec::new())
    }

    /// Get a specific device by interface name
    pub async fn get(&self, interface: &str) -> Result<Device> {
        // TODO: Implement D-Bus or nmcli parsing
        Err(Error::DeviceNotFound(interface.to_string()))
    }

    /// Get device status
    pub async fn get_status(&self, interface: &str) -> Result<Device> {
        self.get(interface).await
    }

    /// Reapply connection settings to a device
    pub async fn reapply(&self, interface: &str) -> Result<()> {
        let _device = self.get(interface).await?;
        // TODO: Implement D-Bus reapply
        Ok(())
    }

    /// Get available devices of a specific type
    pub async fn list_by_type(&self, device_type: DeviceType) -> Result<Vec<Device>> {
        let all = self.list_all().await?;
        Ok(all.into_iter()
            .filter(|d| d.device_type == device_type)
            .collect())
    }

    /// Disconnect a device from its active connection
    pub async fn disconnect(&self, interface: &str) -> Result<()> {
        let device = self.get(interface).await?;
        
        if !device.is_active() {
            return Err(Error::ConnectionInactive(device.interface.clone()));
        }

        // TODO: Implement D-Bus disconnect
        Ok(())
    }

    /// Check device carrier status
    pub async fn check_carrier(&self, interface: &str) -> Result<bool> {
        let device = self.get(interface).await?;
        Ok(device.carrier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_manager_creation() {
        let dm = DeviceManager::new(false);
        assert!(!dm.use_sudo);
    }
}
