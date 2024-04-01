#![no_std]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use core::fmt::Display;
use uefi::proto::unsafe_protocol;
use uefi::CStr16;
use uefi::Char16;
use uefi::{prelude::*, Guid};

pub use uefi::proto;

#[repr(C)]
#[unsafe_protocol("b9d4c360-bcfb-4f9b-9298-53c136982258")]
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
#[unsafe_protocol("ef9fc172-a1b2-4693-b327-6d32fc416042")]
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

pub unsafe fn ucs2_decode_ptr(p: *const Char16) -> Result<String> {
    ucs2_decode(CStr16::from_ptr(p))
}

pub fn ucs2_decode(s: &CStr16) -> Result<String> {
    let mut buffer: Vec<u8> = vec![0; s.to_u16_slice().len() * 2];
    let bytes =
        ucs2::decode(s.to_u16_slice(), &mut buffer).with_context("Failed to decode ucs2")?;
    buffer.resize(bytes, 0);
    Ok(String::from_utf8(buffer).with_context("Failed to decode utf8")?)
}

#[repr(C)]
#[unsafe_protocol("752f3136-4e16-4fdc-a22a-e5f46812f4ca")]
pub struct ShellParameters {
    pub argv: *const *const Char16,
    pub argc: usize,
}

pub trait WithContext {
    type ReturnType;

    fn with_context(self, ctx: &str) -> Result<Self::ReturnType>;
}

impl<T, E> WithContext for core::result::Result<T, E>
where
    E: Into<InnerError>,
{
    type ReturnType = T;

    fn with_context(self, ctx: &str) -> Result<Self::ReturnType> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Error {
                context: ctx.to_string(),
                inner: err.into(),
            }),
        }
    }
}

pub struct Error {
    context: String,
    inner: InnerError,
}

pub enum InnerError {
    Uefi(uefi::Error),
    FromUtf8(alloc::string::FromUtf8Error),
    Ucs2(ucs2::Error),
    GuidFromStr(uguid::GuidFromStrError),
    ParseInt(core::num::ParseIntError),
    FromSliceWithNul(uefi::data_types::FromSliceWithNulError),
}

impl From<uefi::Error> for InnerError {
    fn from(value: uefi::Error) -> Self {
        Self::Uefi(value)
    }
}

impl From<alloc::string::FromUtf8Error> for InnerError {
    fn from(value: alloc::string::FromUtf8Error) -> Self {
        Self::FromUtf8(value)
    }
}

impl From<ucs2::Error> for InnerError {
    fn from(value: ucs2::Error) -> Self {
        Self::Ucs2(value)
    }
}

impl From<uguid::GuidFromStrError> for InnerError {
    fn from(value: uguid::GuidFromStrError) -> Self {
        Self::GuidFromStr(value)
    }
}

impl From<core::num::ParseIntError> for InnerError {
    fn from(value: core::num::ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

impl From<uefi::data_types::FromSliceWithNulError> for InnerError {
    fn from(value: uefi::data_types::FromSliceWithNulError) -> Self {
        Self::FromSliceWithNul(value)
    }
}

impl Display for InnerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Uefi(e) => write!(f, "{}", e),
            Self::FromUtf8(e) => write!(f, "{}", e),
            Self::Ucs2(e) => write!(f, "{:?}", e),
            Self::GuidFromStr(e) => write!(f, "{}", e),
            Self::ParseInt(e) => write!(f, "{}", e),
            Self::FromSliceWithNul(e) => write!(f, "{}", e),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Context: {}, Error: {}", self.context, self.inner)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
