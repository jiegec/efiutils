#![no_std]
#![no_main]
#![feature(abi_efiapi, negative_impls)]

extern crate alloc;

use log::*;
use alloc::vec::Vec;
use alloc::{collections::BTreeSet, vec};
use efiutils::{FormBrowser2, HiiDatabase};
use uefi::{prelude::*, Guid};

#[entry]
fn efi_main(_image: uefi::Handle, st: SystemTable<Boot>) -> Status {
    uefi_services::init(&st).expect_success("UEFI services init failed");
    let bt = st.boot_services();

    let db = bt
        .locate_protocol::<HiiDatabase>()
        .expect_success("Locate hii database protocol failed");
    let db = unsafe { &mut *db.get() };

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
    let buffer: Vec<usize> = buffer_set.into_iter().collect();

    let browser = bt
        .locate_protocol::<FormBrowser2>()
        .expect_success("Locate form browser2 protocol failed");
    let browser = unsafe { &mut *browser.get() };
    let res = (browser.send_form)(
        &browser,
        buffer.as_ptr(),
        buffer.len(),
        0 as *const Guid,
        0,
        0 as *const u8,
        0 as *mut u8,
    );
    info!("Res {:?}", res);

    Status::SUCCESS
}
