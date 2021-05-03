#![no_std]
#![no_main]
#![feature(abi_efiapi, negative_impls)]

extern crate alloc;

use alloc::vec;
use bitflags::bitflags;
use core::fmt::Write;
use uefi::{data_types::chars::NUL_16, CStr16};
use uefi::{prelude::*, Guid};

#[entry]
fn efi_main(_image: uefi::Handle, st: SystemTable<Boot>) -> Status {
    uefi_services::init(&st).expect_success("UEFI services init failed");
    let stdout = st.stdout();
    let rt = st.runtime_services();

    const NAME_SIZE: usize = 1024;
    let mut name = [NUL_16; NAME_SIZE];
    let mut guid = Guid::from_values(0, 0, 0, 0, [0u8; 6]);
    let mut data = vec![0u8; 1024];

    loop {
        let mut name_size = NAME_SIZE;
        let res =
            unsafe { rt.get_next_variable_name(&mut name_size, name.as_mut_ptr(), &mut guid) };
        if let Err(error) = res {
            if error.status() != Status::NOT_FOUND {
                writeln!(stdout, "GetNextVariable name failed with {:?}", error).unwrap();
            }
            break;
        }

        writeln!(stdout, "GUID={}", guid).unwrap();
        let name_slice = unsafe { CStr16::from_ptr(name.as_ptr()) };
        writeln!(stdout, "NAME={}", efiutils::ucs2_decode(name_slice)).unwrap();

        let mut data_size = data.len();
        let mut attributes = 0u32;
        let res = unsafe {
            rt.get_variable(
                name.as_ptr(),
                &guid,
                &mut attributes,
                &mut data_size,
                data.as_mut_ptr(),
            )
        };
        if let Err(error) = res {
            if error.status() != Status::BUFFER_TOO_SMALL {
                writeln!(stdout, "GetVariable name failed with {:?}", error).unwrap();
                break;
            } else {
                // retry with large buffer
                data.resize(data_size, 0);
                unsafe {
                    rt.get_variable(
                        name.as_ptr(),
                        &guid,
                        &mut attributes,
                        &mut data_size,
                        data.as_mut_ptr(),
                    )
                    .expect_success("GetVariable failed")
                }
            }
        }

        writeln!(
            stdout,
            "ATTR={:?}",
            Attributes::from_bits_truncate(attributes)
        )
        .unwrap();

        let data_slice = &data[..data_size];
        write!(stdout, "DATA=").unwrap();
        for byte in data_slice {
            write!(stdout, "{:02X}", byte).unwrap();
        }
        writeln!(stdout).unwrap();
    }

    Status::SUCCESS
}

bitflags! {
    pub struct Attributes: u32 {
        const NON_VOLATILE = 0x1;
        const BOOTSERVICE_ACCESS = 0x2;
        const RUNTIME_ACCESS = 0x4;
        const READ_ONLY = 0x8;
    }
}
