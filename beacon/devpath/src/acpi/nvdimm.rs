//! NVDIMM device path implementation
//!
//! This module implements the NVDIMM device path as defined in UEFI 2.11 specification.
//! Used for non-volatile dual in-line memory modules.

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

/// `NVDIMM` Device Path (SubType 0x04)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Nvdimm {
    handle: u32,
}

impl<'a> TryFrom<Head<'a>> for Nvdimm {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            handle: node.data.finish(ByteOrder::Little)?,
        })
    }
}
