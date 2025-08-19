//! SCSI Device Path
//!
//! This module implements the SCSI device path node as defined in UEFI 2.11 specification
//! section 10.3.4.2. This device path describes a SCSI target device.

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

/// SCSI Device Path (SubType 0x02)
///
/// According to UEFI 2.11 spec section 10.3.4.2:
/// - Length: 8 bytes
/// - Target ID: 2 bytes (PUN)
/// - Logical Unit Number: 2 bytes (LUN)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Scsi {
    /// Target ID on the SCSI bus (PUN)
    pub tid: u16,

    /// Logical Unit Number (LUN)
    pub lun: u16,
}

impl<'a> TryFrom<Head<'a>> for Scsi {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            tid: node.data.parse(ByteOrder::Little)?,
            lun: node.data.finish(ByteOrder::Little)?,
        })
    }
}
