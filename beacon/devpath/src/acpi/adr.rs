//! ACPI Device Path address (`_ADR`) implementation
//!
//! This module implements the ACPI `_ADR` device path as defined in UEFI 2.11 specification.
//! Used for identifying video output devices using ACPI `_ADR` values.

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

use alloc::vec::Vec;

/// ACPI `_ADR` Device Path (SubType 0x03) - for video output devices
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Adr {
    adrs: Vec<u32>,
}

impl TryFrom<Head<'_>> for Adr {
    type Error = Error;

    fn try_from(mut node: Head<'_>) -> Result<Self, Self::Error> {
        let mut values = Vec::new();

        while node.data.len() >= 4 {
            let value: u32 = node.data.parse(ByteOrder::Little)?;
            values.push(value);
        }

        if !node.data.is_empty() {
            return Err(Error::Invalid);
        }

        Ok(Self { adrs: values })
    }
}
