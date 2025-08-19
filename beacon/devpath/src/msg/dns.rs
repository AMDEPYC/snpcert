//! DNS Device Path
//!
//! This module implements the DNS device path node as defined in UEFI 2.11 specification
//! section 10.3.4.27. This device path describes DNS server configuration.

use core::net::{Ipv4Addr, Ipv6Addr};

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};
use alloc::vec::Vec;

/// DNS Server Address Lists
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Servers {
    /// IPv4 server addresses
    Ipv4(Vec<Ipv4Addr>),

    /// IPv6 server addresses
    Ipv6(Vec<Ipv6Addr>),
}

/// DNS Device Path (SubType 0x1F)
///
/// According to UEFI 2.11 spec section 10.3.4.29:
/// - Length: 5+n bytes (variable length due to DNS server addresses)
/// - IsIPv6: 1 byte (0x00 = IPv4, 0x01 = IPv6)
/// - DNS Server Addresses: variable length list of IP addresses
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dns {
    /// DNS server addresses
    pub servers: Servers,
}

impl<'a> TryFrom<Head<'a>> for Dns {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            servers: match node.data.parse(())? {
                0u8 => {
                    let mut servers = Vec::new();
                    while !node.data.is_empty() {
                        servers.push(Ipv4Addr::from_bits(node.data.parse(ByteOrder::Big)?));
                    }

                    Servers::Ipv4(servers)
                }

                1u8 => {
                    let mut servers = Vec::new();
                    while !node.data.is_empty() {
                        servers.push(Ipv6Addr::from_bits(node.data.parse(ByteOrder::Big)?));
                    }

                    Servers::Ipv6(servers)
                }

                _ => return Err(Error::Invalid),
            },
        })
    }
}
