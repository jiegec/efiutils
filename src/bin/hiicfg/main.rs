#![no_std]
#![no_main]

use efiutils::WithContext;
use log::warn;
use uefi::proto::unsafe_protocol;
use uefi::table::boot::SearchType;
use uefi::CStr16;
use uefi::{prelude::*, Char16};

pub use uefi::proto;
use uefi_services::println;

#[repr(C)]
#[unsafe_protocol("587e72d7-cc50-4f79-8209-ca291fc1a10f")]
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

fn main(_image: uefi::Handle, st: SystemTable<Boot>) -> efiutils::Result<()> {
    let bt = st.boot_services();

    let handle = bt
        .locate_handle_buffer(SearchType::from_proto::<HiiConfigRouting>())
        .with_context("Failed to find protocol handle")?[0];

    let routing = bt
        .open_protocol_exclusive::<HiiConfigRouting>(handle)
        .with_context("Locate hii config routing protocol failed")?;

    let mut results: *const Char16 = 0 as *const Char16;
    let _res = (routing.export_config)(&routing, &mut results);
    let s = unsafe { CStr16::from_ptr(results) };
    println!("{}", s);
    Ok(())
}

#[entry]
fn efi_main(image: uefi::Handle, mut st: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut st).expect("UEFI services init failed");

    match main(image, st) {
        Ok(_) => Status::SUCCESS,
        Err(err) => {
            warn!("Error {}", err);
            Status::ABORTED
        }
    }
}
