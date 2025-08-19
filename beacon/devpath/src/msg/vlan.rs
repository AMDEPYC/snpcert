//! VLAN Device Path
//!
//! This module implements the VLAN device path node as defined in UEFI 2.11 specification
//! section 10.3.4.13. This device path describes a 802.1Q VLAN interface.

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

/// VLAN Device Path (SubType 0x14)
///
/// According to UEFI 2.11 spec section 10.3.4.13:
/// - Length: 6 bytes
/// - VLAN ID: 2 bytes (VLAN identifier 0-4094)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vlan {
    /// VLAN identifier (0-4094)
    pub id: u16,
}

impl<'a> TryFrom<Head<'a>> for Vlan {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: node.data.finish(ByteOrder::Little)?,
        })
    }
}
