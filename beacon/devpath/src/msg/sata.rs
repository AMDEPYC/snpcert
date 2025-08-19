//! SATA Device Path
//!
//! This module implements the SATA device path node as defined in UEFI 2.11 specification
//! section 10.3.4.6. This device path describes a Serial ATA device.

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

/// SATA Device Path (SubType 0x12)
///
/// According to UEFI 2.11 spec section 10.3.4.6:
/// - Length: 10 bytes
/// - HBA Port Number: 2 bytes (0xFFFF reserved)
/// - Port Multiplier Port Number: 2 bytes (0xFFFF if direct connect)
/// - Logical Unit Number: 2 bytes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sata {
    /// HBA Port Number (0xFFFF reserved)
    pub hba_port: u16,

    /// Port Multiplier Port Number (0xFFFF if direct connect)
    pub pm_port: u16,

    /// Logical Unit Number
    pub lun: u16,
}

impl<'a> TryFrom<Head<'a>> for Sata {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            hba_port: node.data.parse(ByteOrder::Little)?,
            pm_port: node.data.parse(ByteOrder::Little)?,
            lun: node.data.finish(ByteOrder::Little)?,
        })
    }
}
