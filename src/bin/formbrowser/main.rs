#![no_std]
#![no_main]
#![feature(abi_efiapi, negative_impls)]

extern crate alloc;

use alloc::vec::Vec;
use alloc::{collections::BTreeSet, vec};
use efiutils::{ucs2_decode, FormBrowser2, HiiDatabase, ShellParameters};
use log::*;
use uefi::{prelude::*, CStr16, Char16, Guid};

fn main(image: uefi::Handle, st: SystemTable<Boot>) -> anyhow::Result<()> {
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

    let params = bt
        .handle_protocol::<ShellParameters>(image)
        .expect_success("Locate shell parameter protocol failed");
    let params = unsafe { &mut *params.get() };

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
    uefi_services::init(&mut st).expect_success("UEFI services init failed");

    match main(image, st) {
        Ok(_) => Status::SUCCESS,
        Err(err) => {
            warn!("Error {}", err);
            Status::ABORTED
        }
    }
}
