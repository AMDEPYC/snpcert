//! Rust doesn't currently provide the artifact dependency environment
//! variables to the compiler during the build process. We re-export
//! the variable as POWEROFF_BIN_PATH to overcome this limitation.

use std::env;

fn main() {
    let poweroff_bin = env::var("CARGO_BIN_FILE_POWEROFF_poweroff").unwrap();
    println!("cargo:rustc-env=POWEROFF_BIN_PATH={}", poweroff_bin);
}
