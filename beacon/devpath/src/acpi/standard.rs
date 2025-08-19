//! ACPI Device Path standard format implementation
//!
//! This module implements the standard ACPI device path format as defined in UEFI 2.11
//! specification. The standard format uses 32-bit numeric identifiers.

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

/// Standard ACPI Device Path (SubType 0x01)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Standard {
    /// Hardware ID - ACPI hardware identifier
    pub hid: u32,

    /// Unique ID - distinguishes multiple instances of the same device
    pub uid: u32,
}

impl<'a> TryFrom<Head<'a>> for Standard {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            hid: node.data.parse(ByteOrder::Little)?,
            uid: node.data.finish(ByteOrder::Little)?,
        })
    }
}
