//! UFS Device Path
//!
//! This module implements the UFS device path node as defined in UEFI 2.11 specification
//! section 10.3.4.23. This device path describes a Universal Flash Storage device.

use crate::parser::Parser;
use crate::{Error, Head};

/// UFS Device Path (SubType 0x19)
///
/// According to UEFI 2.11 spec section 10.3.4.23:
/// - Length: 6 bytes
/// - Target ID: 1 byte (Target ID on the UFS interface - PUN)
/// - LUN: 1 byte (Logical Unit Number)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ufs {
    /// Target ID on the UFS interface (PUN)
    pub tid: u8,

    /// Logical Unit Number (LUN)
    pub lun: u8,
}

impl<'a> TryFrom<Head<'a>> for Ufs {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            tid: node.data.parse(())?,
            lun: node.data.finish(())?,
        })
    }
}
