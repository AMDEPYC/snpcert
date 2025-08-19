//! Messaging device path types and parsing
//!
//! This module contains all messaging device path sub-types defined in UEFI 2.11.
//! Messaging device paths describe communication interfaces and protocols.

pub mod atapi;
pub mod bt;
pub mod btle;
pub mod dlu;
pub mod dns;
pub mod emmc;
pub mod fc;
pub mod i2o;
pub mod ib;
pub mod ieee1394;
pub mod ipv4;
pub mod ipv6;
pub mod iscsi;
pub mod mac;
pub mod nvdimm;
pub mod nvme;
pub mod nvme_of;
pub mod rest;
pub mod sasex;
pub mod sata;
pub mod scsi;
pub mod sd;
pub mod uart;
pub mod ufs;
pub mod uri;
pub mod usb;
pub mod vendor;
pub mod vlan;
pub mod wifi;

use crate::error::Type;

use crate::{Error, Head};

/// UEFI Messaging Device Path Types
///
/// This enum represents all messaging device path sub-types defined in UEFI 2.11 specification.
/// Each variant corresponds to a specific messaging protocol or device interface.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Messaging {
    /// ATAPI Device Path [0x01] - IDE/ATAPI controller device
    Atapi(atapi::Atapi),

    /// SCSI Device Path [0x02] - SCSI target device
    Scsi(scsi::Scsi),

    /// Fibre Channel Device Path [0x03] - Fibre Channel target device
    FibreChannel(fc::FibreChannel),

    /// IEEE 1394 Device Path [0x04] - FireWire device
    Ieee1394(ieee1394::Ieee1394),

    /// USB Device Path [0x05] - USB device on specific port/interface
    Usb(usb::Usb),

    /// I2O Device Path [0x06] - I2O Random Block Storage Class device
    I2O(i2o::I2o),

    /// InfiniBand Device Path [0x09] - InfiniBand fabric device
    InfiniBand(ib::InfiniBand),

    /// Vendor-Defined Messaging Device Path [0x0A] - Vendor-specific messaging protocol
    Vendor(vendor::Vendor),

    /// MAC Address Device Path [0x0B] - Network interface by MAC address
    MacAddress(mac::MacAddress),

    /// IPv4 Device Path [0x0C] - IPv4 network connection
    Ipv4(ipv4::Ipv4),

    /// IPv6 Device Path [0x0D] - IPv6 network connection  
    Ipv6(ipv6::Ipv6),

    /// UART Device Path [0x0E] - Serial UART device
    Uart(uart::Uart),

    /// USB Class Device Path [0x0F] - USB device by class/vendor/product
    UsbClass(usb::UsbClass),

    /// USB WWID Device Path [0x10] - USB device by World Wide ID
    UsbWwid(usb::UsbWwid),

    /// Device Logical Unit [0x11] - Logical unit number for multi-LUN devices
    DeviceLogicalUnit(dlu::DeviceLogicalUnit),

    /// SATA Device Path [0x12] - Serial ATA device
    Sata(sata::Sata),

    /// iSCSI Device Path [0x13] - iSCSI target device
    IScsi(iscsi::IScsi),

    /// VLAN Device Path [0x14] - 802.1Q VLAN interface
    Vlan(vlan::Vlan),

    /// Fibre Channel Ex Device Path [0x15] - Extended Fibre Channel target (byte arrays)
    FibreChannelEx(fc::FibreChannelEx),

    /// SAS Extended Device Path [0x16] - Serial Attached SCSI Extended
    SasExtended(sasex::SasExtended),

    /// NVMe Namespace Device Path [0x17] - NVMe namespace
    NvmeNamespace(nvme::NvmeNamespace),

    /// URI Device Path [0x18] - URI Device Path
    Uri(uri::Uri),

    /// UFS Device Path [0x19] - Universal Flash Storage
    Ufs(ufs::Ufs),

    /// SD Device Path [0x1A] - Secure Digital
    Sd(sd::SecureDigital),

    /// Bluetooth Device Path [0x1B] - Bluetooth device
    Bluetooth(bt::Bluetooth),

    /// Wi-Fi Device Path [0x1C] - Wi-Fi device
    Wifi(wifi::Wifi),

    /// eMMC Device Path [0x1D] - Embedded MultiMediaCard
    EMmc(emmc::EMmc),

    /// BluetoothLE Device Path [0x1E] - Bluetooth Low Energy
    BluetoothLe(btle::BluetoothLe),

    /// DNS Device Path [0x1F] - DNS device
    Dns(dns::Dns),

    /// NVDIMM Namespace Device Path [0x20] - NVDIMM Namespace
    NvdimmNamespace(nvdimm::NvdimmNamespace),

    /// REST Service Device Path [0x21] - REST service endpoint
    RestService(rest::Rest),

    /// NVMe-oF Namespace Device Path [0x22] - NVMe over Fabric Namespace
    NvmeOfNamespace(nvme_of::NvmeOfNamespace),
}

impl TryFrom<Head<'_>> for Messaging {
    type Error = Error;

    fn try_from(head: Head<'_>) -> Result<Self, Self::Error> {
        match head.subkind {
            0x01 => TryFrom::try_from(head).map(Messaging::Atapi),
            0x02 => TryFrom::try_from(head).map(Messaging::Scsi),
            0x03 => TryFrom::try_from(head).map(Messaging::FibreChannel),
            0x04 => TryFrom::try_from(head).map(Messaging::Ieee1394),
            0x05 => TryFrom::try_from(head).map(Messaging::Usb),
            0x06 => TryFrom::try_from(head).map(Messaging::I2O),
            0x09 => TryFrom::try_from(head).map(Messaging::InfiniBand),
            0x0A => TryFrom::try_from(head).map(Messaging::Vendor),
            0x0B => TryFrom::try_from(head).map(Messaging::MacAddress),
            0x0C => TryFrom::try_from(head).map(Messaging::Ipv4),
            0x0D => TryFrom::try_from(head).map(Messaging::Ipv6),
            0x0E => TryFrom::try_from(head).map(Messaging::Uart),
            0x0F => TryFrom::try_from(head).map(Messaging::UsbClass),
            0x10 => TryFrom::try_from(head).map(Messaging::UsbWwid),
            0x11 => TryFrom::try_from(head).map(Messaging::DeviceLogicalUnit),
            0x12 => TryFrom::try_from(head).map(Messaging::Sata),
            0x13 => TryFrom::try_from(head).map(Messaging::IScsi),
            0x14 => TryFrom::try_from(head).map(Messaging::Vlan),
            0x15 => TryFrom::try_from(head).map(Messaging::FibreChannelEx),
            0x16 => TryFrom::try_from(head).map(Messaging::SasExtended),
            0x17 => TryFrom::try_from(head).map(Messaging::NvmeNamespace),
            0x18 => TryFrom::try_from(head).map(Messaging::Uri),
            0x19 => TryFrom::try_from(head).map(Messaging::Ufs),
            0x1A => TryFrom::try_from(head).map(Messaging::Sd),
            0x1B => TryFrom::try_from(head).map(Messaging::Bluetooth),
            0x1C => TryFrom::try_from(head).map(Messaging::Wifi),
            0x1D => TryFrom::try_from(head).map(Messaging::EMmc),
            0x1E => TryFrom::try_from(head).map(Messaging::BluetoothLe),
            0x1F => TryFrom::try_from(head).map(Messaging::Dns),
            0x20 => TryFrom::try_from(head).map(Messaging::NvdimmNamespace),
            0x21 => TryFrom::try_from(head).map(Messaging::RestService),
            0x22 => TryFrom::try_from(head).map(Messaging::NvmeOfNamespace),
            n => Err(Error::UnknownSubType(Type::Messaging, n)),
        }
    }
}
