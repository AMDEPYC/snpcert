//! RAM Disk Media Device Path
//!
//! This module implements the RAM Disk media device path node as defined in UEFI 2.11 specification
//! section 10.3.3.9. This device path describes a RAM disk media device.

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

/// RAM Disk (SubType 0x09)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RamDisk {
    /// Starting memory address
    pub start: u64,

    /// Ending memory address
    pub end: u64,

    /// Type GUID
    pub guid: [u8; 16],

    /// RAM disk instance number
    pub instance: u16,
}

impl<'a> TryFrom<Head<'a>> for RamDisk {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            start: node.data.parse(ByteOrder::Little)?,
            end: node.data.parse(ByteOrder::Little)?,
            guid: node.data.parse(())?,
            instance: node.data.finish(ByteOrder::Little)?,
        })
    }
}
