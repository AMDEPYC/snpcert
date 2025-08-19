use std::ops::{Deref, DerefMut};

use crate::{Error, FromBytes, Head, Node, Path};

/// A collection of UEFI device paths.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Paths(Vec<Path>);

impl Deref for Paths {
    type Target = Vec<Path>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Paths {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<Vec<Node>>> for Paths {
    fn from(paths: Vec<Vec<Node>>) -> Self {
        Self(paths.into_iter().map(Path::from).collect())
    }
}

impl From<Vec<Path>> for Paths {
    #[inline]
    fn from(paths: Vec<Path>) -> Self {
        Self(paths)
    }
}

impl From<Path> for Paths {
    #[inline]
    fn from(path: Path) -> Self {
        Self(vec![path])
    }
}

impl<'de> FromBytes<'de> for Paths {
    type Error = Error;

    fn from_bytes(mut bytes: &'de [u8]) -> Result<Self, Self::Error> {
        let mut paths = Self(Vec::new());

        'paths: loop {
            paths.push(Path::default());

            'path: loop {
                let head = Head::from_bytes(bytes)?;
                bytes = &bytes[head.data.len() + 4..];

                match head {
                    Head::END_ONE => break 'path,
                    Head::END_ALL => break 'paths,
                    _ => paths.last_mut().unwrap().push(head.try_into()?),
                }
            }
        }

        Ok(paths)
    }
}
