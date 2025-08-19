//! REST Service Device Path
//!
//! This module implements the REST Service device path node as defined in UEFI 2.11 specification
//! section 10.3.4.31. This device path describes a REST service endpoint.

use crate::parser::{Invalid, Parser};
use crate::{Error, Head};

/// REST Service Type as defined in UEFI 2.11 spec
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Service {
    /// Redfish Service
    Redfish,

    /// OData Service
    OData,

    /// Vendor-specific service
    VendorSpecific {
        /// Service type GUID
        guid: [u8; 16],

        /// Vendor-specific data
        data: Vec<u8>,
    },
}

/// REST Service Access Mode as defined in UEFI 2.11 spec
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessMode {
    /// In-band access
    InBand,

    /// Out-of-band access
    OutOfBand,
}

impl Parser<AccessMode> for &[u8] {
    type Arg = ();

    fn parse(&mut self, arg: Self::Arg) -> Result<AccessMode, Invalid> {
        match self.parse(arg)? {
            0x01u8 => Ok(AccessMode::InBand),
            0x02u8 => Ok(AccessMode::OutOfBand),
            _ => Err(Invalid),
        }
    }
}

/// REST Service Device Path (SubType 0x21)
///
/// According to UEFI 2.11 spec section 10.3.4.31:
/// - Length: 6+ bytes (minimum 6 bytes, may include vendor-specific data)
/// - REST Service Type: 1 byte (Redfish, OData, or Vendor-specific)
/// - Access Mode: 1 byte (In-band or Out-of-band)
/// - Vendor Data: variable length (present only for vendor-specific services)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rest {
    /// REST Service Type
    pub service: Service,

    /// Access Mode
    pub mode: AccessMode,
}

impl TryFrom<Head<'_>> for Rest {
    type Error = Error;

    fn try_from(mut node: Head<'_>) -> Result<Self, Self::Error> {
        let service_type = node.data.parse(())?;
        let access_mode = node.data.parse(())?;

        Ok(Self {
            service: match service_type {
                0x01u8 => Service::Redfish,
                0x02u8 => Service::OData,
                0x03u8 => {
                    let guid = node.data.parse(())?;
                    let data = node.data.finish(node.data.len())?;
                    Service::VendorSpecific { guid, data }
                }

                _ => return Err(Error::Invalid),
            },
            mode: access_mode,
        })
    }
}
