//! BIOS Device Path Types
//!
//! This module implements BIOS device path nodes as defined in UEFI 2.11 specification
//! section 10.3.5. BIOS device paths are used for legacy BIOS boot support.

pub mod spec;

use crate::error::Type;
use crate::{Error, Head};

/// BIOS Device Path Types
///
/// Represents BIOS device path nodes for legacy boot support.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Bios {
    /// BIOS Boot Specification Device Path (SubType 0x01)
    BootSpec(spec::BootSpec),
}

impl TryFrom<Head<'_>> for Bios {
    type Error = Error;

    fn try_from(head: Head<'_>) -> Result<Self, Self::Error> {
        match head.subkind {
            0x01 => TryFrom::try_from(head).map(Self::BootSpec),
            n => Err(Error::UnknownSubType(Type::Bios, n)),
        }
    }
}
