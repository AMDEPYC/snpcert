use crate::error::Type;
use crate::{Error, Head};

/// Device Path types as defined in UEFI specification
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    /// Hardware device path
    Hardware(crate::hw::Hardware),

    /// ACPI device path
    Acpi(crate::acpi::Acpi),

    /// Messaging device path
    Messaging(crate::msg::Messaging),

    /// Media device path
    Media(crate::media::Media),

    /// BIOS device path
    Bios(crate::bios::Bios),
}

impl TryFrom<Head<'_>> for Node {
    type Error = Error;

    fn try_from(head: Head<'_>) -> Result<Self, Self::Error> {
        match Type::decode(head.kind) {
            Ok(Type::Hardware) => TryFrom::try_from(head).map(Node::Hardware),
            Ok(Type::Acpi) => TryFrom::try_from(head).map(Node::Acpi),
            Ok(Type::Messaging) => TryFrom::try_from(head).map(Node::Messaging),
            Ok(Type::Media) => TryFrom::try_from(head).map(Node::Media),
            Ok(Type::Bios) => TryFrom::try_from(head).map(Node::Bios),
            Err(n) => Err(Error::UnknownType(n)),
        }
    }
}
