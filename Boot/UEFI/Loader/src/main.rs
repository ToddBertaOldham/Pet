// *************************************************************************
// main.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]
#![no_main]

use uefi_core::{Handle, Status, SystemTable, printrln };
use core::fmt::Write;
use core::panic::PanicInfo;

#[no_mangle]
pub unsafe extern "win64" fn efi_main(image_handle : Handle, system_table : *mut SystemTable) -> Status {
    uefi_core::init(image_handle, system_table);
    main();
    Status::Success    
}

fn main() {    
    {
        // Initialize graphics, print header, and then print graphics info.

        let provider = uefi_core::graphics_output_provider();

        for index in 0..provider.count() {
            provider.get(index).maximize(true);
        }

        printrln!("Pet UEFI Boot Loader").unwrap();
        printrln!("Copyright 2019 Todd Berta-Oldham").unwrap();

        if cfg!(debug_assertions) {
            printrln!("This is a debug build.").unwrap();
        }

        for index in 0..provider.count() {
            let output = provider.get(index);
            match output.framebuffer_address() {
                Some(address) => printrln!("Graphics output {} initialized at address {:#X} with {}x{} resolution.", index, address, output.width(), output.height()).unwrap(),
                None => printrln!("Graphics output {} could not be initialized with a linear framebuffer.", index).unwrap()
            }
        }
    }

    loop { }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}