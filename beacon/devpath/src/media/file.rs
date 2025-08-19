//! File Path Media Device Path
//!
//! This module implements the File Path media device path node as defined in UEFI 2.11 specification
//! section 10.3.3.4. This device path describes a file using a file path.

use crate::parser::{Format, Parser};
use crate::{Error, Head};
use alloc::string::String;

/// File Path Media Device Path (SubType 0x04)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilePath(String);

impl<'a> TryFrom<Head<'a>> for FilePath {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        Ok(Self(node.data.finish(Format::Utf16(None))?))
    }
}
