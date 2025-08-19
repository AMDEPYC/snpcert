//! NVDIMM Namespace Device Path
//!
//! This module implements the NVDIMM Namespace device path node as defined in UEFI 2.11 specification
//! section 10.3.4.30. This device path describes an NVDIMM namespace.

use crate::parser::Parser;
use crate::{Error, Head};

/// NVDIMM Namespace Device Path (SubType 0x20)
///
/// According to UEFI 2.11 spec section 10.3.4.30:
/// - Length: 20 bytes
/// - Namespace UUID: 16 bytes (UUID identifying the NVDIMM namespace)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NvdimmNamespace {
    /// Namespace UUID (16 bytes)
    pub uuid: [u8; 16],
}

impl<'a> TryFrom<Head<'a>> for NvdimmNamespace {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            uuid: node.data.finish(())?,
        })
    }
}
