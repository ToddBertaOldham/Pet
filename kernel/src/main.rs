// *************************************************************************
// main.rs
// Copyright 2018-2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]
#![no_main]

#![feature(abi_x86_interrupt)]
#![feature(asm)]

#[macro_use]
mod arch;

use core::panic::PanicInfo;

pub fn main_stage_2() -> ! {
    loop { }
}

pub fn print_header() {
    println!("Pet Kernel");
    println!("Copyright 2018-2019 Todd Berta-Oldham");

    if cfg!(debug_assertions) {
        println!("This is a debug build.");
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}