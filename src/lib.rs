#![no_std]
#![feature(abi_efiapi, negative_impls)]

extern crate alloc;

use alloc::string::String;
use alloc::{vec, vec::Vec};
use uefi::proto::Protocol;
use uefi::unsafe_guid;
use uefi::CStr16;
use uefi::{prelude::*, Guid, Identify};

pub use uefi::proto;

#[repr(C)]
#[unsafe_guid("b9d4c360-bcfb-4f9b-9298-53c136982258")]
#[derive(Protocol)]
pub struct FormBrowser2 {
    pub send_form: extern "efiapi" fn(
        &FormBrowser2,
        handles: *const usize,
        handle_count: usize,
        formset_guid: *const Guid,
        form_id: u16,
        screen_dimensions: *const u8,
        action_request: *mut u8,
    ) -> Status,
}

#[repr(C)]
#[unsafe_guid("ef9fc172-a1b2-4693-b327-6d32fc416042")]
#[derive(Protocol)]
pub struct HiiDatabase {
    pub new_package_list: extern "efiapi" fn(
        &HiiDatabase,
        package_list: *const u8,
        driver_handle: usize,
        handle: *mut usize,
    ) -> Status,
    pub remove_package_list: extern "efiapi" fn(&HiiDatabase, handle: usize) -> Status,
    pub update_package_list:
        extern "efiapi" fn(&HiiDatabase, handle: usize, package_list: *const u8) -> Status,
    pub list_package_lists: extern "efiapi" fn(
        &HiiDatabase,
        package_type: u8,
        package_guid: *const Guid,
        handle_buffer_length: *mut usize,
        handle: *mut usize,
    ) -> Status,
    pub export_package_lists: extern "efiapi" fn(
        &HiiDatabase,
        handle: usize,
        buffer_size: *mut usize,
        buffer: *mut u8,
    ) -> Status,
}

pub fn ucs2_decode(s: &CStr16) -> String {
    let mut buffer: Vec<u8> = vec![0; s.to_u16_slice().len() * 2];
    let bytes = ucs2::decode(s.to_u16_slice(), &mut buffer).expect("UCS-2 decode failed");
    buffer.resize(bytes, 0);
    String::from_utf8(buffer).expect("UTF-8 decode failed")
}
