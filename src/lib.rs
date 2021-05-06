#![no_std]
#![feature(abi_efiapi, negative_impls)]

extern crate alloc;

use alloc::fmt::Debug;
use alloc::string::String;
use alloc::{format, vec, vec::Vec};
use anyhow::anyhow;
use uefi::unsafe_guid;
use uefi::CStr16;
use uefi::{prelude::*, Guid, Identify};
use uefi::{proto::Protocol, Char16};

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

fn err<T: Debug>(e: T) -> anyhow::Error {
    anyhow!("{:?}", e)
}

pub fn ucs2_decode(s: &CStr16) -> anyhow::Result<String> {
    let mut buffer: Vec<u8> = vec![0; s.to_u16_slice().len() * 2];
    let bytes = ucs2::decode(s.to_u16_slice(), &mut buffer).map_err(err)?;
    buffer.resize(bytes, 0);
    Ok(String::from_utf8(buffer).map_err(err)?)
}

#[repr(C)]
#[unsafe_guid("752f3136-4e16-4fdc-a22a-e5f46812f4ca")]
#[derive(Protocol)]
pub struct ShellParameters {
    pub argv: *const *const Char16,
    pub argc: usize,
}

pub fn parse_guid(s: &str) -> anyhow::Result<Guid> {
    let parts: Vec<&str> = s.split("-").collect();
    if parts.len() != 5 {
        return Err(anyhow!("No enough guid parts"));
    }
    let a = u32::from_str_radix(parts[0], 16).map_err(err)?;
    let b = u16::from_str_radix(parts[1], 16).map_err(err)?;
    let c = u16::from_str_radix(parts[2], 16).map_err(err)?;
    let d = u16::from_str_radix(parts[3], 16).map_err(err)?;
    let e = u64::from_str_radix(parts[4], 16).map_err(err)?;
    let e0 = (e >> 40) as u8;
    let e1 = (e >> 32) as u8;
    let e2 = (e >> 24) as u8;
    let e3 = (e >> 16) as u8;
    let e4 = (e >> 8) as u8;
    let e5 = (e >> 0) as u8;
    Ok(Guid::from_values(a, b, c, d, [e0, e1, e2, e3, e4, e5]))
}
