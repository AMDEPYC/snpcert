use std::ops::{Deref, DerefMut};

use crate::{Error, FromBytes, Head, Node};

/// A UEFI device path, represented as a sequence of nodes.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Path(Vec<Node>);

impl Deref for Path {
    type Target = Vec<Node>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Path {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<Node>> for Path {
    fn from(nodes: Vec<Node>) -> Self {
        Self(nodes)
    }
}

impl<'de> FromBytes<'de> for Path {
    type Error = Error;

    fn from_bytes(mut bytes: &'de [u8]) -> Result<Self, Self::Error> {
        let mut path = Self(Vec::new());

        loop {
            let head = Head::from_bytes(bytes)?;

            // We found a terminator.
            if let Head::END_ONE | Head::END_ALL = head {
                return Ok(path);
            }

            bytes = &bytes[head.data.len() + 4..];
            path.0.push(head.try_into()?);
        }
    }
}
