//! Baseboard Management Controller Device Path
//!
//! This module implements the BMC device path node as defined in UEFI 2.11 specification
//! section 10.3.1.6. This device path describes a BMC (Baseboard Management Controller)
//! device and the method to access it.

use crate::parser::{ByteOrder, Invalid, Parser};
use crate::{Error, Head};

/// BMC interface types as defined in the UEFI specification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    /// Unknown BMC interface type
    Unknown,

    /// Keyboard Controller Style BMC interface
    KeyboardControllerStyle,

    /// Server Management Interface Chip BMC interface  
    ServerManagementInterfaceChip,

    /// Block Transfer BMC interface
    BlockTransfer,
}

impl Parser<Type> for &[u8] {
    type Arg = ();

    fn parse(&mut self, arg: Self::Arg) -> Result<Type, Invalid> {
        match self.parse(arg)? {
            0x00u8 => Ok(Type::Unknown),
            0x01u8 => Ok(Type::KeyboardControllerStyle),
            0x02u8 => Ok(Type::ServerManagementInterfaceChip),
            0x03u8 => Ok(Type::BlockTransfer),
            _ => Err(Invalid),
        }
    }
}

/// BMC Device Path (SubType 0x06)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bmc {
    /// The type of BMC interface
    pub kind: Type,

    /// Base address of the BMC interface
    pub addr: u64,
}

impl<'a> TryFrom<Head<'a>> for Bmc {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: node.data.parse(())?,
            addr: node.data.finish(ByteOrder::Little)?,
        })
    }
}
