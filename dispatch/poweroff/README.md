# UEFI Power Off Module

A simple UEFI application that powers off the system when loaded. Written in Rust 
with minimal dependencies and no unsafe code.

## Purpose

This application is designed to be loaded by UEFI firmware and will immediately 
trigger a system shutdown. This provides a quick and convenient way to power off 
a system from:

- A bootable USB drive
- A UEFI shell
- UEFI HTTP boot
- Any EFI-capable firmware interface

## Building

### Prerequisites

- Rust 1.81+ toolchain

The project includes a `rust-toolchain.toml` file that automatically installs the 
required `x86_64-unknown-uefi` target.

### Steps

To build the project:
   
```bash
cargo build --release --target=x86_64-unknown-uefi
```

The resulting EFI file will be located at  `target/x86_64-unknown-uefi/release/poweroff.efi`.

The `--target` option is likely optional for any tools that respect `.cargo/config.toml`.

## License

This project is licensed under the MIT License.
