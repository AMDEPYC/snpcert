//! IPv4 Device Path
//!
//! This module implements the IPv4 device path node as defined in UEFI 2.11 specification
//! section 10.3.4.12. This device path describes an IPv4 network connection.

use core::net::{Ipv4Addr, SocketAddrV4};

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

/// IP Protocol numbers as defined in RFC 3232 (IANA assignments)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Protocol(pub u16);

impl Protocol {
    /// Internet Control Message Protocol
    pub const ICMP: Self = Self(1);

    /// Internet Group Management Protocol
    pub const IGMP: Self = Self(2);

    /// Transmission Control Protocol
    pub const TCP: Self = Self(6);

    /// User Datagram Protocol
    pub const UDP: Self = Self(17);

    /// IPv6 encapsulation
    pub const IPV6: Self = Self(41);

    /// Routing Header for IPv6
    pub const IPV6_ROUTE: Self = Self(43);

    /// Fragment Header for IPv6
    pub const IPV6_FRAG: Self = Self(44);

    /// Generic Routing Encapsulation
    pub const GRE: Self = Self(47);

    /// Encapsulating Security Payload
    pub const ESP: Self = Self(50);

    /// Authentication Header
    pub const AH: Self = Self(51);

    /// ICMP for IPv6
    pub const IPV6_ICMP: Self = Self(58);

    /// No Next Header for IPv6
    pub const IPV6_NO_NXT: Self = Self(59);

    /// Destination Options for IPv6
    pub const IPV6_OPTS: Self = Self(60);

    /// Stream Control Transmission Protocol
    pub const SCTP: Self = Self(132);
}

/// IPv4 Device Path (SubType 0x0C)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv4 {
    /// Local socket address
    pub local: SocketAddrV4,
    /// Remote socket address
    pub remote: SocketAddrV4,
    /// IP protocol number
    pub protocol: Protocol,
    /// Whether to use static IP configuration
    pub static_ip: bool,
    /// Gateway IP address
    pub gateway_ip: Ipv4Addr,
    /// Subnet mask
    pub subnet_mask: Ipv4Addr,
}

impl<'a> TryFrom<Head<'a>> for Ipv4 {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        let local_ip: Ipv4Addr = From::<[u8; 4]>::from(node.data.parse(())?);
        let remote_ip: Ipv4Addr = From::<[u8; 4]>::from(node.data.parse(())?);
        let local = SocketAddrV4::new(local_ip, node.data.parse(ByteOrder::Little)?);
        let remote = SocketAddrV4::new(remote_ip, node.data.parse(ByteOrder::Little)?);
        let protocol = Protocol(node.data.parse(ByteOrder::Little)?);
        let static_ip = node.data.parse(())?;
        let gateway_ip: Ipv4Addr = From::<[u8; 4]>::from(node.data.parse(())?);
        let subnet_mask: Ipv4Addr = From::<[u8; 4]>::from(node.data.finish(())?);

        Ok(Self {
            local,
            remote,
            protocol,
            static_ip,
            gateway_ip,
            subnet_mask,
        })
    }
}
