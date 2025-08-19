//! URI Device Path
//!
//! This module implements the URI device path node as defined in UEFI 2.11 specification
//! section 10.3.4.22. This device path describes a URI device path.

use std::ops::{Deref, DerefMut};

use crate::parser::{Format, Parser};
use crate::{Error, Head};

/// URI Device Path (SubType 0x18)
///
/// According to UEFI 2.11 spec section 10.3.4.22:
/// - Length: 4+n bytes (variable length due to URI string)
/// - URI: variable length UTF-8 string containing the URI (NOT null-terminated)
/// - For an empty URI, Length is 4 bytes
///
/// RFC 3986 URIs are always valid UTF-8 strings.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Uri(pub String);

impl Deref for Uri {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Uri {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<Head<'_>> for Uri {
    type Error = Error;

    fn try_from(mut node: Head<'_>) -> Result<Self, Self::Error> {
        Ok(Self(node.data.finish(Format::Utf8(Some(node.data.len())))?))
    }
}
