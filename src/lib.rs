#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::{vec, vec::Vec};
use uefi::CStr16;

pub fn ucs2_decode(s: &CStr16) -> String {
    let mut buffer: Vec<u8> = vec![0; s.to_u16_slice().len() * 2];
    let bytes = ucs2::decode(s.to_u16_slice(), &mut buffer).expect("UCS-2 decode failed");
    buffer.resize(bytes, 0);
    String::from_utf8(buffer).expect("UTF-8 decode failed")
}
