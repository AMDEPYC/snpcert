//! Hardware Device Path Types
//!
//! This module implements hardware device path nodes as defined in UEFI 2.11 specification
//! section 10.3.1. Hardware device paths describe physical hardware devices.

pub mod bmc;
pub mod controller;
pub mod memmap;
pub mod pccard;
pub mod pci;
pub mod vendor;

use crate::error::Type;

use crate::{Error, Head};

/// Hardware Device Path Types
///
/// Represents the different types of hardware device path nodes defined in UEFI 2.11.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Hardware {
    /// PCI Device Path (SubType 0x01)
    Pci(pci::Pci),

    /// PCCARD Device Path (SubType 0x02)
    PcCard(pccard::PcCard),

    /// Memory Mapped Device Path (SubType 0x03)
    MemMap(memmap::MemMap),

    /// Vendor Device Path (SubType 0x04)
    Vendor(vendor::Vendor),

    /// Controller Device Path (SubType 0x05)
    Controller(controller::Controller),

    /// BMC Device Path (SubType 0x06)
    Bmc(bmc::Bmc),
}

impl TryFrom<Head<'_>> for Hardware {
    type Error = Error;

    fn try_from(head: Head<'_>) -> Result<Self, Self::Error> {
        match head.subkind {
            0x01 => TryFrom::try_from(head).map(Hardware::Pci),
            0x02 => TryFrom::try_from(head).map(Hardware::PcCard),
            0x03 => TryFrom::try_from(head).map(Hardware::MemMap),
            0x04 => TryFrom::try_from(head).map(Hardware::Vendor),
            0x05 => TryFrom::try_from(head).map(Hardware::Controller),
            0x06 => TryFrom::try_from(head).map(Hardware::Bmc),
            n => Err(Error::UnknownSubType(Type::Hardware, n)),
        }
    }
}
