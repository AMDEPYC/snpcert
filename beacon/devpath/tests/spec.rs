use devpath::*;

extern crate std;

fn decode(groups: &[&[u8]]) -> Result<Paths, Error> {
    let mut bytes = Vec::new();

    for group in groups {
        bytes.extend_from_slice(group);
    }

    Paths::from_bytes(&bytes)
}

// ACPI Root (PNP0A03) device path
// Source: UEFI 2.11 spec section 10.3.3 ACPI Device Path - basic PCI root bridge example
// Path: PciRoot(0) - represents PNP0A03 with UID 0
//
// HEX BREAKDOWN ACCORDING TO UEFI 2.11 SPEC:
// ACPI Root: 02 01 0c 00 d0 41 03 0a 00 00 00 00 (12 bytes)
//   Type=02 (ACPI), SubType=01 (Standard), Length=12, HID=0x0a0341d0, UID=0
// End: 7f ff 04 00 (4 bytes)
//   Type=7f (End), SubType=ff (End Entire), Length=4
#[test]
fn acpi_root() {
    let results = decode(&[
        &[0x02, 0x01, 0x0c, 0x00], // Node: ACPI: Type=02, SubType=01, Length=12
        &[0xd0, 0x41, 0x03, 0x0a], // HID: 0x0a0341d0 (little-endian)
        &[0x00, 0x00, 0x00, 0x00], // UID: 0 (little-endian)
        &[0x7f, 0xff, 0x04, 0x00], // Node: Terminator: Type=7f, SubType=ff, Length=4
    ])
    .unwrap();

    let expected = Path::from(vec![Node::Acpi(acpi::Acpi::Standard(
        acpi::standard::Standard {
            hid: 0x0a03_41d0,
            uid: 0,
        },
    ))]);

    assert_eq!(results, expected.into());
}

// PCI Root + PCI Device (function 0, device 1)
// Source: UEFI 2.11 spec section 10.3.2.1 PCI Device Path - basic PCI device example
// Path: PciRoot(0)/PCI(1,0) - PCI device 1, function 0 behind root bridge
//
// HEX BREAKDOWN ACCORDING TO UEFI 2.11 SPEC:
// ACPI Root: 02 01 0c 00 d0 41 03 0a 00 00 00 00 (12 bytes)
//   Type=02 (ACPI), SubType=01 (Standard), Length=12, HID=0x0a0341d0, UID=0
// PCI: 01 01 06 00 00 01 (6 bytes)
//   Type=01 (Hardware), SubType=01 (PCI), Length=6, Function=0, Device=1
// End: 7f ff 04 00 (4 bytes)
//   Type=7f (End), SubType=ff (End Entire), Length=4
#[test]
fn pci_device() {
    let results = decode(&[
        &[0x02, 0x01, 0x0c, 0x00], // Node: ACPI: Type=02, SubType=01, Length=12
        &[0xd0, 0x41, 0x03, 0x0a], // HID: 0x0a0341d0 (little-endian)
        &[0x00, 0x00, 0x00, 0x00], // UID: 0 (little-endian)
        &[0x01, 0x01, 0x06, 0x00], // Node: PCI: Type=01, SubType=01, Length=6
        &[0x00],                   // Function: 0
        &[0x01],                   // Device: 1
        &[0x7f, 0xff, 0x04, 0x00], // Node: Terminator: Type=7f, SubType=ff, Length=4
    ])
    .unwrap();

    let expected = Path::from(vec![
        Node::Acpi(acpi::Acpi::Standard(acpi::standard::Standard {
            hid: 0x0a03_41d0,
            uid: 0,
        })),
        Node::Hardware(hw::Hardware::Pci(hw::pci::Pci {
            device: 1,
            function: 0,
        })),
    ]);

    assert_eq!(results, expected.into());
}

// USB Hub Device Path with two USB nodes
// Source: UEFI 2.11 spec section 10.3.4.5.1, Table 10-13 "USB Device Path Example"
// Path: PciRoot(0)/PCI(31,2)/USB(1,0)/USB(3,0) - USB device on port 3 of hub on port 1
//
// HEX BREAKDOWN ACCORDING TO UEFI 2.11 SPEC:
// ACPI Root: 02 01 0c 00 d0 41 03 0a 00 00 00 00 (12 bytes)
//   Type=02 (ACPI), SubType=01 (Standard), Length=12, HID=0x0a0341d0, UID=0
// PCI: 01 01 06 00 02 1f (6 bytes)
//   Type=01 (Hardware), SubType=01 (PCI), Length=6, Function=2, Device=31
// USB Hub: 03 05 06 00 01 00 (6 bytes)
//   Type=03 (Messaging), SubType=05 (USB), Length=6, Port=1, Interface=0
// USB Device: 03 05 06 00 03 00 (6 bytes)
//   Type=03 (Messaging), SubType=05 (USB), Length=6, Port=3, Interface=0
// End: 7f ff 04 00 (4 bytes)
//   Type=7f (End), SubType=ff (End Entire), Length=4
#[test]
fn usb_nodes() {
    let results = decode(&[
        &[0x02, 0x01, 0x0c, 0x00], // Node: ACPI: Type=02, SubType=01, Length=12
        &[0xd0, 0x41, 0x03, 0x0a], // HID: 0x0a0341d0 (little-endian)
        &[0x00, 0x00, 0x00, 0x00], // UID: 0 (little-endian)
        &[0x01, 0x01, 0x06, 0x00], // Node: PCI: Type=01, SubType=01, Length=6
        &[0x02],                   // Function: 2
        &[0x1f],                   // Device: 31
        &[0x03, 0x05, 0x06, 0x00], // Node: USB: Type=03, SubType=05, Length=6
        &[0x01],                   // Port: 1
        &[0x00],                   // Interface: 0
        &[0x03, 0x05, 0x06, 0x00], // Node: USB: Type=03, SubType=05, Length=6
        &[0x03],                   // Port: 3
        &[0x00],                   // Interface: 0
        &[0x7f, 0xff, 0x04, 0x00], // Node: Terminator: Type=7f, SubType=ff, Length=4
    ])
    .unwrap();

    let expected = Path::from(vec![
        Node::Acpi(acpi::Acpi::Standard(acpi::standard::Standard {
            hid: 0x0a03_41d0,
            uid: 0,
        })),
        Node::Hardware(hw::Hardware::Pci(hw::pci::Pci {
            device: 31,
            function: 2,
        })),
        Node::Messaging(msg::Messaging::Usb(msg::usb::Usb {
            port: 1,
            interface: 0,
        })),
        Node::Messaging(msg::Messaging::Usb(msg::usb::Usb {
            port: 3,
            interface: 0,
        })),
    ]);

    assert_eq!(results, expected.into());
}

// USB Device Path (simplified single USB node)
// Source: UEFI 2.11 spec section 10.3.4.5 USB Device Path - basic USB device example
// Path: PciRoot(0)/PCI(31,2)/USB(0,0) - USB device on port 0
//
// HEX BREAKDOWN ACCORDING TO UEFI 2.11 SPEC:
// ACPI Root: 02 01 0c 00 d0 41 03 0a 00 00 00 00 (12 bytes)
//   Type=02 (ACPI), SubType=01 (Standard), Length=12, HID=0x0a0341d0, UID=0
// PCI: 01 01 06 00 02 1f (6 bytes)
//   Type=01 (Hardware), SubType=01 (PCI), Length=6, Function=2, Device=31
// USB: 03 05 06 00 00 00 (6 bytes)
//   Type=03 (Messaging), SubType=05 (USB), Length=6, Port=0, Interface=0
// End: 7f ff 04 00 (4 bytes)
//   Type=7f (End), SubType=ff (End Entire), Length=4
#[test]
fn usb_device() {
    let results = decode(&[
        &[0x02, 0x01, 0x0c, 0x00], // Node: ACPI: Type=02, SubType=01, Length=12
        &[0xd0, 0x41, 0x03, 0x0a], // HID: 0x0a0341d0 (little-endian)
        &[0x00, 0x00, 0x00, 0x00], // UID: 0 (little-endian)
        &[0x01, 0x01, 0x06, 0x00], // Node: PCI: Type=01, SubType=01, Length=6
        &[0x02],                   // Function: 2
        &[0x1f],                   // Device: 31
        &[0x03, 0x05, 0x06, 0x00], // Node: USB: Type=03, SubType=05, Length=6
        &[0x00],                   // Port: 0
        &[0x00],                   // Interface: 0
        &[0x7f, 0xff, 0x04, 0x00], // Node: Terminator: Type=7f, SubType=ff, Length=4
    ])
    .unwrap();

    let expected = Path::from(vec![
        Node::Acpi(acpi::Acpi::Standard(acpi::standard::Standard {
            hid: 0x0a03_41d0,
            uid: 0,
        })),
        Node::Hardware(hw::Hardware::Pci(hw::pci::Pci {
            function: 2,
            device: 31,
        })),
        Node::Messaging(msg::Messaging::Usb(msg::usb::Usb {
            port: 0,
            interface: 0,
        })),
    ]);

    assert_eq!(results, expected.into());
}

// Fibre Channel Extended Device Path
// Source: UEFI 2.11 spec section 10.3.4.3 "Fibre Channel Ex Device Path Example"
// Path: PciRoot(0)/PCI(31,0)/FibreEx(0x0001020304050607, 0x0001020304050607)
//
// HEX BREAKDOWN ACCORDING TO UEFI 2.11 SPEC:
// ACPI Root: 02 01 0c 00 d0 41 03 0a 00 00 00 00 (12 bytes)
//   Type=02 (ACPI), SubType=01 (Standard), Length=12, HID=0x0a0341d0, UID=0
// PCI: 01 01 06 00 00 1f (6 bytes)
//   Type=01 (Hardware), SubType=01 (PCI), Length=6, Function=0, Device=31
// Fibre Channel Ex: 03 15 18 00 00 00 00 01 02 03 04 05 06 07 00 01 02 03 04 05 06 07 00 (24 bytes)
//   Type=03 (Messaging), SubType=15 (FC Ex), Length=24, Reserved=0, WWN=0x0001020304050607, LUN=0x0001020304050607
// End: 7f ff 04 00 (4 bytes)
//   Type=7f (End), SubType=ff (End Entire), Length=4
#[test]
fn fibre_channel_ex() {
    let results = decode(&[
        &[0x02, 0x01, 0x0c, 0x00], // Node: ACPI: Type=02, SubType=01, Length=12
        &[0xd0, 0x41, 0x03, 0x0a], // HID: 0x0a0341d0 (little-endian)
        &[0x00, 0x00, 0x00, 0x00], // UID: 0 (little-endian)
        &[0x01, 0x01, 0x06, 0x00], // Node: PCI: Type=01, SubType=01, Length=6
        &[0x00],                   // Function: 0
        &[0x1f],                   // Device: 31
        &[0x03, 0x15, 0x18, 0x00], // Node: FC Ex: Type=03, SubType=15, Length=24
        &[0x00, 0x00, 0x00, 0x00], // Reserved: 0
        &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00], // WWN: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00]
        &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00], // LUN: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00]
        &[0x7f, 0xff, 0x04, 0x00], // Node: Terminator: Type=7f, SubType=ff, Length=4
    ])
    .unwrap();

    let expected = Path::from(vec![
        Node::Acpi(acpi::Acpi::Standard(acpi::standard::Standard {
            hid: 0x0a03_41d0,
            uid: 0,
        })),
        Node::Hardware(hw::Hardware::Pci(hw::pci::Pci {
            function: 0,
            device: 31,
        })),
        Node::Messaging(msg::Messaging::FibreChannelEx(msg::fc::FibreChannelEx {
            wwn: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00],
            lun: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00],
        })),
    ]);

    assert_eq!(results, expected.into());
}

// SCSI Device Path
// Source: UEFI 2.11 spec section 10.3.4.2 "SCSI Device Path"
// Path: PciRoot(0)/PCI(1,0)/SCSI(0,0) - SCSI target 0, LUN 0
//
// HEX BREAKDOWN ACCORDING TO UEFI 2.11 SPEC:
// ACPI Root: 02 01 0c 00 d0 41 03 0a 00 00 00 00 (12 bytes)
//   Type=02 (ACPI), SubType=01 (Standard), Length=12, HID=0x0a0341d0, UID=0
// PCI: 01 01 06 00 00 01 (6 bytes)
//   Type=01 (Hardware), SubType=01 (PCI), Length=6, Function=0, Device=1
// SCSI: 03 02 08 00 00 00 00 00 (8 bytes)
//   Type=03 (Messaging), SubType=02 (SCSI), Length=8, Target=0, LUN=0
// End: 7f ff 04 00 (4 bytes)
//   Type=7f (End), SubType=ff (End Entire), Length=4
#[test]
fn scsi() {
    let results = decode(&[
        &[0x02, 0x01, 0x0c, 0x00], // Node: ACPI: Type=02, SubType=01, Length=12
        &[0xd0, 0x41, 0x03, 0x0a], // HID: 0x0a0341d0 (little-endian)
        &[0x00, 0x00, 0x00, 0x00], // UID: 0 (little-endian)
        &[0x01, 0x01, 0x06, 0x00], // Node: PCI: Type=01, SubType=01, Length=6
        &[0x00],                   // Function: 0
        &[0x01],                   // Device: 1
        &[0x03, 0x02, 0x08, 0x00], // Node: SCSI: Type=03, SubType=02, Length=8
        &[0x00, 0x00],             // Target ID: 0 (little-endian)
        &[0x00, 0x00],             // LUN: 0 (little-endian)
        &[0x7f, 0xff, 0x04, 0x00], // Node: Terminator: Type=7f, SubType=ff, Length=4
    ])
    .unwrap();

    let expected = Path::from(vec![
        Node::Acpi(acpi::Acpi::Standard(acpi::standard::Standard {
            hid: 0x0a03_41d0,
            uid: 0,
        })),
        Node::Hardware(hw::Hardware::Pci(hw::pci::Pci {
            function: 0,
            device: 1,
        })),
        Node::Messaging(msg::Messaging::Scsi(msg::scsi::Scsi { tid: 0, lun: 0 })),
    ]);

    assert_eq!(results, expected.into());
}

// SATA Device Path
// Source: UEFI 2.11 spec section 10.3.4.6 SATA Device Path - HBA port 0, no port multiplier, LUN 0
// Path: PciRoot(0)/PCI(1,0)/Sata(0,0xFFFF,0) - SATA direct connection
//
// HEX BREAKDOWN ACCORDING TO UEFI 2.11 SPEC:
// ACPI Root: 02 01 0c 00 d0 41 03 0a 00 00 00 00 (12 bytes)
//   Type=02 (ACPI), SubType=01 (Standard), Length=12, HID=0x0a0341d0, UID=0
// PCI: 01 01 06 00 00 01 (6 bytes)
//   Type=01 (Hardware), SubType=01 (PCI), Length=6, Function=0, Device=1
// SATA: 03 12 0a 00 00 00 ff ff 00 00 (10 bytes)
//   Type=03 (Messaging), SubType=12 (SATA), Length=10, HBA=0, PM=0xFFFF, LUN=0
// End: 7f ff 04 00 (4 bytes)
//   Type=7f (End), SubType=ff (End Entire), Length=4
#[test]
fn sata() {
    let results = decode(&[
        &[0x02, 0x01, 0x0c, 0x00], // Node: ACPI: Type=02, SubType=01, Length=12
        &[0xd0, 0x41, 0x03, 0x0a], // HID: 0x0a0341d0 (little-endian)
        &[0x00, 0x00, 0x00, 0x00], // UID: 0 (little-endian)
        &[0x01, 0x01, 0x06, 0x00], // Node: PCI: Type=01, SubType=01, Length=6
        &[0x00],                   // Function: 0
        &[0x01],                   // Device: 1
        &[0x03, 0x12, 0x0a, 0x00], // Node: SATA: Type=03, SubType=12, Length=10
        &[0x00, 0x00],             // HBA Port: 0 (little-endian)
        &[0xff, 0xff],             // PM Port: 0xFFFF (no multiplier, little-endian)
        &[0x00, 0x00],             // LUN: 0 (little-endian)
        &[0x7f, 0xff, 0x04, 0x00], // Node: Terminator: Type=7f, SubType=ff, Length=4
    ])
    .unwrap();

    let expected = Path::from(vec![
        Node::Acpi(acpi::Acpi::Standard(acpi::standard::Standard {
            hid: 0x0a03_41d0,
            uid: 0,
        })),
        Node::Hardware(hw::Hardware::Pci(hw::pci::Pci {
            device: 1,
            function: 0,
        })),
        Node::Messaging(msg::Messaging::Sata(msg::sata::Sata {
            hba_port: 0,
            pm_port: 0xFFFF,
            lun: 0,
        })),
    ]);

    assert_eq!(results, expected.into());
}

// ATAPI Device Path
// Source: UEFI 2.11 spec section 10.3.4.1 "ATAPI Device Path"
// Path: PciRoot(0)/PCI(1,1)/Atapi(Primary,Master,0) - ATAPI primary master drive
//
// HEX BREAKDOWN ACCORDING TO UEFI 2.11 SPEC:
// ACPI Root: 02 01 0c 00 d0 41 03 0a 00 00 00 00 (12 bytes)
//   Type=02 (ACPI), SubType=01 (Standard), Length=12, HID=0x0a0341d0, UID=0
// PCI: 01 01 06 00 01 01 (6 bytes)
//   Type=01 (Hardware), SubType=01 (PCI), Length=6, Function=1, Device=1
// ATAPI: 03 01 08 00 00 00 00 00 (8 bytes)
//   Type=03 (Messaging), SubType=01 (ATAPI), Length=8, Primary=0, Master=0, LUN=0
// End: 7f ff 04 00 (4 bytes)
//   Type=7f (End), SubType=ff (End Entire), Length=4
#[test]
fn atapi() {
    let results = decode(&[
        &[0x02, 0x01, 0x0c, 0x00], // Node: ACPI: Type=02, SubType=01, Length=12
        &[0xd0, 0x41, 0x03, 0x0a], // HID: 0x0a0341d0 (little-endian)
        &[0x00, 0x00, 0x00, 0x00], // UID: 0 (little-endian)
        &[0x01, 0x01, 0x06, 0x00], // Node: PCI: Type=01, SubType=01, Length=6
        &[0x01],                   // Function: 1
        &[0x01],                   // Device: 1
        &[0x03, 0x01, 0x08, 0x00], // Node: ATAPI: Type=03, SubType=01, Length=8
        &[0x00],                   // Primary/Secondary: 0 (Primary)
        &[0x00],                   // Master/Slave: 0 (Master)
        &[0x00, 0x00],             // LUN: 0 (little-endian)
        &[0x7f, 0xff, 0x04, 0x00], // Node: Terminator: Type=7f, SubType=ff, Length=4
    ])
    .unwrap();

    let expected = Path::from(vec![
        Node::Acpi(acpi::Acpi::Standard(acpi::standard::Standard {
            hid: 0x0a03_41d0,
            uid: 0,
        })),
        Node::Hardware(hw::Hardware::Pci(hw::pci::Pci {
            function: 1,
            device: 1,
        })),
        Node::Messaging(msg::Messaging::Atapi(msg::atapi::Atapi {
            primary: 0,
            slave: 0,
            lun: 0,
        })),
    ]);

    assert_eq!(results, expected.into());
}

// NVMe over Fabric Namespace Device Path
// Source: UEFI 2.11 spec section 10.3.4.33 "NVMe over Fabric (NVMe-oF) Namespace Device Path Example"
// Path: PciRoot(0)/Pci(25,0)/Mac(001320F5FA77,1)/IPv4(192.168.0.1,TCP,Static,192.168.0.100)/NVMEoF(nqn.1991-05.org.uefi:nvmeof-nvme-target,4EFF7F8ED3534E9BA4ECDEEA8EAB84D7)
//
// HEX BREAKDOWN ACCORDING TO UEFI 2.11 SPEC:
// ACPI Root: 02 01 0c 00 d0 41 03 0a 00 00 00 00 (12 bytes)
//   Type=02 (ACPI), SubType=01 (Standard), Length=12, HID=0x0a0341d0, UID=0
// PCI: 01 01 06 00 00 19 (6 bytes)
//   Type=01 (Hardware), SubType=01 (PCI), Length=6, Function=0, Device=25
// MAC: 03 0b 25 00 00 13 20 f5 fa 77 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01 (37 bytes)
//   Type=03 (Messaging), SubType=0b (MAC), Length=37, MAC=[00,13,20,f5,fa,77,...], IfType=1
// IPv4: 03 0c 1b 00 c0 a8 00 64 c0 a8 00 01 bc 0c 06 00 06 00 01 00 00 00 00 ff ff ff 00 (27 bytes)
//   Type=03 (Messaging), SubType=0c (IPv4), Length=27, Local=192.168.0.100:3260, Remote=192.168.0.1:6, Protocol=TCP, Static=true, Gateway=0.0.0.0, Mask=255.255.255.0
// NVMEoF: 03 22 44 00 02 4e ff 7f 8e d3 53 4e 9b a4 ec de ea 8e ab 84 d7 00 00 00 00 00 6e 71 6e 2e 31 39 39 31 2d 30 35 2e 6f 72 67 2e 75 65 66 69 3a 6e 76 6d 65 6f 66 2d 6e 76 6d 65 2d 74 61 72 67 65 74 (68 bytes)
//   Type=03 (Messaging), SubType=22 (NVMEoF), Length=68, NIDT=02 (NGUID), NID=4EFF7F8ED3534E9BA4ECDEEA8EAB84D7, NQN="nqn.1991-05.org.uefi:nvmeof-nvme-target"
// End: 7f ff 04 00 (4 bytes)
//   Type=7f (End), SubType=ff (End Entire), Length=4
// NVMe over Fabric Namespace Device Path
// Source: UEFI 2.11 spec section 10.3.4.33 "NVMe over Fabric (NVMe-oF) Namespace Device Path Example"
// Path: PciRoot(0)/Pci(25,0)/Mac(001320F5FA77,1)/IPv4(192.168.0.1,TCP,Static,192.168.0.100)/NVMEoF(nqn.1991-05.org.uefi:nvmeof-nvme-target,4EFF7F8ED3534E9BA4ECDEEA8EAB84D7)
//
// HEX BREAKDOWN CORRECTED FOR SPEC ERRORS:
// ACPI Root: 02 01 0c 00 d0 41 03 0a 00 00 00 00 (12 bytes)
//   Type=02 (ACPI), SubType=01 (Standard), Length=12, HID=0x0a0341d0, UID=0
// PCI: 01 01 06 00 00 19 (6 bytes)
//   Type=01 (Hardware), SubType=01 (PCI), Length=6, Function=0, Device=25
// MAC: 03 0b 25 00 00 13 20 f5 fa 77 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01 (37 bytes)
//   Type=03 (Messaging), SubType=0b (MAC), Length=37, MAC=[00,13,20,f5,fa,77,...], IfType=1
// IPv4: 03 0c 1b 00 c0 a8 00 64 c0 a8 00 01 00 00 bc 0c 06 00 01 00 00 00 00 ff ff ff 00 (27 bytes)
//   Type=03 (Messaging), SubType=0c (IPv4), Length=27, Local=192.168.0.100:0, Remote=192.168.0.1:3260, Protocol=TCP, Static=true, Gateway=0.0.0.0, Mask=255.255.255.0
//   CORRECTED: Local port now 00 00, Remote port now bc 0c (3260 in little-endian)
// NVMEoF: 03 22 3d 00 02 4e ff 7f 8e d3 53 4e 9b a4 ec de ea 8e ab 84 d7 6e 71 6e 2e 31 39 39 31 2d 30 35 2e 6f 72 67 2e 75 65 66 69 3a 6e 76 6d 65 6f 66 2d 6e 76 6d 65 2d 74 61 72 67 65 74 00 (61 bytes)
//   Type=03 (Messaging), SubType=22 (NVMEoF), Length=61, NIDT=02 (NGUID), NID=4EFF7F8ED3534E9BA4ECDEEA8EAB84D7, NQN="nqn.1991-05.org.uefi:nvmeof-nvme-target\0"
//   CORRECTED: Length now 61 bytes (4+1+16+40 for string+null terminator), NQN includes null terminator
// End: 7f ff 04 00 (4 bytes)
//   Type=7f (End), SubType=ff (End Entire), Length=4
#[test]
fn nvmeof_ipv4() {
    let results = decode(&[
        &[0x02, 0x01, 0x0c, 0x00], // Node: ACPI: Type=02, SubType=01, Length=12
        &[0xd0, 0x41, 0x03, 0x0a], // HID: 0x0a0341d0 (little-endian)
        &[0x00, 0x00, 0x00, 0x00], // UID: 0 (little-endian)
        &[0x01, 0x01, 0x06, 0x00], // Node: PCI: Type=01, SubType=01, Length=6
        &[0x00],                   // Function: 0
        &[0x19],                   // Device: 25
        &[0x03, 0x0b, 0x25, 0x00], // Node: MAC: Type=03, SubType=0b, Length=37
        &[0x00, 0x13, 0x20, 0xf5, 0xfa, 0x77], // MAC address first 6 bytes
        &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // MAC padding 10 bytes
        &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // MAC padding 10 bytes
        &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // MAC padding 6 bytes
        &[0x01],                   // Interface Type: 1 (OTHER)
        &[0x03, 0x0c, 0x1b, 0x00], // Node: IPv4: Type=03, SubType=0c, Length=27
        &[0xc0, 0xa8, 0x00, 0x64], // Local IP: 192.168.0.100
        &[0xc0, 0xa8, 0x00, 0x01], // Remote IP: 192.168.0.1
        &[0x00, 0x00],             // Local Port: 0
        &[0xbc, 0x0c],             // Remote Port: 3260 (CORRECTED: little-endian)
        &[0x06, 0x00],             // Protocol: 6 (TCP, little-endian)
        &[0x01],                   // Static IP: true
        &[0x00, 0x00, 0x00, 0x00], // Gateway IP: 0.0.0.0
        &[0xff, 0xff, 0xff, 0x00], // Subnet Mask: 255.255.255.0
        &[0x03, 0x22, 0x3d, 0x00], // Node: NVMEoF: Type=03, SubType=22, Length=61 (CORRECTED)
        &[0x02],                   // NIDT: 2 (NGUID)
        &[0x4e, 0xff, 0x7f, 0x8e, 0xd3, 0x53, 0x4e, 0x9b], // NID first 8 bytes
        &[0xa4, 0xec, 0xde, 0xea, 0x8e, 0xab, 0x84, 0xd7], // NID last 8 bytes
        // NQN: "nqn.1991-05.org.uefi:nvmeof-nvme-target" + null terminator (40 bytes UTF-8)
        // "nqn.1991-05.org.uefi:nvmeof-nvme-target\0" (CORRECTED: includes null terminator)
        &[
            0x6e, 0x71, 0x6e, 0x2e, 0x31, 0x39, 0x39, 0x31, 0x2d, 0x30, 0x35, 0x2e, 0x6f, 0x72,
            0x67, 0x2e, 0x75, 0x65, 0x66, 0x69, 0x3a, 0x6e, 0x76, 0x6d, 0x65, 0x6f, 0x66, 0x2d,
            0x6e, 0x76, 0x6d, 0x65, 0x2d, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x00,
        ],
        &[0x7f, 0xff, 0x04, 0x00], // Node: Terminator: Type=7f, SubType=ff, Length=4
    ])
    .unwrap();

    let expected = Path::from(vec![
        Node::Acpi(acpi::Acpi::Standard(acpi::standard::Standard {
            hid: 0x0a03_41d0,
            uid: 0,
        })),
        Node::Hardware(hw::Hardware::Pci(hw::pci::Pci {
            function: 0,
            device: 25,
        })),
        Node::Messaging(msg::Messaging::MacAddress(msg::mac::MacAddress {
            mac: [
                0x00, 0x13, 0x20, 0xf5, 0xfa, 0x77, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
            if_type: msg::mac::InterfaceType::OTHER,
        })),
        Node::Messaging(msg::Messaging::Ipv4(msg::ipv4::Ipv4 {
            local: std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(192, 168, 0, 100), 0),
            remote: std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(192, 168, 0, 1), 3260),
            protocol: msg::ipv4::Protocol::TCP,
            static_ip: true,
            gateway_ip: std::net::Ipv4Addr::UNSPECIFIED,
            subnet_mask: std::net::Ipv4Addr::new(255, 255, 255, 0),
        })),
        Node::Messaging(msg::Messaging::NvmeOfNamespace(
            msg::nvme_of::NvmeOfNamespace {
                nid: msg::nvme_of::NamespaceId::Nguid([
                    0x4e, 0xff, 0x7f, 0x8e, 0xd3, 0x53, 0x4e, 0x9b, 0xa4, 0xec, 0xde, 0xea, 0x8e,
                    0xab, 0x84, 0xd7,
                ]),
                nqn: "nqn.1991-05.org.uefi:nvmeof-nvme-target".to_string(),
            },
        )),
    ]);

    assert_eq!(results, expected.into());
}

// Memory Mapped Device Path
// Source: UEFI 2.11 spec section 10.3.2.3 "Memory Mapped Device Path"
// Path: PciRoot(0)/MemoryMapped(EfiMemoryMappedIO,0x000000003F000000,0x000000003F00FFFF)
//
// HEX BREAKDOWN ACCORDING TO UEFI 2.11 SPEC:
// ACPI Root: 02 01 0c 00 d0 41 03 0a 00 00 00 00 (12 bytes)
//   Type=02 (ACPI), SubType=01 (Standard), Length=12, HID=0x0a0341d0, UID=0
// Memory Mapped: 01 03 18 00 00 00 00 00 00 00 00 00 3f 00 00 00 00 ff ff 00 3f 00 00 00 (24 bytes)
//   Type=01 (Hardware), SubType=03 (MemMap), Length=24, MemType=0, Start=0x000000003F000000, End=0x000000003F00FFFF
// End: 7f ff 04 00 (4 bytes)
//   Type=7f (End), SubType=ff (End Entire), Length=4
#[test]
fn memory_mapped() {
    let results = decode(&[
        &[0x02, 0x01, 0x0c, 0x00], // Node: ACPI: Type=02, SubType=01, Length=12
        &[0xd0, 0x41, 0x03, 0x0a], // HID: 0x0a0341d0 (little-endian)
        &[0x00, 0x00, 0x00, 0x00], // UID: 0 (little-endian)
        &[0x01, 0x03, 0x18, 0x00], // Node: MemMap: Type=01, SubType=03, Length=24
        &[0x00, 0x00, 0x00, 0x00], // Memory Type: 0 (EfiReservedMemoryType)
        &[0x00, 0x00, 0x00, 0x3f, 0x00, 0x00, 0x00, 0x00], // Start Address: 0x3f000000 (little-endian)
        &[0xff, 0xff, 0x00, 0x3f, 0x00, 0x00, 0x00, 0x00], // End Address: 0x3f00ffff (little-endian)
        &[0x7f, 0xff, 0x04, 0x00], // Node: Terminator: Type=7f, SubType=ff, Length=4
    ])
    .unwrap();

    let expected = Path::from(vec![
        Node::Acpi(acpi::Acpi::Standard(acpi::standard::Standard {
            hid: 0x0a03_41d0,
            uid: 0,
        })),
        Node::Hardware(hw::Hardware::MemMap(hw::memmap::MemMap {
            kind: hw::memmap::MemoryType::Reserved,
            start: 0x3f00_0000,
            end: 0x3f00_ffff,
        })),
    ]);

    assert_eq!(results, expected.into());
}

// UART Device Path
// Source: UEFI 2.11 spec section 10.3.4.15 "UART Device Path"
// Path: PciRoot(0)/PCI(1,1)/Uart(115200,8,N,1) - Standard UART configuration
//
// HEX BREAKDOWN ACCORDING TO UEFI 2.11 SPEC:
// ACPI Root: 02 01 0c 00 d0 41 03 0a 00 00 00 00 (12 bytes)
//   Type=02 (ACPI), SubType=01 (Standard), Length=12, HID=0x0a0341d0, UID=0
// PCI: 01 01 06 00 01 01 (6 bytes)
//   Type=01 (Hardware), SubType=01 (PCI), Length=6, Function=1, Device=1
// UART: 03 0e 13 00 00 00 00 00 00 00 c2 01 00 00 00 00 00 00 08 01 01 (19 bytes)
//   Type=03 (Messaging), SubType=0e (UART), Length=19, Reserved=0, BaudRate=115200, DataBits=8, Parity=1(None), StopBits=1
// End: 7f ff 04 00 (4 bytes)
//   Type=7f (End), SubType=ff (End Entire), Length=4
#[test]
fn uart() {
    let results = decode(&[
        &[0x02, 0x01, 0x0c, 0x00], // Node: ACPI: Type=02, SubType=01, Length=12
        &[0xd0, 0x41, 0x03, 0x0a], // HID: 0x0a0341d0 (little-endian)
        &[0x00, 0x00, 0x00, 0x00], // UID: 0 (little-endian)
        &[0x01, 0x01, 0x06, 0x00], // Node: PCI: Type=01, SubType=01, Length=6
        &[0x01],                   // Function: 1
        &[0x01],                   // Device: 1
        &[0x03, 0x0e, 0x13, 0x00], // Node: UART: Type=03, SubType=0e, Length=19
        &[0x00, 0x00, 0x00, 0x00], // Reserved: 0
        &[0x00, 0xc2, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00], // Baud Rate: 115200 (little-endian)
        &[0x08],                   // Data Bits: 8
        &[0x01],                   // Parity: 1 (None)
        &[0x01],                   // Stop Bits: 1
        &[0x7f, 0xff, 0x04, 0x00], // Node: Terminator: Type=7f, SubType=ff, Length=4
    ])
    .unwrap();

    let expected = Path::from(vec![
        Node::Acpi(acpi::Acpi::Standard(acpi::standard::Standard {
            hid: 0x0a03_41d0,
            uid: 0,
        })),
        Node::Hardware(hw::Hardware::Pci(hw::pci::Pci {
            function: 1,
            device: 1,
        })),
        Node::Messaging(msg::Messaging::Uart(msg::uart::Uart {
            baud: 115_200,
            data_bits: 8,
            parity: 1,    // None
            stop_bits: 1, // One
        })),
    ]);

    assert_eq!(results, expected.into());
}

// Controller Device Path
// Source: UEFI 2.11 spec section 10.3.2.5 "Controller Device Path"
// Path: PciRoot(0)/PCI(1,0)/Ctrl(0) - Controller number 0
//
// HEX BREAKDOWN ACCORDING TO UEFI 2.11 SPEC:
// ACPI Root: 02 01 0c 00 d0 41 03 0a 00 00 00 00 (12 bytes)
//   Type=02 (ACPI), SubType=01 (Standard), Length=12, HID=0x0a0341d0, UID=0
// PCI: 01 01 06 00 00 01 (6 bytes)
//   Type=01 (Hardware), SubType=01 (PCI), Length=6, Function=0, Device=1
// Controller: 01 05 08 00 00 00 00 00 (8 bytes)
//   Type=01 (Hardware), SubType=05 (Controller), Length=8, Controller=0
// End: 7f ff 04 00 (4 bytes)
//   Type=7f (End), SubType=ff (End Entire), Length=4
#[test]
fn controller() {
    let results = decode(&[
        &[0x02, 0x01, 0x0c, 0x00], // Node: ACPI: Type=02, SubType=01, Length=12
        &[0xd0, 0x41, 0x03, 0x0a], // HID: 0x0a0341d0 (little-endian)
        &[0x00, 0x00, 0x00, 0x00], // UID: 0 (little-endian)
        &[0x01, 0x01, 0x06, 0x00], // Node: PCI: Type=01, SubType=01, Length=6
        &[0x00],                   // Function: 0
        &[0x01],                   // Device: 1
        &[0x01, 0x05, 0x08, 0x00], // Node: Controller: Type=01, SubType=05, Length=8
        &[0x00, 0x00, 0x00, 0x00], // Controller Number: 0 (little-endian)
        &[0x7f, 0xff, 0x04, 0x00], // Node: Terminator: Type=7f, SubType=ff, Length=4
    ])
    .unwrap();

    let expected = Path::from(vec![
        Node::Acpi(acpi::Acpi::Standard(acpi::standard::Standard {
            hid: 0x0a03_41d0,
            uid: 0,
        })),
        Node::Hardware(hw::Hardware::Pci(hw::pci::Pci {
            function: 0,
            device: 1,
        })),
        Node::Hardware(hw::Hardware::Controller(hw::controller::Controller(0))),
    ]);

    assert_eq!(results, expected.into());
}
