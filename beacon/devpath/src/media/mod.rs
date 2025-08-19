//! Media Device Path Types
//!
//! This module implements media device path nodes as defined in UEFI 2.11 specification
//! section 10.3.3. Media device paths describe storage media and partitions.

pub mod cdrom;
pub mod file;
pub mod guid;
pub mod hd;
pub mod offset;
pub mod ramdisk;
pub mod vendor;

use crate::error::Type;

use crate::{Error, Head};

/// Media Device Path router enum
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Media {
    /// Hard Drive media device path
    HardDrive(hd::HardDrive),

    /// CD-ROM media device path
    CdRom(cdrom::CdRom),

    /// Vendor-defined media device path
    Vendor(vendor::Vendor),

    /// File path media device path
    FilePath(file::FilePath),

    /// Media protocol device path
    MediaProtocol(guid::Guid),

    /// PIWG firmware file device path
    PiwgFirmwareFile(guid::Guid),

    /// PIWG firmware volume device path
    PiwgFirmwareVolume(guid::Guid),

    /// Relative offset range device path
    RelativeOffsetRange(offset::RelativeRange),

    /// RAM disk device path
    RamDisk(ramdisk::RamDisk),
}

impl TryFrom<Head<'_>> for Media {
    type Error = Error;

    fn try_from(head: Head<'_>) -> Result<Self, Self::Error> {
        match head.subkind {
            0x01 => TryFrom::try_from(head).map(Media::HardDrive),
            0x02 => TryFrom::try_from(head).map(Media::CdRom),
            0x03 => TryFrom::try_from(head).map(Media::Vendor),
            0x04 => TryFrom::try_from(head).map(Media::FilePath),
            0x05 => TryFrom::try_from(head).map(Media::MediaProtocol),
            0x06 => TryFrom::try_from(head).map(Media::PiwgFirmwareFile),
            0x07 => TryFrom::try_from(head).map(Media::PiwgFirmwareVolume),
            0x08 => TryFrom::try_from(head).map(Media::RelativeOffsetRange),
            0x09 => TryFrom::try_from(head).map(Media::RamDisk),
            n => Err(Error::UnknownSubType(Type::Media, n)),
        }
    }
}
