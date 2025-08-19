//! Memory Mapped Device Path
//!
//! This module implements the Memory Mapped device path node as defined in UEFI 2.11 specification
//! section 10.3.1.3. This device path describes a range of memory mapped I/O addresses.

use crate::parser::{ByteOrder, Invalid, Parser};
use crate::{Error, Head};

/// Memory types as defined in the UEFI specification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryType {
    /// Reserved memory type
    Reserved,

    /// Loader code memory
    LoaderCode,

    /// Loader data memory
    LoaderData,

    /// Boot services code memory
    BootServicesCode,

    /// Boot services data memory
    BootServicesData,

    /// Runtime services code memory
    RuntimeServicesCode,

    /// Runtime services data memory
    RuntimeServicesData,

    /// Conventional memory
    ConventionalMemory,

    /// Unusable memory
    UnusableMemory,

    /// `ACPI` reclaimable memory
    AcpiReclaimMemory,

    /// `ACPI` memory `NVS`
    AcpiMemoryNvs,

    /// Memory mapped I/O
    MemoryMappedIo,

    /// Memory mapped I/O port space
    MemoryMappedIoPortSpace,

    /// `PAL` code memory
    PalCode,

    /// Persistent memory
    PersistentMemory,

    /// Unaccepted memory
    UnacceptedMemory,

    /// Maximum memory type
    MaxMemory,
}

impl Parser<MemoryType> for &[u8] {
    type Arg = ByteOrder;

    fn parse(&mut self, arg: Self::Arg) -> Result<MemoryType, Invalid> {
        match self.parse(arg)? {
            0u32 => Ok(MemoryType::Reserved),
            1 => Ok(MemoryType::LoaderCode),
            2 => Ok(MemoryType::LoaderData),
            3 => Ok(MemoryType::BootServicesCode),
            4 => Ok(MemoryType::BootServicesData),
            5 => Ok(MemoryType::RuntimeServicesCode),
            6 => Ok(MemoryType::RuntimeServicesData),
            7 => Ok(MemoryType::ConventionalMemory),
            8 => Ok(MemoryType::UnusableMemory),
            9 => Ok(MemoryType::AcpiReclaimMemory),
            10 => Ok(MemoryType::AcpiMemoryNvs),
            11 => Ok(MemoryType::MemoryMappedIo),
            12 => Ok(MemoryType::MemoryMappedIoPortSpace),
            13 => Ok(MemoryType::PalCode),
            14 => Ok(MemoryType::PersistentMemory),
            15 => Ok(MemoryType::UnacceptedMemory),
            16 => Ok(MemoryType::MaxMemory),
            _ => Err(Invalid),
        }
    }
}

/// Memory Mapped Device Path (SubType 0x03)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemMap {
    /// The memory type
    pub kind: MemoryType,

    /// Starting memory address
    pub start: u64,

    /// Ending memory address
    pub end: u64,
}

impl<'a> TryFrom<Head<'a>> for MemMap {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: node.data.parse(ByteOrder::Little)?,
            start: node.data.parse(ByteOrder::Little)?,
            end: node.data.finish(ByteOrder::Little)?,
        })
    }
}
