//! Bluetooth Device Path
//!
//! This module implements the Bluetooth device path node as defined in UEFI 2.11 specification
//! section 10.3.4.25. This device path describes a Bluetooth device.

use crate::parser::Parser;
use crate::{Error, Head};

/// Bluetooth Device Path (SubType 0x1B)
///
/// According to UEFI 2.11 spec section 10.3.4.25:
/// - Length: 10 bytes
/// - Bluetooth Device Address: 6 bytes (48-bit Bluetooth device address)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bluetooth {
    /// 48-bit Bluetooth device address
    pub address: [u8; 6],
}

impl<'a> TryFrom<Head<'a>> for Bluetooth {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            address: node.data.finish(())?,
        })
    }
}
