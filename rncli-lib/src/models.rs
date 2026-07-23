//! Data models for network connections and devices

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported connection types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConnectionType {
    Ethernet,
    WiFi,
    VPN,
    Bridge,
    Bond,
    VLAN,
    Tunnel,
    Tethering,
    Unknown,
}

impl ConnectionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ethernet => "ethernet",
            Self::WiFi => "wifi",
            Self::VPN => "vpn",
            Self::Bridge => "bridge",
            Self::Bond => "bond",
            Self::VLAN => "vlan",
            Self::Tunnel => "tunnel",
            Self::Tethering => "tethering",
            Self::Unknown => "unknown",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "ethernet" | "802-3-ethernet" => Self::Ethernet,
            "wifi" | "802-11-wireless" => Self::WiFi,
            "vpn" => Self::VPN,
            "bridge" => Self::Bridge,
            "bond" => Self::Bond,
            "vlan" => Self::VLAN,
            "tunnel" | "ip-tunnel" => Self::Tunnel,
            "tethering" => Self::Tethering,
            _ => Self::Unknown,
        }
    }
}

/// Supported device types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceType {
    Ethernet,
    WiFi,
    Bluetooth,
    Cellular,
    Bridge,
    Bond,
    VLAN,
    Tunnel,
    Unknown,
}

impl DeviceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ethernet => "ethernet",
            Self::WiFi => "wifi",
            Self::Bluetooth => "bluetooth",
            Self::Cellular => "cellular",
            Self::Bridge => "bridge",
            Self::Bond => "bond",
            Self::VLAN => "vlan",
            Self::Tunnel => "tunnel",
            Self::Unknown => "unknown",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "ethernet" => Self::Ethernet,
            "wifi" | "wireless" => Self::WiFi,
            "bluetooth" => Self::Bluetooth,
            "cellular" | "gsm" | "cdma" => Self::Cellular,
            "bridge" => Self::Bridge,
            "bond" => Self::Bond,
            "vlan" => Self::VLAN,
            "tunnel" => Self::Tunnel,
            _ => Self::Unknown,
        }
    }
}

/// Connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionState {
    Active,
    Inactive,
    Activating,
    Deactivating,
    Unknown,
}

impl ConnectionState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Activating => "activating",
            Self::Deactivating => "deactivating",
            Self::Unknown => "unknown",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "active" => Self::Active,
            "inactive" => Self::Inactive,
            "activating" => Self::Activating,
            "deactivating" => Self::Deactivating,
            _ => Self::Unknown,
        }
    }

    pub fn is_active(&self) -> bool {
        *self == Self::Active
    }
}

/// Network connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub id: String,
    pub uuid: String,
    pub connection_type: ConnectionType,
    pub state: ConnectionState,
    pub device: Option<String>,
    pub interface_name: Option<String>,
    pub autoconnect: bool,
    pub read_only: bool,
    pub created: i64,
    pub modified: i64,
    pub settings: HashMap<String, String>,
}

impl Connection {
    pub fn new(id: String, uuid: String, connection_type: ConnectionType) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        Self {
            id,
            uuid,
            connection_type,
            state: ConnectionState::Inactive,
            device: None,
            interface_name: None,
            autoconnect: false,
            read_only: false,
            created: now,
            modified: now,
            settings: HashMap::new(),
        }
    }

    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }
}

/// Network device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub interface: String,
    pub device_type: DeviceType,
    pub state: ConnectionState,
    pub ip_address: Option<String>,
    pub ipv6_address: Option<String>,
    pub mac_address: Option<String>,
    pub mtu: Option<u32>,
    pub carrier: bool,
    pub speed: Option<u32>, // Mbps
    pub active_connection: Option<String>,
    pub available_connections: Vec<String>,
}

impl Device {
    pub fn new(interface: String, device_type: DeviceType) -> Self {
        Self {
            interface,
            device_type,
            state: ConnectionState::Inactive,
            ip_address: None,
            ipv6_address: None,
            mac_address: None,
            mtu: None,
            carrier: false,
            speed: None,
            active_connection: None,
            available_connections: Vec::new(),
        }
    }

    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }

    pub fn is_connected(&self) -> bool {
        self.carrier && self.state.is_active()
    }
}

/// WiFi access point information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WiFiNetwork {
    pub ssid: String,
    pub bssid: String,
    pub frequency: u32,
    pub signal_strength: u32, // 0-100
    pub security: Vec<String>,
    pub active: bool,
    pub in_use: bool,
}

impl WiFiNetwork {
    pub fn signal_quality(&self) -> String {
        match self.signal_strength {
            80..=100 => "Excellent".to_string(),
            60..=79 => "Good".to_string(),
            40..=59 => "Fair".to_string(),
            20..=39 => "Weak".to_string(),
            _ => "Very Weak".to_string(),
        }
    }
}

/// System networking status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkingStatus {
    pub networking_enabled: bool,
    pub wifi_enabled: bool,
    pub wireless_hardware_enabled: bool,
    pub cellular_enabled: bool,
}

/// Event emitted by NetworkManager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEvent {
    pub event_type: EventType,
    pub timestamp: i64,
    pub details: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EventType {
    ConnectionActivated,
    ConnectionDeactivated,
    ConnectionAdded,
    ConnectionRemoved,
    DeviceAdded,
    DeviceRemoved,
    DeviceStateChanged,
    NetworkingEnabled,
    NetworkingDisabled,
    WiFiEnabled,
    WiFiDisabled,
}
