//! Hard Drive Media Device Path
//!
//! This module implements the Hard Drive media device path node as defined in UEFI 2.11 specification
//! section 10.3.3.1. This device path describes a partition on a hard drive.

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

/// Partition signature types for hard drive device paths
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Signature {
    /// No signature
    None,

    /// MBR signature (4 bytes)
    Mbr([u8; 4]),

    /// GPT signature (16 bytes)
    Gpt([u8; 16]),
}

/// Partition table format types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    /// Master Boot Record format
    Mbr,

    /// GUID Partition Table format
    Gpt,
}

/// Hard Drive Media Device Path (SubType 0x01)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HardDrive {
    /// Partition number
    pub number: u32,

    /// Starting LBA of the partition
    pub start: u64,

    /// Size of the partition in LBA
    pub size: u64,

    /// Partition table format
    pub format: Format,

    /// Partition signature
    pub signature: Signature,
}

impl<'a> TryFrom<Head<'a>> for HardDrive {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        let number = node.data.parse(ByteOrder::Little)?;
        let start = node.data.parse(ByteOrder::Little)?;
        let size = node.data.parse(ByteOrder::Little)?;
        let guid: [u8; 16] = node.data.parse(())?;

        let format = match node.data.parse(())? {
            1u8 => Format::Mbr,
            2u8 => Format::Gpt,
            _ => return Err(Error::Invalid),
        };

        let signature = match node.data.finish(())? {
            0u8 => Signature::None,
            1u8 => Signature::Mbr([guid[0], guid[1], guid[2], guid[3]]),
            2u8 => Signature::Gpt(guid),
            _ => return Err(Error::Invalid),
        };

        Ok(Self {
            number,
            start,
            size,
            format,
            signature,
        })
    }
}
