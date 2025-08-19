use core::ops::{BitAnd, BitOr, BitXor, Not};
use devpath::{Error, FromBytes};

/// EFI variable with attributes and data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EfiVar<'a> {
    /// Variable attributes
    pub attributes: Attributes,

    /// Variable data
    pub data: &'a [u8],
}

impl<'a> FromBytes<'a> for EfiVar<'a> {
    type Error = Error;

    /// Parse an EFI variable from a byte slice
    ///
    /// Parsing is actually infallible if the size of bytes is >=4.
    fn from_bytes(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        // Parse the attributes.
        let (head, tail) = bytes.split_at_checked(4).ok_or(Error::Invalid)?;
        let head: &[u8; 4] = head.try_into().map_err(|_| Error::Invalid)?;
        let attr = u32::from_le_bytes(*head);

        Ok(EfiVar {
            attributes: Attributes(attr),
            data: tail,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Attributes(u32);

impl Attributes {
    /// Variable is stored in non-volatile storage
    pub const NON_VOLATILE: Self = Self(1 << 0);

    /// Variable is accessible during boot services
    pub const BOOTSERVICE_ACCESS: Self = Self(1 << 1);

    /// Variable is accessible during runtime services
    pub const RUNTIME_ACCESS: Self = Self(1 << 2);

    /// Variable contains a hardware error record
    pub const HARDWARE_ERROR_RECORD: Self = Self(1 << 3);

    /// Variable is authenticated write access
    pub const AUTHENTICATED_WRITE_ACCESS: Self = Self(1 << 4);

    /// Variable is time-based authenticated write access
    pub const TIME_BASED_AUTHENTICATED_WRITE_ACCESS: Self = Self(1 << 5);

    /// Variable payload is append write
    pub const APPEND_WRITE: Self = Self(1 << 6);

    /// Variable is enhanced authenticated access
    pub const ENHANCED_AUTHENTICATED_ACCESS: Self = Self(1 << 7);

    /// Check if a specific attribute flag is set
    pub fn has(self, flag: Self) -> bool {
        (self & flag).0 != 0
    }
}

impl core::fmt::Display for Attributes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:08b}", self.0 & 0xFF)
    }
}

impl BitAnd for Attributes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for Attributes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitXor for Attributes {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl Not for Attributes {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl From<u32> for Attributes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Attributes> for u32 {
    fn from(attr: Attributes) -> Self {
        attr.0
    }
}
