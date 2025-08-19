//! UEFI Device Path parsing library
//!
//! This library provides parsing and manipulation of UEFI device paths as defined
//! in the UEFI 2.11 specification.

#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![warn(missing_docs)]
#![deny(unused_must_use)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::print_stdout)]
#![warn(clippy::print_stderr)]
#![allow(clippy::doc_markdown)]

extern crate alloc;

pub mod acpi;
pub mod bios;
pub mod hw;
pub mod media;
pub mod msg;

mod error;
mod node;
mod parser;
mod path;
mod paths;

pub use error::Error;
pub use node::Node;
pub use path::Path;
pub use paths::Paths;

/// Trait for parsing structures from raw byte data.
///
/// This trait provides a standardized interface for converting byte slices
/// into structured data types, commonly used for parsing UEFI data structures.
pub trait FromBytes<'de>: Sized {
    /// The error type returned when parsing fails.
    type Error;

    /// Parse a structure from raw bytes.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The byte slice to parse from
    ///
    /// # Returns
    ///
    /// Returns the parsed structure on success.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte data is invalid, incomplete, or cannot be parsed
    /// into the target structure. This can occur when:
    /// - The input is too short for the expected structure
    /// - The data format is invalid or corrupted
    /// - Required fields contain invalid values
    fn from_bytes(bytes: &'de [u8]) -> Result<Self, Self::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Head<'a> {
    kind: u8,
    subkind: u8,
    data: &'a [u8],
}

impl Head<'_> {
    const END_ONE: Self = Self {
        kind: 0x7f,
        subkind: 0x01,
        data: &[],
    };

    const END_ALL: Self = Self {
        kind: 0x7f,
        subkind: 0xff,
        data: &[],
    };
}

impl<'a> FromBytes<'a> for Head<'a> {
    type Error = Error;

    fn from_bytes(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 4 {
            return Err(Error::Invalid);
        }

        let length = u16::from_le_bytes([bytes[2], bytes[3]]) as usize;
        if length < 4 {
            return Err(Error::Invalid);
        }

        match bytes.split_at_checked(length) {
            None => Err(Error::Invalid),
            Some((node, ..)) => Ok(Head {
                kind: node[0],
                subkind: node[1],
                data: &node[4..],
            }),
        }
    }
}
