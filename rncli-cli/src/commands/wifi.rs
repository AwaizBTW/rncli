//! WiFi command handler

use rncli_lib::NetworkManager;
use crate::formatter::OutputFormatter;
use crate::main::{WiFiCmd, WiFiRadioCmd};

pub async fn handle(
    nm: &NetworkManager,
    cmd: WiFiCmd,
    formatter: &OutputFormatter,
    _verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        WiFiCmd::List => {
            formatter.status("WiFi Networks", "Listing available WiFi networks...");
            let networks = nm.wifi().list().await?;
            
            if networks.is_empty() {
                formatter.warning("No WiFi networks found");
            } else {
                let headers = vec!["SSID", "Signal", "Security", "In Use"];
                let rows: Vec<Vec<String>> = networks.iter().map(|net| {
                    vec![
                        net.ssid.clone(),
                        net.signal_quality(),
                        net.security.join(","),
                        if net.in_use { "Yes" } else { "No" }.to_string(),
                    ]
                }).collect();
                formatter.table(headers, rows);
            }
        }
        WiFiCmd::Scan { interface } => {
            formatter.status("WiFi Scan", "Scanning for WiFi networks...");
            let networks = nm.wifi().scan(interface.as_deref()).await?;
            
            if networks.is_empty() {
                formatter.warning("No WiFi networks found");
            } else {
                let headers = vec!["SSID", "BSSID", "Frequency", "Signal", "Security"];
                let rows: Vec<Vec<String>> = networks.iter().map(|net| {
                    vec![
                        net.ssid.clone(),
                        net.bssid.clone(),
                        format!("{}MHz", net.frequency),
                        format!("{}%", net.signal_strength),
                        net.security.join(","),
                    ]
                }).collect();
                formatter.table(headers, rows);
            }
            formatter.success("WiFi scan completed");
        }
        WiFiCmd::Connect { ssid, password } => {
            formatter.status("WiFi Connect", &format!("Connecting to '{}'", ssid));
            
            let pwd = match password {
                Some(p) => p,
                None => {
                    use std::io::{self, Write};
                    print!("Enter WiFi password: ");
                    io::stdout().flush()?;
                    rpasswrd::read_password()?
                }
            };
            
            nm.wifi().connect(&ssid, Some(&pwd)).await?;
            formatter.success(&format!("Connected to '{}'", ssid));
        }
        WiFiCmd::Disconnect { interface } => {
            formatter.status("WiFi Disconnect", "Disconnecting from WiFi...");
            nm.wifi().disconnect(interface.as_deref()).await?;
            formatter.success("WiFi disconnected");
        }
        WiFiCmd::Forget { ssid } => {
            formatter.status("Forget WiFi", &format!("Forgetting '{}'", ssid));
            nm.wifi().forget(&ssid).await?;
            formatter.success(&format!("Forgot '{}'", ssid));
        }
        WiFiCmd::Radio(radio_cmd) => {
            match radio_cmd {
                WiFiRadioCmd::On => {
                    formatter.status("WiFi Radio", "Enabling WiFi...");
                    nm.set_wifi(true).await?;
                    formatter.success("WiFi enabled");
                }
                WiFiRadioCmd::Off => {
                    formatter.status("WiFi Radio", "Disabling WiFi...");
                    nm.set_wifi(false).await?;
                    formatter.success("WiFi disabled");
                }
            }
        }
    }
    
    Ok(())
}
