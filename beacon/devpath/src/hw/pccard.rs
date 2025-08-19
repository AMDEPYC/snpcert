//! PC Card Device Path
//!
//! This module implements the PC Card device path node as defined in UEFI 2.11 specification
//! section 10.3.1.2. This device path describes a PC Card device.

use crate::parser::Parser;
use crate::{Error, Head};

/// PCCARD Device Path (SubType 0x02)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PcCard {
    function: u8,
}

impl<'a> TryFrom<Head<'a>> for PcCard {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            function: node.data.finish(())?,
        })
    }
}
