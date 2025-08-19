//! ATAPI Device Path
//!
//! This module implements the ATAPI device path node as defined in UEFI 2.11 specification
//! section 10.3.4.1. This device path describes an ATAPI device.

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

/// ATAPI Device Path (SubType 0x01)
///
/// According to UEFI 2.11 spec section 10.3.4.1:
/// - Length: 8 bytes
/// - PrimarySecondary: 1 byte (0=primary, 1=secondary)
/// - SlaveMaster: 1 byte (0=master, 1=slave)
/// - Logical Unit Number: 2 bytes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Atapi {
    /// Primary (0) or Secondary (1) controller
    pub primary: u8,

    /// Master (0) or Slave (1) device
    pub slave: u8,

    /// Logical Unit Number
    pub lun: u16,
}

impl<'a> TryFrom<Head<'a>> for Atapi {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            primary: node.data.parse(())?,
            slave: node.data.parse(())?,
            lun: node.data.finish(ByteOrder::Little)?,
        })
    }
}
