#![no_std]
#![no_main]
#![feature(abi_efiapi, negative_impls)]

extern crate alloc;

use alloc::collections::BTreeSet;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Write;
use core::mem::MaybeUninit;
use uefi::proto::Protocol;
use uefi::unsafe_guid;
use uefi::{prelude::*, Guid, Handle, Identify};

pub use uefi::proto;

#[repr(C)]
#[unsafe_guid("ef9fc172-a1b2-4693-b327-6d32fc416042")]
#[derive(Protocol)]
pub struct HiiDatabase {
    new_package_list: extern "efiapi" fn(
        &HiiDatabase,
        package_list: *const u8,
        driver_handle: usize,
        handle: *mut usize,
    ) -> Status,
    remove_package_list: extern "efiapi" fn(&HiiDatabase, handle: usize) -> Status,
    update_package_list:
        extern "efiapi" fn(&HiiDatabase, handle: usize, package_list: *const u8) -> Status,
    list_package_lists: extern "efiapi" fn(
        &HiiDatabase,
        package_type: u8,
        package_guid: *const Guid,
        handle_buffer_length: *mut usize,
        handle: *mut usize,
    ) -> Status,
    export_package_lists: extern "efiapi" fn(
        &HiiDatabase,
        handle: usize,
        buffer_size: *mut usize,
        buffer: *mut u8,
    ) -> Status,
}

fn dump(db: &mut HiiDatabase, handle: usize) -> Vec<u8> {
    let mut buffer_size = 0;
    // EFI_HII_PACKAGE_TYPE_ALL
    let res = (db.export_package_lists)(&db, handle, &mut buffer_size, 0 as *mut u8);
    assert!(res == Status::BUFFER_TOO_SMALL);

    let mut buffer: Vec<u8> = vec![0u8; buffer_size];
    let res = (db.export_package_lists)(&db, handle, &mut buffer_size, buffer.as_mut_ptr());
    assert!(res == Status::SUCCESS);

    buffer
}

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

    let stdout = st.stdout();
    for handle in buffer_set {
        let v = dump(db, handle);

        for byte in v {
            write!(stdout, "{:02X}", byte).unwrap();
        }
        writeln!(stdout).unwrap();
    }

    Status::SUCCESS
}
