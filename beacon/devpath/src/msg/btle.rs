//! Bluetooth Low Energy Device Path
//!
//! This module implements the Bluetooth LE device path node as defined in UEFI 2.11 specification
//! section 10.3.4.26. This device path describes a Bluetooth Low Energy device.

use crate::parser::{Invalid, Parser};
use crate::{Error, Head};

/// Bluetooth LE Address Type as defined in UEFI 2.11 spec
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddressType {
    /// Public Device Address
    Public,

    /// Random Device Address
    Random,
}

impl Parser<AddressType> for &[u8] {
    type Arg = ();

    fn parse(&mut self, arg: Self::Arg) -> Result<AddressType, Invalid> {
        match self.parse(arg)? {
            0x00u8 => Ok(AddressType::Public),
            0x01u8 => Ok(AddressType::Random),
            _ => Err(Invalid),
        }
    }
}

/// BluetoothLE Device Path (SubType 0x1E)
///
/// According to UEFI 2.11 spec section 10.3.4.28:
/// - Length: 11 bytes
/// - Bluetooth Device Address: 6 bytes (48-bit Bluetooth device address)
/// - Address Type: 1 byte (0x00 = Public, 0x01 = Random)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BluetoothLe {
    /// 48-bit Bluetooth device address
    pub address: [u8; 6],

    /// Address Type: Public or Random Device Address
    pub kind: AddressType,
}

impl<'a> TryFrom<Head<'a>> for BluetoothLe {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            address: node.data.parse(())?,
            kind: node.data.finish(())?,
        })
    }
}
