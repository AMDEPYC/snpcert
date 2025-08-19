//! GUID Media Device Path
//!
//! This module implements the GUID media device path node types as defined in UEFI 2.11 specification
//! section 10.3.3.5, 10.3.3.6, and 10.3.3.7. These device paths describe media using GUID identifiers.

use crate::parser::Parser;
use crate::{Error, Head};

/// A GUID type used throughout UEFI device paths
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Guid(pub [u8; 16]);

impl TryFrom<Head<'_>> for Guid {
    type Error = Error;

    fn try_from(mut node: Head<'_>) -> Result<Self, Self::Error> {
        Ok(Self(node.data.finish(())?))
    }
}
