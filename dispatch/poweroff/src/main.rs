// SPDX-FileCopyrightText: 2025 Advanced Micro Devices, Inc.
// SPDX-License-Identifier: MIT

#![no_main]
#![no_std]

use uefi::prelude::*;
use uefi::runtime::{reset, ResetType};

#[entry]
fn main() -> Status {
    reset(ResetType::SHUTDOWN, Status::SUCCESS, None)
}
