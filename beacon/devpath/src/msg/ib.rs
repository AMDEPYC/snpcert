//! InfiniBand Device Path
//!
//! This module implements the InfiniBand device path node as defined in UEFI 2.11 specification
//! section 10.3.4.5. This device path describes an InfiniBand fabric device.

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

/// InfiniBand Resource Flags as defined in UEFI 2.11 spec
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceFlags(pub u32);

impl ResourceFlags {
    /// IOC/Service flag
    pub const SERVICE: Self = Self(1 << 0);

    /// Extended boot flag
    pub const EXTENDED_BOOT: Self = Self(1 << 1);

    /// Console protocol flag
    pub const CONSOLE_PROTOCOL: Self = Self(1 << 2);

    /// Storage protocol flag
    pub const STORAGE_PROTOCOL: Self = Self(1 << 3);

    /// Network protocol flag
    pub const NETWORK_PROTOCOL: Self = Self(1 << 4);
}

/// InfiniBand Device Path (SubType 0x09)
///
/// According to UEFI 2.11 spec section 10.3.4.14:
/// - Length: 48 bytes
/// - Resource Flags: 4 bytes (flags indicating the resource type)
/// - Port GID: 16 bytes (Global Identifier for the IB port)
/// - IOC GUID/Service ID: 16 bytes (either IOC GUID or Service ID)
/// - Target Port ID: 8 bytes (Target port identifier)
/// - Device ID: 8 bytes (Device identifier)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InfiniBand {
    /// Resource flags indicating resource type
    pub flags: ResourceFlags,

    /// Port Global Identifier (16 bytes)
    pub gid: [u8; 16],

    /// IOC GUID or Service ID (16 bytes)
    pub sid: u64,

    /// Target Port ID (8 bytes)
    pub tid: u64,

    /// Device ID (8 bytes)
    pub did: u64,
}

impl<'a> TryFrom<Head<'a>> for InfiniBand {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            flags: ResourceFlags(node.data.parse(ByteOrder::Little)?),
            gid: node.data.parse(())?,
            sid: node.data.parse(ByteOrder::Little)?,
            tid: node.data.parse(ByteOrder::Little)?,
            did: node.data.finish(ByteOrder::Little)?,
        })
    }
}
