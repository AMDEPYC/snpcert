//! BIOS Boot Specification device path implementation
//!
//! This module implements the BIOS Boot Specification device path as defined in UEFI 2.11
//! specification. Used for legacy BIOS boot options.

use crate::parser::{ByteOrder, Format, Parser};
use crate::{Error, Head};

/// BIOS Boot Specification Device Path (SubType 0x01)
///
/// Used to point to a boot option defined via the BIOS Boot Specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BootSpec {
    /// Device type as defined by the BIOS Boot Specification
    pub device_type: u16,

    /// Status flags as defined by the BIOS Boot Specification  
    pub status_flag: u16,

    /// Null-terminated ASCII description string of the boot device
    pub description: String,
}

impl TryFrom<Head<'_>> for BootSpec {
    type Error = Error;

    fn try_from(mut node: Head<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            device_type: node.data.parse(ByteOrder::Little)?,
            status_flag: node.data.parse(ByteOrder::Little)?,
            description: node.data.finish(Format::Utf8(None))?,
        })
    }
}
