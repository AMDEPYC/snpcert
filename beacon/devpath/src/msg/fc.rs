//! Fibre Channel Device Path
//!
//! This module implements Fibre Channel device path nodes as defined in UEFI 2.11 specification
//! sections 10.3.4.3 and 10.3.4.21. These device paths describe Fibre Channel target devices.

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

/// Fibre Channel Device Path (SubType 0x03)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FibreChannel {
    /// World Wide Name
    pub wwn: u64,

    /// Logical Unit Number
    pub lun: u64,
}

impl<'a> TryFrom<Head<'a>> for FibreChannel {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        // Skip reserved field
        let _reserved: u32 = node.data.parse(ByteOrder::Little)?;

        Ok(Self {
            wwn: node.data.parse(ByteOrder::Little)?,
            lun: node.data.finish(ByteOrder::Little)?,
        })
    }
}

/// Fibre Channel Ex Device Path (SubType 0x15)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FibreChannelEx {
    /// World Wide Name
    pub wwn: [u8; 8],

    /// Logical Unit Number
    pub lun: [u8; 8],
}

impl<'a> TryFrom<Head<'a>> for FibreChannelEx {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        // Skip reserved field
        let _reserved: u32 = node.data.parse(ByteOrder::Little)?;

        Ok(Self {
            wwn: node.data.parse(())?,
            lun: node.data.finish(())?,
        })
    }
}
