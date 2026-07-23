//! Networking command handler

use rncli_lib::NetworkManager;
use crate::formatter::OutputFormatter;
use crate::main::NetworkingCmd;

pub async fn handle(
    nm: &NetworkManager,
    _cmd: NetworkingCmd,
    formatter: &OutputFormatter,
    _verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match _cmd {
        NetworkingCmd::On => {
            formatter.status("Networking", "Enabling networking...");
            nm.set_networking(true).await?;
            formatter.success("Networking enabled");
        }
        NetworkingCmd::Off => {
            formatter.status("Networking", "Disabling networking...");
            nm.set_networking(false).await?;
            formatter.success("Networking disabled");
        }
    }
    
    Ok(())
}
