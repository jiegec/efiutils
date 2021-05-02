#![no_std]
#![no_main]
#![feature(abi_efiapi, negative_impls)]

use core::{borrow::Borrow, fmt::Write};

use log::*;
use uefi::{prelude::*, proto::media::file::RegularFile, Char16, Guid, Identify};
use uefi::{
    proto::media::{
        file::{File, FileAttribute},
        fs::SimpleFileSystem,
    },
    unsafe_guid,
};
use uefi::{
    proto::{console::text::Input, Protocol},
    CStr16,
};

pub use uefi::proto;

#[repr(C)]
#[unsafe_guid("587e72d7-cc50-4f79-8209-ca291fc1a10f")]
#[derive(Protocol)]
pub struct HiiConfigRouting {
    extract_config: extern "efiapi" fn(
        &HiiConfigRouting,
        request: *const Char16,
        progress: *mut *const Char16,
        results: *mut *const Char16,
    ) -> Status,
    export_config: extern "efiapi" fn(&HiiConfigRouting, results: *mut *const Char16) -> Status,
    route_config: extern "efiapi" fn(
        &HiiConfigRouting,
        configuration: *const Char16,
        progress: *mut *const Char16,
    ) -> Status,
}

#[entry]
fn efi_main(image: uefi::Handle, st: SystemTable<Boot>) -> Status {
    uefi_services::init(&st).expect_success("failed to initialize utilities");
    let bt = st.boot_services();

    let routing = bt.locate_protocol::<HiiConfigRouting>().unwrap().unwrap();
    let routing = unsafe { &mut *routing.get() };

    let mut results: *const Char16 = 0 as *const Char16;
    let res = (routing.export_config)(&routing, &mut results);
    let s = unsafe { CStr16::from_ptr(results) };
    st.stdout().output_string(s).unwrap().unwrap();

    Status::SUCCESS
}
