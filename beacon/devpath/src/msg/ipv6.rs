//! IPv6 Device Path
//!
//! This module implements the IPv6 device path node as defined in UEFI 2.11 specification
//! section 10.3.4.13. This device path describes an IPv6 network connection.

use super::ipv4::Protocol;

use crate::parser::{ByteOrder, Invalid, Parser};
use crate::{Error, Head};

use core::net::{Ipv6Addr, SocketAddrV6};

/// IPv6 address origin as defined in UEFI specification section 10.3.4.12
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Origin {
    /// The Local IP Address was manually configured
    Manual = 0x00,

    /// The Local IP Address is assigned through IPv6 stateless auto-configuration
    StatelessAutoConfiguration = 0x01,

    /// The Local IP Address is assigned through IPv6 stateful configuration
    StatefulConfiguration = 0x02,
}

impl Parser<Origin> for &[u8] {
    type Arg = ();

    fn parse(&mut self, arg: ()) -> Result<Origin, Invalid> {
        match self.parse(arg)? {
            0x00u8 => Ok(Origin::Manual),
            0x01 => Ok(Origin::StatelessAutoConfiguration),
            0x02 => Ok(Origin::StatefulConfiguration),
            _ => Err(Invalid),
        }
    }
}

/// IPv6 Device Path (SubType 0x0D)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv6 {
    /// Local socket address
    pub local: SocketAddrV6,

    /// Remote socket address
    pub remote: SocketAddrV6,

    /// IP protocol number
    pub protocol: Protocol,

    /// IPv6 address origin (manual, stateless auto-config, stateful config)
    pub origin: Origin,

    /// Subnet prefix length
    pub prefix_length: u8,

    /// Gateway IP address
    pub gateway_ip: Ipv6Addr,
}

impl<'a> TryFrom<Head<'a>> for Ipv6 {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        let local_ip_bytes: [u8; 16] = node.data.parse(())?;
        let local_ip = Ipv6Addr::from(local_ip_bytes);

        let remote_ip_bytes: [u8; 16] = node.data.parse(())?;
        let remote_ip = Ipv6Addr::from(remote_ip_bytes);

        let local_port = node.data.parse(ByteOrder::Little)?;
        let remote_port = node.data.parse(ByteOrder::Little)?;
        let protocol = Protocol(node.data.parse(ByteOrder::Little)?);
        let ip_address_origin = node.data.parse(())?;
        let prefix_length = node.data.parse(())?;

        let gateway_ip_bytes: [u8; 16] = node.data.finish(())?;
        let gateway_ip = Ipv6Addr::from(gateway_ip_bytes);

        Ok(Self {
            local: SocketAddrV6::new(local_ip, local_port, 0, 0),
            remote: SocketAddrV6::new(remote_ip, remote_port, 0, 0),
            protocol,
            origin: ip_address_origin,
            prefix_length,
            gateway_ip,
        })
    }
}
