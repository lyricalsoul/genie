#!/bin/bash
mkdir -p .qemu/esp/EFI/BOOT
cp target/x86_64-unknown-uefi/debug/birthday.efi .qemu/esp/EFI/BOOT/BOOTX64.efi

qemu-system-x86_64 -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=.qemu/OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=.qemu/OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:.qemu/esp