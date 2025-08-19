//! ACPI Device Path Types
//!
//! This module implements ACPI device path nodes as defined in UEFI 2.11 specification
//! section 10.3.2. ACPI device paths describe devices that can be found in the
//! ACPI name space.

pub mod adr;
pub mod expanded;
pub mod nvdimm;
pub mod standard;

use crate::error::Type;
use crate::{Error, Head};

/// ACPI Device Path Types
///
/// Represents the different types of ACPI device path nodes defined in UEFI 2.11.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Acpi {
    /// Standard ACPI Device Path (SubType 0x01)
    Standard(standard::Standard),

    /// Expanded ACPI Device Path (SubType 0x02)
    Expanded(expanded::Expanded),

    /// ACPI `_ADR` Device Path (SubType 0x03)
    Adr(adr::Adr),

    /// `NVDIMM` Device Path (SubType 0x04)
    Nvdimm(nvdimm::Nvdimm),
}

impl TryFrom<Head<'_>> for Acpi {
    type Error = Error;

    fn try_from(head: Head<'_>) -> Result<Self, Self::Error> {
        match head.subkind {
            0x01 => TryFrom::try_from(head).map(Acpi::Standard),
            0x02 => TryFrom::try_from(head).map(Acpi::Expanded),
            0x03 => TryFrom::try_from(head).map(Acpi::Adr),
            0x04 => TryFrom::try_from(head).map(Acpi::Nvdimm),
            n => Err(Error::UnknownSubType(Type::Acpi, n)),
        }
    }
}
