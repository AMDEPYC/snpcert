//! Relative Offset Range Media Device Path
//!
//! This module implements the Relative Offset Range media device path node as defined in UEFI 2.11 specification
//! section 10.3.3.8. This device path describes a range of media using relative offset ranges.

use crate::parser::{ByteOrder, Parser};
use crate::{Error, Head};

/// Relative Offset Range (SubType 0x08)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RelativeRange {
    /// Starting offset
    pub start: u64,

    /// Ending offset
    pub end: u64,
}

impl<'a> TryFrom<Head<'a>> for RelativeRange {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            start: node.data.parse(ByteOrder::Little)?,
            end: node.data.finish(ByteOrder::Little)?,
        })
    }
}
