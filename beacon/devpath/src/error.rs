use crate::parser::Invalid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Hardware,
    Acpi,
    Messaging,
    Media,
    Bios,
}

impl Type {
    pub(crate) const fn decode(value: u8) -> Result<Self, u8> {
        match value {
            0x01 => Ok(Self::Hardware),
            0x02 => Ok(Self::Acpi),
            0x03 => Ok(Self::Messaging),
            0x04 => Ok(Self::Media),
            0x05 => Ok(Self::Bios),
            n => Err(n),
        }
    }
}

/// Errors that can occur when parsing device paths
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Unknown sub-type for the given device path type
    UnknownSubType(Type, u8),

    /// Unknown device path type
    UnknownType(u8),

    /// Invalid data format
    Invalid,
}

impl From<Invalid> for Error {
    fn from(_: Invalid) -> Self {
        Self::Invalid
    }
}

impl core::error::Error for Error {}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownSubType(t, st) => write!(f, "unknown {t:?} sub-type: {st}"),
            Self::UnknownType(t) => write!(f, "unknown device path type: {t}"),
            Self::Invalid => write!(f, "invalid data format"),
        }
    }
}
