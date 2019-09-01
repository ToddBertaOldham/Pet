//**************************************************************************************************
// main.rs                                                                                         *
// Copyright (c) 2018-2019 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(alloc_layout_extra)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
pub mod arch;
pub mod memory;

use core::alloc::Layout;
use core::panic::PanicInfo;

#[global_allocator]
static ALLOCATOR: memory::Allocator = memory::Allocator;

pub fn main_stage_2() -> ! {
    loop {}
}

pub fn print_header() {
    println!("Pet Kernel");
    println!("Copyright (c) 2018-2019 Todd Berta-Oldham");

    if cfg!(debug_assertions) {
        println!("This is a debug build.");
    }
}

#[alloc_error_handler]
fn on_oom(layout: Layout) -> ! {
    println!("Out of memory. {:?}", layout);
    unsafe { arch::stall() }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel panic! (╯‵□′)╯︵┻━┻");
    println!("{}", info);
    unsafe { arch::stall() }
}
