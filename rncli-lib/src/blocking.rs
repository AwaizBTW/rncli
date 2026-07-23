//! Website blocking functionality

use crate::error::{Error, Result};
use regex::Regex;
use std::fs;
use std::path::Path;
use once_cell::sync::Lazy;

// Compile regex at module load time (lazy)
static DOMAIN_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?$")
        .expect("Domain regex is valid")
});

/// Website blocking manager
pub struct BlockingManager {
    use_sudo: bool,
    hosts_file: String,
}

impl BlockingManager {
    pub fn new(use_sudo: bool) -> Self {
        Self {
            use_sudo,
            hosts_file: "/etc/hosts".to_string(),
        }
    }

    /// Create with custom hosts file (for testing)
    #[cfg(test)]
    pub fn with_hosts_file(use_sudo: bool, hosts_file: String) -> Self {
        Self { use_sudo, hosts_file }
    }

    /// Block a website by domain or URL
    pub async fn block(&self, domain_or_url: &str) -> Result<()> {
        let domain = extract_domain(domain_or_url)?;
        
        // Validate domain format
        if domain.is_empty() {
            return Err(Error::InvalidInput("Invalid domain or URL".to_string()));
        }

        // Add to hosts file
        self.add_to_hosts(&domain)?;
        Ok(())
    }

    /// Unblock a website
    pub async fn unblock(&self, domain_or_url: &str) -> Result<()> {
        let domain = extract_domain(domain_or_url)?;
        
        if domain.is_empty() {
            return Err(Error::InvalidInput("Invalid domain or URL".to_string()));
        }

        // Remove from hosts file
        self.remove_from_hosts(&domain)?;
        Ok(())
    }

    /// List currently blocked domains
    pub async fn list_blocked(&self) -> Result<Vec<String>> {
        let content = self.read_hosts_file()?;
        let mut blocked = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            // Lines added by rncli have special comment
            if !line.contains("rncli-blocked") {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[0] == "127.0.0.1" {
                blocked.push(parts[1].to_string());
            }
        }

        Ok(blocked)
    }

    /// Add domain to hosts file
    fn add_to_hosts(&self, domain: &str) -> Result<()> {
        let mut content = self.read_hosts_file()?;

        // Check if already blocked
        if content.contains(&format!("127.0.0.1 {}", domain)) {
            return Err(Error::BlockingConfigFailed(
                format!("Domain '{}' is already blocked", domain),
            ));
        }

        // Add block entry
        let entry = format!(
            "127.0.0.1 {} # rncli-blocked\n",
            domain
        );
        content.push_str(&entry);

        self.write_hosts_file(&content)?;
        Ok(())
    }

    /// Remove domain from hosts file
    fn remove_from_hosts(&self, domain: &str) -> Result<()> {
        let content = self.read_hosts_file()?;
        
        let lines: Vec<&str> = content
            .lines()
            .filter(|line| {
                // Only remove lines that match BOTH conditions:
                // 1. Contain the blocked domain marker from rncli
                // 2. Target the domain we want to unblock
                !(line.contains(&format!("127.0.0.1 {}", domain))
                    && line.contains("rncli-blocked"))
            })
            .collect();

        let new_content = lines.join("\n");
        if new_content == content {
            return Err(Error::BlockingConfigFailed(
                format!("Domain '{}' is not blocked", domain),
            ));
        }

        self.write_hosts_file(&new_content)?;
        Ok(())
    }

    /// Read hosts file
    fn read_hosts_file(&self) -> Result<String> {
        fs::read_to_string(&self.hosts_file)
            .map_err(|e| Error::BlockingConfigFailed(
                format!("Cannot read hosts file: {}", e),
            ))
    }

    /// Write hosts file with atomic operation
    fn write_hosts_file(&self, content: &str) -> Result<()> {
        use std::io::Write;
        use tempfile::NamedTempFile;

        // Write to temp file first, then rename (atomic operation)
        let temp_file = NamedTempFile::new()
            .map_err(|e| Error::BlockingConfigFailed(
                format!("Cannot create temporary file: {}", e),
            ))?;

        let mut file = temp_file;
        file.write_all(content.as_bytes())
            .map_err(|e| Error::BlockingConfigFailed(
                format!("Cannot write to temporary file: {}", e),
            ))?;

        file.persist(&self.hosts_file)
            .map_err(|e| Error::BlockingConfigFailed(
                format!("Cannot write hosts file. Requires elevated privileges. Error: {}", e),
            ))?;

        Ok(())
    }
}

/// Extract domain from URL or domain string with validation
fn extract_domain(domain_or_url: &str) -> Result<String> {
    // Validate input length
    if domain_or_url.is_empty() || domain_or_url.len() > 255 {
        return Err(Error::InvalidInput("Domain must be between 1 and 255 characters".to_string()));
    }

    // Remove protocol if present
    let domain = domain_or_url
        .trim()
        .trim_start_matches("http://")
        .trim_start_matches("https://")
        .trim_start_matches("ftp://");

    // Remove path and query string
    let domain = domain
        .split('/')
        .next()
        .unwrap_or("")
        .split('?')
        .next()
        .unwrap_or("");

    // Remove port if present
    let domain = domain.split(':').next().unwrap_or("");

    // Trim whitespace
    let domain = domain.trim();

    // Validate domain format using pre-compiled regex
    if DOMAIN_REGEX.is_match(&domain.to_lowercase()) {
        Ok(domain.to_string())
    } else {
        Err(Error::InvalidInput(format!(
            "'{}' is not a valid domain (must be valid DNS name)",
            domain
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_domain_from_url() {
        assert_eq!(
            extract_domain("https://example.com").unwrap(),
            "example.com"
        );
        assert_eq!(
            extract_domain("http://subdomain.example.com/path").unwrap(),
            "subdomain.example.com"
        );
        assert_eq!(
            extract_domain("example.com").unwrap(),
            "example.com"
        );
    }

    #[test]
    fn test_extract_domain_invalid() {
        assert!(extract_domain("").is_err());
        assert!(extract_domain("invalid..domain").is_err());
        assert!(extract_domain("192.168.1.1").is_err()); // IP addresses not allowed
    }

    #[test]
    fn test_extract_domain_with_whitespace() {
        assert_eq!(
            extract_domain("  example.com  ").unwrap(),
            "example.com"
        );
    }

    #[test]
    fn test_extract_domain_too_long() {
        let long_domain = "a".repeat(256);
        assert!(extract_domain(&long_domain).is_err());
    }

    #[test]
    fn test_blocking_manager_creation() {
        let bm = BlockingManager::new(false);
        assert!(!bm.use_sudo);
    }
}
