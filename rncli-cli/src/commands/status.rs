//! Status command handler

use rncli_lib::NetworkManager;
use crate::formatter::OutputFormatter;
use std::collections::HashMap;

pub async fn handle(
    nm: &NetworkManager,
    formatter: &OutputFormatter,
    _verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    formatter.status("Getting network status", "");
    
    match nm.status().await {
        Ok(status) => {
            let mut data = HashMap::new();
            data.insert("Networking".to_string(), 
                if status.networking_enabled { "Enabled" } else { "Disabled" }.to_string());
            data.insert("WiFi".to_string(), 
                if status.wifi_enabled { "Enabled" } else { "Disabled" }.to_string());
            data.insert("Wireless Hardware".to_string(), 
                if status.wireless_hardware_enabled { "Enabled" } else { "Disabled" }.to_string());
            data.insert("Cellular".to_string(), 
                if status.cellular_enabled { "Enabled" } else { "Disabled" }.to_string());
            
            formatter.key_value(&data);
            formatter.success("Status retrieved successfully");
        }
        Err(e) => {
            formatter.error(&format!("Failed to get status: {}", e));
            return Err(e.into());
        }
    }
    
    Ok(())
}
