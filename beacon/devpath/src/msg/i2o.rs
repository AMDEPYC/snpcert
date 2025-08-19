//! I2O Device Path
//!
//! This module implements the I2O device path node as defined in UEFI 2.11 specification
//! section 10.3.4.2. This device path describes an I2O Random Block Storage Class device.

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

/// I2O Device Path (SubType 0x02)
///
/// According to UEFI 2.11 spec section 10.3.4.2:
/// - Length: 8 bytes
/// - TID: 4 bytes (Target ID for the device)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I2o {
    /// Target ID (TID)
    pub tid: u32,
}

impl<'a> TryFrom<Head<'a>> for I2o {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            tid: node.data.finish(ByteOrder::Little)?,
        })
    }
}
