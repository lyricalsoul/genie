#![no_main]
#![no_std]
#![feature(abi_efiapi)]

use uefi::prelude::*;
use uefi_services::println;
use uefi_helpers::loaded_image::get_loaded_image_path;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let bs = system_table.boot_services();
    println!("the mahogany kernel");
    println!("booted from {}", &*get_loaded_image_path(bs));
    bs.stall(10_000_000);
    Status::SUCCESS
}