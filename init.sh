#!/bin/bash
dd if=/dev/zero of=esp.img count=32 bs=1M
sudo mkfs.vfat esp.img