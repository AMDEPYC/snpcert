//! eMMC Device Path
//!
//! This module implements the eMMC device path node as defined in UEFI 2.11 specification
//! section 10.3.4.27. This device path describes an embedded MultiMediaCard device.

use crate::parser::Parser;
use crate::{Error, Head};

/// eMMC Device Path (SubType 0x1D)
///
/// According to UEFI 2.11 spec section 10.3.4.27:
/// - Length: 5 bytes
/// - Slot Number: 1 byte
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EMmc {
    /// eMMC slot number
    pub slot: u8,
}

impl<'a> TryFrom<Head<'a>> for EMmc {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            slot: node.data.finish(())?,
        })
    }
}
