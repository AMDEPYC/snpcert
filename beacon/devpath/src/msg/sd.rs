//! Secure Digital Device Path
//!
//! This module implements the Secure Digital device path node as defined in UEFI 2.11 specification
//! section 10.3.4.26. This device path describes a Secure Digital device.

use crate::parser::Parser;
use crate::{Error, Head};

/// Secure Digital (SD) Device Path (SubType 0x1A)
///
/// According to UEFI 2.11 spec section 10.3.4.26:
/// - Length: 5 bytes
/// - Slot Number: 1 byte (slot number on the SD controller)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SecureDigital {
    /// SD card slot number
    pub slot: u8,
}

impl<'a> TryFrom<Head<'a>> for SecureDigital {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            slot: node.data.finish(())?,
        })
    }
}
