//! IEEE 1394 Device Path
//!
//! This module implements the IEEE 1394 device path node as defined in UEFI 2.11 specification
//! section 10.3.4.4. This device path describes a FireWire device.

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

/// IEEE 1394 Device Path (SubType 0x04)
///
/// According to UEFI 2.11 spec section 10.3.4.4:
/// - Length: 16 bytes
/// - Reserved: 4 bytes (must be zero)
/// - GUID: 8 bytes (1394 Global Unique ID, not the same as EFI_GUID)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ieee1394 {
    /// 1394 Global Unique ID (GUID) - Note: This is per 1394 spec, not EFI_GUID
    pub guid: u64,
}

impl<'a> TryFrom<Head<'a>> for Ieee1394 {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        // Parse and validate reserved field (must be zero)
        let reserved: u32 = node.data.parse(ByteOrder::Little)?;
        if reserved != 0 {
            return Err(Error::Invalid);
        }

        Ok(Self {
            guid: node.data.finish(ByteOrder::Little)?,
        })
    }
}
