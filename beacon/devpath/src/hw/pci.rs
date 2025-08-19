//! PCI Device Path
//!
//! This module implements the PCI device path node as defined in UEFI 2.11 specification
//! section 10.3.1.1. This device path describes a PCI device.

use crate::parser::Parser;
use crate::{Error, Head};

/// PCI Device Path (SubType 0x01)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pci {
    /// PCI function number
    pub function: u8,

    /// PCI device number
    pub device: u8,
}

impl TryFrom<Head<'_>> for Pci {
    type Error = Error;

    fn try_from(mut node: Head<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            function: node.data.parse(())?,
            device: node.data.finish(())?,
        })
    }
}
