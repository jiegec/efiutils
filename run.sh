#!/bin/bash
set -e
set -x
cargo build
mkdir -p esp/EFI/Boot
cp target/x86_64-unknown-uefi/debug/*.efi esp/
if [ ! -f OVMF_VARS.fd ]; then
    dd if=/dev/zero of=OVMF_VARS.fd bs=1048576 count=1
    dd if=/usr/share/OVMF/OVMF_VARS.fd of=OVMF_VARS.fd conv=notrunc
fi
qemu-system-x86_64 -drive file=/usr/share/OVMF/OVMF_CODE.fd,format=raw,if=pflash,readonly=on -drive format=raw,file=fat:rw:esp -nographic -drive file=./OVMF_VARS.fd,format=raw,if=pflash
