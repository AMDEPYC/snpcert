//! ACPI Device Path expanded format implementation
//!
//! This module implements the expanded ACPI device path format as defined in UEFI 2.11
//! specification. The expanded format includes additional string-based identifiers.

use crate::parser::{ByteOrder, Format, Parser};
use crate::{Error, Head};

/// Helper struct for ACPI device identifiers
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(pub u32, pub String);

/// Expanded ACPI Device Path (SubType 0x02)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expanded {
    /// Hardware ID - identifies the device's Plug and Play ID
    pub hid: Id,

    /// Unique ID - distinguishes multiple instances of the same device
    pub uid: Id,

    /// Compatible ID - identifies devices compatible with this device
    pub cid: Id,
}

impl TryFrom<Head<'_>> for Expanded {
    type Error = Error;

    fn try_from(mut node: Head<'_>) -> Result<Self, Self::Error> {
        let hid = node.data.parse(ByteOrder::Little)?;
        let uid = node.data.parse(ByteOrder::Little)?;
        let cid = node.data.parse(ByteOrder::Little)?;

        Ok(Self {
            hid: Id(hid, node.data.parse(Format::Utf8(None))?),
            uid: Id(uid, node.data.parse(Format::Utf8(None))?),
            cid: Id(cid, node.data.finish(Format::Utf8(None))?),
        })
    }
}
