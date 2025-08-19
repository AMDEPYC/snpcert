//! Vendor-defined hardware device path node implementation
//!
//! This module contains the implementation for vendor-defined hardware device paths
//! as specified in UEFI 2.11 specification section 10.3.1.4. These device paths
//! allow hardware vendors to define custom device path formats using their own GUID.

use crate::parser::Parser;
use crate::{Error, Head};

/// Vendor Device Path (SubType 0x0A)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vendor {
    /// Vendor GUID
    pub guid: [u8; 16],

    /// Vendor-specific data
    pub data: Vec<u8>,
}

impl TryFrom<Head<'_>> for Vendor {
    type Error = Error;

    fn try_from(mut node: Head<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            guid: node.data.parse(())?,
            data: node.data.finish(node.data.len())?,
        })
    }
}
