//! SAS Extended Device Path
//!
//! This module implements the SAS Extended device path node as defined in UEFI 2.11 specification
//! section 10.3.4.22. This device path describes a Serial Attached SCSI Extended device.

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

/// SAS Device and Topology Information as defined in UEFI 2.11 spec
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceTopologyInfo(pub u16);

impl DeviceTopologyInfo {
    /// No specific device type
    pub const NO_DRIVE_TYPE: Self = Self(0x0000);

    /// SAS Drive
    pub const SAS_INTERNAL: Self = Self(0x0001);

    /// SATA Drive  
    pub const SATA_INTERNAL: Self = Self(0x0002);

    /// SAS Port Multiplier
    pub const SAS_EXTERNAL: Self = Self(0x0011);

    /// SATA Port Multiplier
    pub const SATA_EXTERNAL: Self = Self(0x0012);

    /// SATA Port Multiplier Port
    pub const SATA_PM_PORT: Self = Self(0x0013);

    /// SAS Expander Device
    pub const SAS_EXPANDER: Self = Self(0x0021);
}

/// SAS Extended Device Path (SubType 0x16)
///
/// According to UEFI 2.11 spec section 10.3.4.20:
/// - Length: 32 bytes
/// - SAS Address: 8 bytes (SAS address for the device)
/// - LUN: 8 bytes (Logical Unit Number)  
/// - Device and Topology Info: 2 bytes (device type information)
/// - Relative Target Port: 2 bytes (RTP identifier)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SasExtended {
    /// SAS Address for the device (8 bytes)
    pub sas_address: [u8; 8],

    /// Logical Unit Number (8 bytes)
    pub lun: [u8; 8],

    /// Device and Topology Information
    pub device_topology_info: DeviceTopologyInfo,

    /// Relative Target Port identifier
    pub rtp: u16,
}

impl<'a> TryFrom<Head<'a>> for SasExtended {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            sas_address: node.data.parse(())?,
            lun: node.data.parse(())?,
            device_topology_info: DeviceTopologyInfo(node.data.parse(ByteOrder::Little)?),
            rtp: node.data.finish(ByteOrder::Little)?,
        })
    }
}
