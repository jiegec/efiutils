#!/bin/bash
rm -rf floppy.img
sudo mkfs.vfat -C floppy.img 1440
mkdir -p floppy
sudo mount -o loop -t vfat floopy.img floppy
sudo cp target/x86_64-unknown-uefi/release/*.efi floppy/
sudo umount floppy
