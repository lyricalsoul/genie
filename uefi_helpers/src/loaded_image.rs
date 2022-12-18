use uefi::prelude::*;
use uefi::proto::device_path::text::{AllowShortcuts, DevicePathToText, DisplayOnly, PoolString};
use uefi::proto::loaded_image::LoadedImage;
use uefi::table::boot::SearchType;
use uefi::Identify;

pub fn get_loaded_image_path(boot_services: &BootServices) -> PoolString<'_> {
    let loaded_image = boot_services
        .open_protocol_exclusive::<LoadedImage>(boot_services.image_handle())
        .expect("Couldn't open LoadedImage protocol. Is it an UEFI firmware issue?");

    let device_path_to_text_handle = *boot_services
        .locate_handle_buffer(SearchType::ByProtocol(&DevicePathToText::GUID))
        .expect("Couldn't locate any UEFI handle supporting DevicePathToText::GUID. Is it an UEFI firmware issue?")
        .handles()
        .first()
        .expect("DevicePathToText is missing");

    let device_path_to_text = boot_services
        .open_protocol_exclusive::<DevicePathToText>(
            device_path_to_text_handle,
        )
        .expect("Couldn't open DevicePathToText protocol. Is it an UEFI firmware issue?");

    let image_device_path =
        loaded_image.file_path().expect("File path is not set");

    let text = device_path_to_text
        .convert_device_path_to_text(
            boot_services,
            image_device_path,
            DisplayOnly(true),
            AllowShortcuts(false),
        )
        .expect("convert_device_path_to_text failed");

    text
}