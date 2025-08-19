//! NVMe over Fabrics Namespace Device Path
//!
//! This module implements the NVMe-oF Namespace device path node as defined in UEFI 2.11 specification
//! section 10.3.4.32. This device path describes an NVMe over Fabric namespace.

use crate::parser::{Format, Invalid, Parser};
use crate::{Error, Head};

/// NVMe Namespace Identifier combining both type and data
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NamespaceId {
    /// IEEE Extended Unique Identifier (EUI-64)
    Eui64([u8; 8]),

    /// Namespace Globally Unique Identifier (NGUID)
    Nguid([u8; 16]),

    /// Namespace UUID (NUUID)
    Uuid([u8; 16]),

    /// Command Set Identifier (CSI)
    Csi(u8),
}

impl Parser<NamespaceId> for &[u8] {
    type Arg = ();

    fn parse(&mut self, _arg: Self::Arg) -> Result<NamespaceId, Invalid> {
        let nidt = self.parse(())?;
        let nid: [u8; 16] = self.parse(())?;

        match nidt {
            0x01u8 => {
                let mut eui64 = [0u8; 8];
                eui64.copy_from_slice(&nid[..8]);
                Ok(NamespaceId::Eui64(eui64))
            }
            0x02u8 => Ok(NamespaceId::Nguid(nid)),
            0x03u8 => Ok(NamespaceId::Uuid(nid)),
            0x04u8 => Ok(NamespaceId::Csi(nid[0])),
            _ => Err(Invalid),
        }
    }
}

/// NVMe-oF Namespace Device Path (SubType 0x22)
///
/// According to UEFI 2.11 spec section 10.3.4.32:
/// - Length: 20+n bytes (variable length due to subsystem NQN)
/// - NIDT: 1 byte (Namespace Identifier Type)
/// - NID: 16 bytes (Namespace Identifier - note: actual length varies by type)
/// - Subsystem NQN: variable length UTF-8 string
///
/// Note: In the full NVMe spec, NID lengths vary (EUI-64: 8 bytes, NGUID/UUID: 16 bytes, CSI: 1 byte),
/// but UEFI device paths use a fixed 16-byte field for all types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NvmeOfNamespace {
    /// Namespace Identifier (combines both type and data)
    pub nid: NamespaceId,

    /// Subsystem NVMe Qualified Name (UTF-8 string)
    pub nqn: String,
}

impl TryFrom<Head<'_>> for NvmeOfNamespace {
    type Error = Error;

    fn try_from(mut node: Head<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            nid: node.data.parse(())?,
            nqn: node.data.finish(Format::Utf8(None))?,
        })
    }
}
