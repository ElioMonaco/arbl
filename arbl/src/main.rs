// src/main.rs
#![no_std]
#![no_main]

use uefi::prelude::*;
use crate::logging::setup_logger;

mod logging; // Declare the logging module

#[entry]
fn efi_main(handle: Handle, system_table: SystemTable<Boot>) -> Status {
    // Set up logging
    setup_logger();

    // Your boot manager logic
    system_table.stdout().write_str("Hello from the Boot Manager!\n").unwrap();

    // Infinite loop to keep UEFI running
    loop {}
}
