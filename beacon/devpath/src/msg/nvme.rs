//! NVMe Namespace Device Path
//!
//! This module implements the NVMe Namespace device path node as defined in UEFI 2.11 specification
//! section 10.3.4.21. This device path describes an NVMe namespace.

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

/// NVMe Namespace Device Path (SubType 0x17)
///
/// According to UEFI 2.11 spec section 10.3.4.21:
/// - Length: 16 bytes
/// - Namespace Identifier (NSID): 4 bytes - values 0 and 0xFFFFFFFF are invalid
/// - IEEE Extended Unique Identifier (EUI-64): 8 bytes - devices without EUI-64 use 0
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NvmeNamespace {
    /// Namespace identifier (NSID). The values of 0 and 0xFFFFFFFF are invalid.
    pub nsid: u32,

    /// IEEE Extended Unique Identifier (EUI-64). Devices without an EUI-64
    /// value must initialize this field with a value of 0.
    pub eui: u64,
}

impl<'a> TryFrom<Head<'a>> for NvmeNamespace {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            nsid: node.data.parse(ByteOrder::Little)?,
            eui: node.data.finish(ByteOrder::Little)?,
        })
    }
}
