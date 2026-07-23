//! Devices command handler

use rncli_lib::NetworkManager;
use crate::formatter::OutputFormatter;
use crate::main::DevicesCmd;

pub async fn handle(
    nm: &NetworkManager,
    cmd: DevicesCmd,
    formatter: &OutputFormatter,
    _verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        DevicesCmd::List => {
            formatter.status("Devices", "Listing all devices...");
            let devices = nm.devices().list_all().await?;
            
            if devices.is_empty() {
                formatter.warning("No devices found");
            } else {
                let headers = vec!["Interface", "Type", "State", "IP Address", "Connected"];
                let rows: Vec<Vec<String>> = devices.iter().map(|dev| {
                    vec![
                        dev.interface.clone(),
                        dev.device_type.as_str().to_string(),
                        dev.state.as_str().to_string(),
                        dev.ip_address.clone().unwrap_or_else(|| "N/A".to_string()),
                        if dev.is_connected() { "Yes" } else { "No" }.to_string(),
                    ]
                }).collect();
                formatter.table(headers, rows);
            }
            formatter.success("Devices listed");
        }
        DevicesCmd::Show { interface } => {
            formatter.status("Device Details", &format!("Showing device '{}'", interface));
            let device = nm.devices().get(&interface).await?;
            
            let mut data = std::collections::HashMap::new();
            data.insert("Interface".to_string(), device.interface.clone());
            data.insert("Type".to_string(), device.device_type.as_str().to_string());
            data.insert("State".to_string(), device.state.as_str().to_string());
            data.insert("IP Address".to_string(), 
                device.ip_address.clone().unwrap_or_else(|| "N/A".to_string()));
            data.insert("IPv6 Address".to_string(), 
                device.ipv6_address.clone().unwrap_or_else(|| "N/A".to_string()));
            data.insert("MAC Address".to_string(), 
                device.mac_address.clone().unwrap_or_else(|| "N/A".to_string()));
            data.insert("MTU".to_string(), 
                device.mtu.map(|m| m.to_string()).unwrap_or_else(|| "N/A".to_string()));
            data.insert("Carrier".to_string(), device.carrier.to_string());
            data.insert("Speed".to_string(), 
                device.speed.map(|s| format!("{}Mbps", s)).unwrap_or_else(|| "N/A".to_string()));
            
            formatter.key_value(&data);
        }
        DevicesCmd::Reapply { interface } => {
            formatter.status("Reapply Device", &format!("Reapplying connection settings to '{}'", interface));
            nm.devices().reapply(&interface).await?;
            formatter.success(&format!("Connection settings reapplied to '{}'", interface));
        }
        DevicesCmd::Disconnect { interface } => {
            formatter.status("Disconnect Device", &format!("Disconnecting '{}'", interface));
            nm.devices().disconnect(&interface).await?;
            formatter.success(&format!("Device '{}' disconnected", interface));
        }
    }
    
    Ok(())
}
