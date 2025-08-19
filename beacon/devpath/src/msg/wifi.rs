//! Wi-Fi Device Path
//!
//! This module implements the Wi-Fi device path node as defined in UEFI 2.11 specification
//! section 10.3.4.26. This device path describes a Wi-Fi device.

use crate::parser::Parser;
use crate::{Error, Head};

/// Wi-Fi Device Path (SubType 0x1C)
///
/// According to UEFI 2.11 spec section 10.3.4.26:
/// - Length: 36 bytes
/// - SSID: 32 bytes (Service Set Identifier, raw octet string)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wifi {
    /// Service Set Identifier (SSID) - 32 bytes, raw octet string
    pub ssid: [u8; 32],
}

impl<'a> TryFrom<Head<'a>> for Wifi {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            ssid: node.data.finish(())?,
        })
    }
}
