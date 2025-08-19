//! Device Logical Unit Device Path
//!
//! This module implements the Device Logical Unit device path node as defined in UEFI 2.11 specification
//! section 10.3.4.8. This device path describes a logical unit number for multi-LUN devices.

use crate::parser::Parser;
use crate::{Error, Head};

/// Device Logical Unit (SubType 0x11)
///
/// According to UEFI 2.11 spec section 10.3.4.8:
/// - Length: 5 bytes
/// - LUN: 1 byte (Logical Unit Number for the interface)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceLogicalUnit {
    /// Logical Unit Number
    pub lun: u8,
}

impl<'a> TryFrom<Head<'a>> for DeviceLogicalUnit {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            lun: node.data.finish(())?,
        })
    }
}
