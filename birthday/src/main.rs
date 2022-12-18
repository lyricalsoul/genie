#![no_main]
#![no_std]
#![feature(abi_efiapi)]

extern crate alloc;

use alloc::format;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi_services::println;
use uefi_helpers::loaded_image::get_loaded_image_path;
use reflection::Display;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let bs = system_table.boot_services();
    println!("welcome to the birthday bootloader");

    println!("opening GraphicsOutput protocol...");
    let gop_handle = bs.get_handle_for_protocol::<GraphicsOutput>().expect("Couldn't get GraphicsOutput handle. Is it an UEFI firmware issue?");
    let gop = bs.open_protocol_exclusive::<GraphicsOutput>(gop_handle).expect("Couldn't open the GraphicsOutput protocol. Is it an UEFI firmware issue?");

    let mut display = Display::new(gop);
    let _ = display.draw_logo();
    // TODO: make display.point a macro
    let next = display.write_bold("birthday ", display.point(2, 0)).unwrap();
    let next = display.write("bootloader - ", next).unwrap();
    let _ = display.write_bold("\"The ReVe Festival\" ", next);

    bs.stall(100_000_000_000);
    Status::SUCCESS
}