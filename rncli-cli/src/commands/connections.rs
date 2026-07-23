//! Connections command handler

use rncli_lib::NetworkManager;
use crate::formatter::OutputFormatter;
use crate::main::ConnectionsCmd;

pub async fn handle(
    nm: &NetworkManager,
    cmd: ConnectionsCmd,
    formatter: &OutputFormatter,
    _verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        ConnectionsCmd::List => {
            formatter.status("Connections", "Listing all connections...");
            let connections = nm.connections().list_all().await?;
            
            if connections.is_empty() {
                formatter.warning("No connections found");
            } else {
                let headers = vec!["ID", "Type", "State", "Device"];
                let rows: Vec<Vec<String>> = connections.iter().map(|conn| {
                    vec![
                        conn.id.clone(),
                        conn.connection_type.as_str().to_string(),
                        conn.state.as_str().to_string(),
                        conn.device.clone().unwrap_or_default(),
                    ]
                }).collect();
                formatter.table(headers, rows);
            }
            formatter.success("Connections listed");
        }
        ConnectionsCmd::Active => {
            formatter.status("Active Connections", "Listing active connections...");
            let connections = nm.connections().list_active().await?;
            
            if connections.is_empty() {
                formatter.warning("No active connections");
            } else {
                let headers = vec!["ID", "Type", "Device", "IP Address"];
                let rows: Vec<Vec<String>> = connections.iter().map(|conn| {
                    vec![
                        conn.id.clone(),
                        conn.connection_type.as_str().to_string(),
                        conn.device.clone().unwrap_or_default(),
                        conn.interface_name.clone().unwrap_or_default(),
                    ]
                }).collect();
                formatter.table(headers, rows);
            }
        }
        ConnectionsCmd::Activate { connection, device } => {
            formatter.status("Activate Connection", &format!("Activating '{}'", connection));
            nm.connections().activate(&connection, device.as_deref()).await?;
            formatter.success(&format!("Connection '{}' activated", connection));
        }
        ConnectionsCmd::Deactivate { connection } => {
            formatter.status("Deactivate Connection", &format!("Deactivating '{}'", connection));
            nm.connections().deactivate(&connection).await?;
            formatter.success(&format!("Connection '{}' deactivated", connection));
        }
        ConnectionsCmd::Show { connection } => {
            formatter.status("Connection Details", &format!("Showing '{}'", connection));
            let conn = nm.connections().get(&connection).await?;
            
            let mut data = std::collections::HashMap::new();
            data.insert("ID".to_string(), conn.id.clone());
            data.insert("UUID".to_string(), conn.uuid.clone());
            data.insert("Type".to_string(), conn.connection_type.as_str().to_string());
            data.insert("State".to_string(), conn.state.as_str().to_string());
            data.insert("Device".to_string(), conn.device.clone().unwrap_or_default());
            data.insert("Autoconnect".to_string(), conn.autoconnect.to_string());
            
            formatter.key_value(&data);
        }
        ConnectionsCmd::Delete { connection } => {
            formatter.status("Delete Connection", &format!("Deleting '{}'", connection));
            nm.connections().delete(&connection).await?;
            formatter.success(&format!("Connection '{}' deleted", connection));
        }
        ConnectionsCmd::Edit { connection } => {
            formatter.warning("Edit functionality requires interactive editor (not yet implemented)");
        }
    }
    
    Ok(())
}
