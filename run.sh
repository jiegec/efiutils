#!/bin/bash
set -e
cargo build
mkdir -p esp/EFI/Boot
cp target/x86_64-unknown-uefi/debug/efivars.efi esp/EFI/Boot/BootX64.efi
qemu-system-x86_64 -bios /usr/share/edk2-ovmf/x64/OVMF.fd -drive format=raw,file=fat:rw:esp -nographic