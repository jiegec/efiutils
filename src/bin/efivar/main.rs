#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec;
use anyhow::bail;
use efiutils::{err, ucs2_decode_ptr, ShellParameters};
use log::*;
use uefi::{prelude::*, Guid};
use uefi::{CStr16, Char16};
use uefi_services::{print, println};

fn main(image: uefi::Handle, st: SystemTable<Boot>) -> anyhow::Result<()> {
    let bt = st.boot_services();
    let rt = st.runtime_services();

    let params = bt
        .open_protocol_exclusive::<ShellParameters>(image)
        .expect("Locate shell parameter protocol failed");

    let argv: &[*const Char16] = unsafe { core::slice::from_raw_parts(params.argv, params.argc) };
    if params.argc == 6 {
        // edit
        // args: guid name offset width value
        let guid_str = unsafe { ucs2_decode_ptr(argv[1]) }?;
        let name_str = unsafe { ucs2_decode_ptr(argv[2]) }?;
        let offset_str = unsafe { ucs2_decode_ptr(argv[3]) }?;
        let width_str = unsafe { ucs2_decode_ptr(argv[4]) }?;
        let value_str = unsafe { ucs2_decode_ptr(argv[5]) }?;

        let guid = match Guid::try_parse(&guid_str) {
            Ok(guid) => guid,
            Err(err) => bail!("Failed to parse guid: {}", err),
        };
        let offset = str::parse::<usize>(&offset_str).map_err(err)?;
        let width = str::parse::<usize>(&width_str).map_err(err)?;
        let value = str::parse::<u64>(&value_str).map_err(err)?;
        println!("GUID={}", guid);
        println!("NAME={}", name_str);

        let name_cstr = unsafe { CStr16::from_ptr(argv[2]) };
        let data_size = rt
            .get_variable_size(&name_cstr, &uefi::table::runtime::VariableVendor(guid))
            .expect("Failed to get variable");
        let mut data = vec![0u8; data_size];
        let (_, attributes) = rt
            .get_variable(
                name_cstr,
                &uefi::table::runtime::VariableVendor(guid),
                data.as_mut_slice(),
            )
            .expect("Failed to get variable");

        print!("ORIG=");
        for byte in &data {
            print!("{:02X}", byte);
        }
        println!();

        let bytes = value.to_le_bytes();
        for i in 0..width {
            data[offset + i] = bytes[width - i - 1];
        }

        print!("NEW =");
        for byte in &data {
            print!("{:02X}", byte);
        }
        println!();

        rt.set_variable(
            &name_cstr,
            &uefi::table::runtime::VariableVendor(guid),
            attributes,
            &data,
        )
        .expect("Failed to set variable");

        print!("Done");
    } else if params.argc == 1 {
        // list
        let mut data = vec![0u8; 1024];

        for key in rt.variable_keys().expect("Failed to list variables") {
            println!("GUID={}", key.vendor.0);
            let name = key.name().unwrap();
            println!("NAME={}", efiutils::ucs2_decode(&name)?);

            let data_size = rt
                .get_variable_size(name, &key.vendor)
                .expect("Failed to get variable size");
            data.resize(data_size, 0);
            let (_, attributes) = rt
                .get_variable(name, &key.vendor, data.as_mut_slice())
                .expect("Failed to get variable");

            println!("ATTR={:?}", attributes);

            let data_slice = &data[..data_size];
            print!("DATA=");
            for byte in data_slice {
                print!("{:02X}", byte);
            }
            println!();
        }
    } else {
        info!("Usage:");
        info!("\tefivar.efi: List all variables");
        info!("\tefivar.efi [guid] [name] [offset] [width] [value]: Update value");
    }

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
