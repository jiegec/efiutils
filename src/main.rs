#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use log::*;
use uefi::prelude::*;

#[entry]
fn efi_main(image: uefi::Handle, st: SystemTable<Boot>) -> Status {
    uefi_services::init(&st).expect_success("failed to initialize utilities");
    info!("Hello, world!");
    Status::SUCCESS
}
