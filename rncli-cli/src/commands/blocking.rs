//! Website blocking command handler

use rncli_lib::blocking::BlockingManager;
use crate::formatter::OutputFormatter;
use crate::main::BlockCmd;

pub async fn handle(
    _nm: &rncli_lib::NetworkManager,
    cmd: BlockCmd,
    formatter: &OutputFormatter,
    _verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let bm = BlockingManager::new(false);
    
    match cmd {
        BlockCmd::Block { target } => {
            formatter.status("Block Website", &format!("Blocking '{}'", target));
            bm.block(&target).await?;
            formatter.success(&format!("Blocked '{}'", target));
        }
        BlockCmd::Unblock { target } => {
            formatter.status("Unblock Website", &format!("Unblocking '{}'", target));
            bm.unblock(&target).await?;
            formatter.success(&format!("Unblocked '{}'", target));
        }
        BlockCmd::List => {
            formatter.status("Blocked Websites", "Listing blocked domains...");
            let blocked = bm.list_blocked().await?;
            
            if blocked.is_empty() {
                formatter.warning("No blocked websites");
            } else {
                formatter.list("Blocked Domains", blocked.iter().map(|s| s.as_str()).collect());
            }
        }
    }
    
    Ok(())
}
