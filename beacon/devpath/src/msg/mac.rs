//! MAC Address Device Path
//!
//! This module implements the MAC Address device path node as defined in UEFI 2.11 specification
//! section 10.3.4.11. This device path describes a network interface by MAC address.

use crate::parser::Parser;
use crate::{Error, Head};

/// Network interface type as defined in RFC 3232 (referenced by UEFI specification section 10.3.4.10)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InterfaceType(pub u8);

impl InterfaceType {
    /// None of the following
    pub const OTHER: Self = Self(1);

    /// Ethernet-like Objects (most common)
    pub const ETHERNET_CSMACD: Self = Self(6);

    /// Token Ring-like Objects
    pub const IEEE802_5: Self = Self(9);

    /// FDDI Objects
    pub const FDDI: Self = Self(15);

    /// Point-to-Point Protocol
    pub const PPP: Self = Self(23);

    /// Software Loopback
    pub const SOFTWARE_LOOPBACK: Self = Self(24);

    /// ATM
    pub const ATM: Self = Self(37);

    /// IEEE 802.11 radio spread spectrum (WiFi)
    pub const IEEE802_11: Self = Self(71);

    /// Tunnel encapsulation interface
    pub const TUNNEL: Self = Self(131);

    /// IEEE 802.3ad Link Aggregate
    pub const IEEE8023AD_LAG: Self = Self(161);
}

/// MAC Address Device Path (SubType 0x0B)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MacAddress {
    /// MAC Address (32 bytes, padded with zeros if shorter)
    pub mac: [u8; 32],

    /// Interface Type
    pub if_type: InterfaceType,
}

impl<'a> TryFrom<Head<'a>> for MacAddress {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            mac: node.data.parse(())?,
            if_type: InterfaceType(node.data.finish(())?),
        })
    }
}
