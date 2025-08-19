//! Controller Device Path
//!
//! This module implements the Controller device path node as defined in UEFI 2.11 specification
//! section 10.3.1.5. This device path describes a controller device and identifies which
//! controller out of a set of controllers that a device is connected to.

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

/// Controller Device Path (SubType 0x05)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Controller(pub u32);

impl<'a> TryFrom<Head<'a>> for Controller {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self(node.data.finish(ByteOrder::Little)?))
    }
}
