# Code Style Guide

## Module Organization

- **Module names**: Shortened, clear (e.g., `bluetooth_le` → `btle`, `device_logical_unit` → `dlu`)
- **Module structure**: One primary struct per sub-type, implementation in same file
- **File naming**: Match shortened module names

## Import Grouping

```rust
// 1. External crates (if any)
use core::net::Ipv4Addr;

// 2. Internal crate imports  
use crate::{Error, Head};
use crate::parser::{ByteOrder, Parser};

// 3. Relative imports
use super::ipv4::Protocol;
```

## Field Naming

- **Brief names**: `vendor_id` → `vid`, `mac_address` → `mac`, `baud_rate` → `baud`
- **Documentation**: Add doc comments for any ambiguous shortened names
- **Standard abbreviations**: `nsid`, `lun`, `wwn`, `eui`, `vid`, `pid`, `hba_port`, `pm_port`

## Data Types

### String Handling
- **UTF-8 strings**: Use `&CStr` for null-terminated UTF-8 strings
- **UTF-16 strings**: Use `String` for UTF-16 strings (converted from parser)
- **Raw strings**: Use `&str` for non-null-terminated UTF-8 strings (rare)

### Newtypes
- **Single field structs**: Use newtype pattern
```rust
pub struct Vlan(pub u16);
pub struct Protocol(pub u16);
```

### Enums
- **Small known value sets**: Use proper enums (no repr guarantees)
- **Parser trait**: Implement for validation
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Origin {
    Manual = 0x00,
    StatelessAutoConfiguration = 0x01,
}
```

### Structs
- **Multi-field types**: Regular structs with brief field names
- **Derive**: Always include `Debug, Clone, Copy, PartialEq, Eq, Hash`

## Reserved Fields

- **Storage**: Do NOT store reserved fields in structs
- **Validation**: Parse and validate in `TryFrom` implementation
- **Pattern**: `let _reserved: u32 = node.data.parse(ByteOrder::Little)?;`

## Error Handling

- **Invalid values**: Return `Error::Invalid` for spec violations
- **Validation**: Check reserved field values, invalid NSIDs, etc.

## Documentation

- **Struct docs**: Include UEFI sub-type number and byte length
- **Field docs**: Required for abbreviated field names
- **Example**:
```rust
/// SATA Device Path (SubType 0x12)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sata {
    /// HBA port number
    pub hba_port: u16,
    /// Port multiplier port number  
    pub pm_port: u16,
    pub lun: u16,
}
```

## Implementation Patterns

### Parse Order
- **Last field**: Always use `finish()` method to guarantee end of data
- **Earlier fields**: Use regular `parse()` method

### TryFrom Implementation
```rust
impl<'a> TryFrom<Head<'a>> for TypeName<'a> {
    type Error = Error;

    fn try_from(mut node: Head<'a>) -> Result<Self, Self::Error> {
        // Parse reserved fields (don't store)
        let _reserved: u32 = node.data.parse(ByteOrder::Little)?;
        
        // Parse fields (use finish() for last field)
        let field1 = node.data.parse(ByteOrder::Little)?;
        let utf8_string = node.data.finish(())?; // &CStr for UTF-8
        
        Ok(TypeName { field1, utf8_string })
    }
}
```

### Byte Order
- **Multi-byte integers**: Always specify `ByteOrder::Little` 
- **Single bytes**: Use `()` argument
- **Arrays**: Use `()` argument

## Constants

- **Protocol numbers**: Define as associated constants
- **Magic values**: Use descriptive constant names
- **Grouping**: Related constants together with docs
