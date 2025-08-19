//! CD-ROM Media Device Path
//!
//! This module implements the CD-ROM media device path node as defined in UEFI 2.11 specification
//! section 10.3.3.2. This device path describes a bootable CD-ROM media using the El Torito format.

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

/// CD-ROM Media Device Path (SubType 0x02)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CdRom {
    /// Boot entry ID for El Torito bootable CD-ROM
    pub boot_entry: u32,

    /// Starting LBA of the partition
    pub partition_start: u64,

    /// Size of the partition in LBA
    pub partition_size: u64,
}

impl<'a> TryFrom<Head<'a>> for CdRom {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            boot_entry: node.data.parse(ByteOrder::Little)?,
            partition_start: node.data.parse(ByteOrder::Little)?,
            partition_size: node.data.finish(ByteOrder::Little)?,
        })
    }
}
