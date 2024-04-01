#![no_std]
#![no_main]

extern crate alloc;

use alloc::collections::BTreeSet;
use alloc::vec;
use alloc::vec::Vec;
use efiutils::{HiiDatabase, WithContext};
use log::warn;
use uefi::{prelude::*, table::boot::SearchType, Guid};
use uefi_services::{print, println};

fn main(_image: uefi::Handle, st: SystemTable<Boot>) -> efiutils::Result<()> {
    let bt = st.boot_services();

    let handle = bt
        .locate_handle_buffer(SearchType::from_proto::<HiiDatabase>())
        .with_context("Failed to find protocol handle")?[0];

    let db = bt
        .open_protocol_exclusive::<HiiDatabase>(handle)
        .with_context("Locate hii database protocol failed")?;

    let mut buffer_size = 0;
    // EFI_HII_PACKAGE_TYPE_ALL
    let res = (db.list_package_lists)(&db, 0, 0 as *const Guid, &mut buffer_size, 0 as *mut usize);
    assert!(res == Status::BUFFER_TOO_SMALL);

    let mut buffer = vec![0usize; buffer_size];
    let res = (db.list_package_lists)(
        &db,
        0,
        0 as *const Guid,
        &mut buffer_size,
        buffer.as_mut_ptr(),
    );
    assert!(res == Status::SUCCESS);

    // unique
    let mut buffer_set = BTreeSet::new();
    buffer_set.extend(buffer);

    for handle in buffer_set {
        let mut buffer_size = 0;
        // EFI_HII_PACKAGE_TYPE_ALL
        let res = (db.export_package_lists)(&db, handle, &mut buffer_size, 0 as *mut u8);
        assert!(res == Status::BUFFER_TOO_SMALL);

        let mut v: Vec<u8> = vec![0u8; buffer_size];
        let res = (db.export_package_lists)(&db, handle, &mut buffer_size, v.as_mut_ptr());
        assert!(res == Status::SUCCESS);

        for byte in v {
            print!("{:02X}", byte);
        }
        println!();
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
