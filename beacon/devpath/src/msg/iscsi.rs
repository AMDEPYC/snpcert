//! iSCSI Device Path
//!
//! This module implements the iSCSI device path node as defined in UEFI 2.11 specification
//! section 10.3.4.17. This device path describes an iSCSI target device.

use crate::parser::{ByteOrder, Format, Invalid, Parser};
use crate::{Error, Head};
use alloc::string::String;

/// iSCSI Protocol as defined in UEFI 2.11 spec section 10.3.4.17
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protocol {
    /// TCP (Transmission Control Protocol)
    Tcp,
}

impl Parser<Protocol> for &[u8] {
    type Arg = ByteOrder;

    fn parse(&mut self, arg: Self::Arg) -> Result<Protocol, Invalid> {
        match self.parse(arg)? {
            0x00u16 => Ok(Protocol::Tcp),
            _ => Err(Invalid),
        }
    }
}

/// iSCSI Login Options as defined in UEFI 2.11 spec section 10.3.4.17
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoginOptions(pub u16);

impl LoginOptions {
    /// No special login options
    pub const NONE: Self = Self(0x0000);

    /// Header Digest using CRC32C
    pub const HEADER_DIGEST_CRC32C: Self = Self(0x0002);

    /// Data Digest using CRC32C  
    pub const DATA_DIGEST_CRC32C: Self = Self(0x0008);

    /// Authentication via CHAP_UNI
    pub const AUTH_CHAP_UNI: Self = Self(0x0800);

    /// Authentication via CHAP_BI
    pub const AUTH_CHAP_BI: Self = Self(0x1000);

    /// Check if a specific option is set
    #[must_use]
    pub const fn has(self, option: Self) -> bool {
        (self.0 & option.0) != 0
    }

    /// Set a specific option
    #[must_use]
    pub const fn with(self, option: Self) -> Self {
        Self(self.0 | option.0)
    }

    /// Remove a specific option
    #[must_use]
    pub const fn without(self, option: Self) -> Self {
        Self(self.0 & !option.0)
    }
}

/// iSCSI Device Path (SubType 0x13)
///
/// According to UEFI 2.11 spec section 10.3.4.17:
/// - Length: 18+n bytes (variable length due to target name)
/// - Protocol: 2 bytes (0x0000 = TCP)
/// - Login Options: 2 bytes (bitfield of login options)
/// - LUN: 8 bytes (Logical Unit Number)
/// - Portal Group Tag: 2 bytes (Target Portal Group Tag)
/// - Target Name: variable length iSCSI name string
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IScsi {
    /// Protocol (currently only TCP)
    pub protocol: Protocol,

    /// Login Options bitfield
    pub options: LoginOptions,

    /// Logical Unit Number (8 bytes)
    pub lun: [u8; 8],

    /// Target Portal Group Tag
    pub tag: u16,

    /// iSCSI Target Name
    pub name: String,
}

impl<'a> TryFrom<Head<'a>> for IScsi {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            protocol: node.data.parse(ByteOrder::Little)?,
            options: LoginOptions(node.data.parse(ByteOrder::Little)?),
            lun: node.data.parse(())?,
            tag: node.data.parse(ByteOrder::Little)?,
            name: node.data.finish(Format::Utf16(Some(node.data.len())))?,
        })
    }
}
