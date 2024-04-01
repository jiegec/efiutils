#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use alloc::{collections::BTreeSet, vec};
use efiutils::{ucs2_decode, FormBrowser2, HiiDatabase, ShellParameters};
use log::*;
use uefi::table::boot::SearchType;
use uefi::{prelude::*, CStr16, Char16, Guid};

fn main(_image: uefi::Handle, st: SystemTable<Boot>) -> anyhow::Result<()> {
    let bt = st.boot_services();

    let handle = bt
        .locate_handle_buffer(SearchType::from_proto::<HiiDatabase>())
        .expect("Failed to find protocol handle")[0];

    let db = bt
        .open_protocol_exclusive::<HiiDatabase>(handle)
        .expect("Locate hii database protocol failed");

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

    let handle = bt
        .locate_handle_buffer(SearchType::from_proto::<FormBrowser2>())
        .expect("Failed to find protocol handle")[0];

    let browser = bt
        .open_protocol_exclusive::<FormBrowser2>(handle)
        .expect("Locate form browser2 protocol failed");

    let handle = bt
        .locate_handle_buffer(SearchType::from_proto::<ShellParameters>())
        .expect("Failed to find protocol handle")[0];

    let params = bt
        .open_protocol_exclusive::<ShellParameters>(handle)
        .expect("Locate shell parameter protocol failed");

    let mut v = vec![];
    let argv: &[*const Char16] = unsafe { core::slice::from_raw_parts(params.argv, params.argc) };
    if params.argc > 1 {
        info!("Handles: {}", buffer.len());
        for s in &argv[1..] {
            let arg = ucs2_decode(unsafe { CStr16::from_ptr(*s) })?;
            info!("Arg: {}", arg);
            if let Ok(i) = str::parse::<usize>(&arg) {
                let res = (browser.send_form)(
                    &browser,
                    &buffer[i],
                    1,
                    0 as *const Guid,
                    0,
                    0 as *const u8,
                    0 as *mut u8,
                );
                v.push((i, res));
            }
        }
    } else {
        // try handles one by one
        for i in 0..buffer.len() {
            info!("Opening form with handle: {}", buffer[i]);
            let res = (browser.send_form)(
                &browser,
                &buffer[i],
                1,
                0 as *const Guid,
                0,
                0 as *const u8,
                0 as *mut u8,
            );
            v.push((i, res));
        }
    }

    info!("Argc {:?}", params.argc);
    info!("Argv {:?}", params.argv);
    info!("Res {:?}", v);
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
