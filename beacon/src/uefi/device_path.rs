use std::fmt;

/// Messaging Device Path subtypes
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum MessagingSubtype {
    Atapi = 0x01,
    Scsi = 0x02,
    FibreChannel = 0x03,
    Firewire = 0x04,
    Usb = 0x05,
    I2o = 0x06,
    InfiniBand = 0x09,
    Vendor = 0x0a,
    MacAddress = 0x0b,
    Ipv4 = 0x0c,
    Ipv6 = 0x0d,
    Uart = 0x0e,
    UsbClass = 0x0f,
    UsbWwid = 0x10,
    DeviceLogicalUnit = 0x11,
    Sata = 0x12,
    Iscsi = 0x13,
    Vlan = 0x14,
    FibreChannelEx = 0x15,
    SasEx = 0x16,
    NvmeNamespace = 0x17,
    Uri = 0x18,
    Ufs = 0x19,
    Sd = 0x1a,
    Bluetooth = 0x1b,
    WiFi = 0x1c,
    Emmc = 0x1d,
    BluetoothLe = 0x1e,
    Dns = 0x1f,
    NvdimmNamespace = 0x20,
    RestService = 0x21,
    NvmeOverFabric = 0x22,
}

impl From<u8> for MessagingSubtype {
    fn from(value: u8) -> Self {
        match value {
            0x01 => MessagingSubtype::Atapi,
            0x02 => MessagingSubtype::Scsi,
            0x03 => MessagingSubtype::FibreChannel,
            0x04 => MessagingSubtype::Firewire,
            0x05 => MessagingSubtype::Usb,
            0x06 => MessagingSubtype::I2o,
            0x09 => MessagingSubtype::InfiniBand,
            0x0a => MessagingSubtype::Vendor,
            0x0b => MessagingSubtype::MacAddress,
            0x0c => MessagingSubtype::Ipv4,
            0x0d => MessagingSubtype::Ipv6,
            0x0e => MessagingSubtype::Uart,
            0x0f => MessagingSubtype::UsbClass,
            0x10 => MessagingSubtype::UsbWwid,
            0x11 => MessagingSubtype::DeviceLogicalUnit,
            0x12 => MessagingSubtype::Sata,
            0x13 => MessagingSubtype::Iscsi,
            0x14 => MessagingSubtype::Vlan,
            0x15 => MessagingSubtype::FibreChannelEx,
            0x16 => MessagingSubtype::SasEx,
            0x17 => MessagingSubtype::NvmeNamespace,
            0x18 => MessagingSubtype::Uri,
            0x19 => MessagingSubtype::Ufs,
            0x1a => MessagingSubtype::Sd,
            0x1b => MessagingSubtype::Bluetooth,
            0x1c => MessagingSubtype::WiFi,
            0x1d => MessagingSubtype::Emmc,
            0x1e => MessagingSubtype::BluetoothLe,
            0x1f => MessagingSubtype::Dns,
            0x20 => MessagingSubtype::NvdimmNamespace,
            0x21 => MessagingSubtype::RestService,
            0x22 => MessagingSubtype::NvmeOverFabric,
            _ => MessagingSubtype::Uri, // Default to Uri for unknown subtypes
        }
    }
}

/// Device Path Node representing a single component in a device path
#[derive(Debug, Clone)]
pub struct DevicePathNode {
    pub device_type: DevicePathType,
    pub subtype: u8,
    pub length: u16,
    pub data: Vec<u8>,
}

/// Parsed messaging device path data
#[derive(Debug, Clone)]
pub enum MessagingDevicePathData {
    MacAddress {
        address: [u8; 6],
        if_type: u8,
    },
    Ipv4 {
        local_ip: [u8; 4],
        remote_ip: [u8; 4],
        local_port: u16,
        remote_port: u16,
        protocol: u16,
        static_ip: bool,
        gateway_ip: [u8; 4],
        subnet_mask: [u8; 4],
    },
    Ipv6 {
        local_ip: [u8; 16],
        remote_ip: [u8; 16],
        local_port: u16,
        remote_port: u16,
        protocol: u16,
        ip_address_origin: u8,
        prefix_length: u8,
        gateway_ip: [u8; 16],
    },
    Uri {
        uri: String,
    },
    Raw {
        subtype: u8,
        data: Vec<u8>,
    },
}

/// Complete Device Path containing multiple nodes
#[derive(Debug, Clone)]
pub struct DevicePath {
    pub nodes: Vec<DevicePathNode>,
}

#[derive(Debug)]
pub enum DevicePathError {
    InvalidLength,
    InvalidFormat(String),
    UnexpectedEndOfData,
}

impl fmt::Display for DevicePathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DevicePathError::InvalidLength => write!(f, "Invalid device path length"),
            DevicePathError::InvalidFormat(msg) => write!(f, "Invalid device path format: {}", msg),
            DevicePathError::UnexpectedEndOfData => write!(f, "Unexpected end of device path data"),
        }
    }
}

impl std::error::Error for DevicePathError {}

impl DevicePath {
    /// Parse a device path from raw binary data
    pub fn parse(data: &[u8]) -> Result<Self, DevicePathError> {
        let mut nodes = Vec::new();
        let mut offset = 0;

        while offset < data.len() {
            // Each device path node has a 4-byte header
            if offset + 4 > data.len() {
                break;
            }

            let device_type = DevicePathType::from(data[offset]);
            let subtype = data[offset + 1];
            let length = u16::from_le_bytes([data[offset + 2], data[offset + 3]]);

            // Validate length
            if length < 4 {
                return Err(DevicePathError::InvalidLength);
            }

            if offset + length as usize > data.len() {
                return Err(DevicePathError::UnexpectedEndOfData);
            }

            // Extract node data (excluding the 4-byte header)
            let node_data = if length > 4 {
                data[offset + 4..offset + length as usize].to_vec()
            } else {
                Vec::new()
            };

            let node = DevicePathNode {
                device_type,
                subtype,
                length,
                data: node_data,
            };

            nodes.push(node);

            // Check for end node
            if matches!(device_type, DevicePathType::End) {
                break;
            }

            offset += length as usize;
        }

        Ok(DevicePath { nodes })
    }

    /// Find all messaging device path nodes
    pub fn messaging_nodes(&self) -> Vec<&DevicePathNode> {
        self.nodes
            .iter()
            .filter(|node| matches!(node.device_type, DevicePathType::Messaging))
            .collect()
    }

    /// Check if this device path contains a MAC address node
    pub fn has_mac_address(&self) -> bool {
        self.messaging_nodes()
            .iter()
            .any(|node| node.subtype == MessagingSubtype::MacAddress as u8)
    }

    /// Extract URI from the device path, if present
    pub fn extract_uri(&self) -> Option<String> {
        for node in self.messaging_nodes() {
            if node.subtype == MessagingSubtype::Uri as u8 && !node.data.is_empty() {
                let uri = String::from_utf8_lossy(&node.data)
                    .trim_end_matches('\0')
                    .to_string();
                if !uri.is_empty() {
                    return Some(uri);
                }
            }
        }
        None
    }

    /// Parse messaging device path data for a specific node
    pub fn parse_messaging_data(&self, node: &DevicePathNode) -> Option<MessagingDevicePathData> {
        if !matches!(node.device_type, DevicePathType::Messaging) {
            return None;
        }

        let subtype = MessagingSubtype::from(node.subtype);

        match subtype {
            MessagingSubtype::MacAddress => {
                // UEFI spec: MAC address is 32 bytes padded with 0s, followed by IfType
                if node.data.len() >= 33 {
                    let mut address = [0u8; 6];
                    address.copy_from_slice(&node.data[0..6]);
                    let if_type = node.data[32]; // IfType is at byte offset 32 (after 32-byte MAC field)

                    Some(MessagingDevicePathData::MacAddress { address, if_type })
                } else {
                    None
                }
            }
            MessagingSubtype::Ipv4 => {
                // UEFI spec: Previous versions were 19 bytes, current spec requires 27 bytes minimum
                if node.data.len() >= 27 {
                    let mut local_ip = [0u8; 4];
                    let mut remote_ip = [0u8; 4];
                    let mut gateway_ip = [0u8; 4];
                    let mut subnet_mask = [0u8; 4];

                    local_ip.copy_from_slice(&node.data[0..4]);
                    remote_ip.copy_from_slice(&node.data[4..8]);

                    let local_port = u16::from_le_bytes([node.data[8], node.data[9]]);
                    let remote_port = u16::from_le_bytes([node.data[10], node.data[11]]);
                    let protocol = u16::from_le_bytes([node.data[12], node.data[13]]);
                    let static_ip = node.data[14] != 0;

                    gateway_ip.copy_from_slice(&node.data[15..19]);
                    subnet_mask.copy_from_slice(&node.data[19..23]);

                    Some(MessagingDevicePathData::Ipv4 {
                        local_ip,
                        remote_ip,
                        local_port,
                        remote_port,
                        protocol,
                        static_ip,
                        gateway_ip,
                        subnet_mask,
                    })
                } else if node.data.len() >= 19 {
                    // Legacy 19-byte format support
                    let mut local_ip = [0u8; 4];
                    let mut remote_ip = [0u8; 4];

                    local_ip.copy_from_slice(&node.data[0..4]);
                    remote_ip.copy_from_slice(&node.data[4..8]);

                    let local_port = u16::from_le_bytes([node.data[8], node.data[9]]);
                    let remote_port = u16::from_le_bytes([node.data[10], node.data[11]]);
                    let protocol = u16::from_le_bytes([node.data[12], node.data[13]]);
                    let static_ip = node.data[14] != 0;

                    Some(MessagingDevicePathData::Ipv4 {
                        local_ip,
                        remote_ip,
                        local_port,
                        remote_port,
                        protocol,
                        static_ip,
                        gateway_ip: [0; 4],  // Default for legacy format
                        subnet_mask: [0; 4], // Default for legacy format
                    })
                } else {
                    None
                }
            }
            MessagingSubtype::Uri => {
                let uri = String::from_utf8_lossy(&node.data)
                    .trim_end_matches('\0')
                    .to_string();
                Some(MessagingDevicePathData::Uri { uri })
            }
            _ => Some(MessagingDevicePathData::Raw {
                subtype: node.subtype,
                data: node.data.clone(),
            }),
        }
    }

    /// Get all parsed messaging data from this device path
    pub fn messaging_data(&self) -> Vec<MessagingDevicePathData> {
        self.messaging_nodes()
            .iter()
            .filter_map(|node| self.parse_messaging_data(node))
            .collect()
    }
}

impl fmt::Display for DevicePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        for node in &self.nodes {
            match node.device_type {
                DevicePathType::Messaging => {
                    if let Some(data) = self.parse_messaging_data(node) {
                        match data {
                            MessagingDevicePathData::MacAddress { address, .. } => {
                                parts.push(format!(
                                    "MAC({:02x}{:02x}{:02x}{:02x}{:02x}{:02x})",
                                    address[0],
                                    address[1],
                                    address[2],
                                    address[3],
                                    address[4],
                                    address[5]
                                ));
                            }
                            MessagingDevicePathData::Ipv4 {
                                local_ip,
                                remote_ip,
                                local_port,
                                remote_port,
                                ..
                            } => {
                                parts.push(format!(
                                    "IPv4({}.{}.{}.{},{}.{}.{}.{},{},{})",
                                    local_ip[0],
                                    local_ip[1],
                                    local_ip[2],
                                    local_ip[3],
                                    remote_ip[0],
                                    remote_ip[1],
                                    remote_ip[2],
                                    remote_ip[3],
                                    local_port,
                                    remote_port
                                ));
                            }
                            MessagingDevicePathData::Uri { uri } => {
                                parts.push(format!("Uri({})", uri));
                            }
                            MessagingDevicePathData::Raw { subtype, .. } => {
                                parts.push(format!("Messaging({:02x})", subtype));
                            }
                            _ => {}
                        }
                    }
                }
                DevicePathType::End => break,
                _ => {
                    parts.push(format!("{:?}({:02x})", node.device_type, node.subtype));
                }
            }
        }

        write!(f, "{}", parts.join("/"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_path_types() {
        assert_eq!(DevicePathType::from(0x03), DevicePathType::Messaging);
        assert_eq!(DevicePathType::from(0x7f), DevicePathType::End);
        assert_eq!(DevicePathType::from(0x99), DevicePathType::End); // Unknown defaults to End
    }

    #[test]
    fn test_messaging_subtypes() {
        assert_eq!(MessagingSubtype::from(0x0b), MessagingSubtype::MacAddress);
        assert_eq!(MessagingSubtype::from(0x0c), MessagingSubtype::Ipv4);
        assert_eq!(MessagingSubtype::from(0x18), MessagingSubtype::Uri);
    }

    #[test]
    fn test_parse_empty_device_path() {
        let result = DevicePath::parse(&[]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().nodes.len(), 0);
    }

    #[test]
    fn test_parse_uri_device_path() {
        // Create a simple URI device path node
        let uri_data = b"http://example.com/boot.efi";
        let mut data = vec![
            0x03, // Messaging type
            0x18, // URI subtype
            (4 + uri_data.len()) as u8,
            0x00, // Length (little endian)
        ];
        data.extend_from_slice(uri_data);

        // Add end node
        data.extend_from_slice(&[0x7f, 0xff, 0x04, 0x00]);

        let device_path = DevicePath::parse(&data).unwrap();
        assert_eq!(device_path.nodes.len(), 2);
        assert_eq!(
            device_path.extract_uri(),
            Some("http://example.com/boot.efi".to_string())
        );
    }
}
