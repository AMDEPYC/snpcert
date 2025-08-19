//! UART Device Path
//!
//! This module implements the UART device path node as defined in UEFI 2.11 specification
//! section 10.3.4.14. This device path describes a serial UART device.

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

/// UART Device Path (SubType 0x0E)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Uart {
    /// Baud Rate (0 = use default)
    pub baud: u64,

    /// Data Bits
    pub data_bits: u8,

    /// Parity
    pub parity: u8,

    /// Stop Bits  
    pub stop_bits: u8,
}

impl<'a> TryFrom<Head<'a>> for Uart {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        // Parse and validate reserved field (must be zero)
        let reserved: u32 = node.data.parse(ByteOrder::Little)?;
        if reserved != 0 {
            return Err(Error::Invalid);
        }

        Ok(Self {
            baud: node.data.parse(ByteOrder::Little)?,
            data_bits: node.data.parse(())?,
            parity: node.data.parse(())?,
            stop_bits: node.data.finish(())?,
        })
    }
}
