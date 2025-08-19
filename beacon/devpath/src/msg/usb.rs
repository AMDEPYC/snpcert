//! USB Device Path
//!
//! This module implements USB device path nodes as defined in UEFI 2.11 specification
//! sections 10.3.4.5, 10.3.4.15, and 10.3.4.16. These device paths describe USB devices.

use crate::parser::{ByteOrder, Format, Parser};
use crate::{Error, Head};
use alloc::string::String;

/// USB Device Path (SubType 0x05)
///
/// According to UEFI 2.11 spec section 10.3.4.5:
/// - Length: 6 bytes
/// - Parent Hub Port Number: 1 byte
/// - Interface: 1 byte
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Usb {
    /// USB Parent Hub Port Number
    pub port: u8,

    /// USB Interface Number
    pub interface: u8,
}

impl<'a> TryFrom<Head<'a>> for Usb {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            port: node.data.parse(())?,
            interface: node.data.finish(())?,
        })
    }
}

/// USB Class Device Path (SubType 0x0F)
///
/// According to UEFI 2.11 spec section 10.3.4.8:
/// - Length: 11 bytes
/// - Vendor ID: 2 bytes (0xFFFF matches any)
/// - Product ID: 2 bytes (0xFFFF matches any)
/// - Device Class: 1 byte (0xFF matches any)
/// - Device Subclass: 1 byte (0xFF matches any)
/// - Device Protocol: 1 byte (0xFF matches any)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UsbClass {
    /// Vendor ID (0xFFFF matches any)
    pub vid: u16,

    /// Product ID (0xFFFF matches any)
    pub pid: u16,

    /// Device Class (0xFF matches any)
    pub class: u8,

    /// Device Subclass (0xFF matches any)
    pub subclass: u8,

    /// Device Protocol (0xFF matches any)
    pub protocol: u8,
}

impl<'a> TryFrom<Head<'a>> for UsbClass {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            vid: node.data.parse(ByteOrder::Little)?,
            pid: node.data.parse(ByteOrder::Little)?,
            class: node.data.parse(())?,
            subclass: node.data.parse(())?,
            protocol: node.data.finish(())?,
        })
    }
}

/// USB WWID Device Path (SubType 0x10)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UsbWwid {
    /// Interface Number
    pub interface: u16,

    /// Vendor ID
    pub vid: u16,

    /// Product ID
    pub pid: u16,

    /// Serial Number
    pub serial: String,
}

impl<'a> TryFrom<Head<'a>> for UsbWwid {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            interface: node.data.parse(ByteOrder::Little)?,
            vid: node.data.parse(ByteOrder::Little)?,
            pid: node.data.parse(ByteOrder::Little)?,
            serial: node.data.finish(Format::Utf16(Some(node.data.len())))?,
        })
    }
}
