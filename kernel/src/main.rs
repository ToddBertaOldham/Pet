//**************************************************************************************************
// main.rs                                                                                         *
// Copyright (c) 2018-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(alloc_layout_extra)]
#![feature(alloc_error_handler)]
#![feature(optin_builtin_traits)]
#![feature(negative_impls)]

extern crate alloc;

#[macro_use]
mod arch;
mod frame;
mod heap;
mod pmm;
mod spinlock;
mod tasks;

use crate::spinlock::Spinlock;
use core::alloc::Layout;
use core::panic::PanicInfo;
use kernel_init;

pub unsafe fn main(args: &'static kernel_init::Args) -> ! {
    loop {}
}

pub unsafe fn main_ap() -> ! {
    loop {}
}

pub fn print_header() {
    println!("Verdure OS Kernel");
    println!("Copyright (c) 2018-2021 The Verdure Project");

    if cfg!(debug_assertions) {
        println!("This is a debug build.");
    }
}

#[alloc_error_handler]
fn on_oom(layout: Layout) -> ! {
    println!("Kernel heap has run out of memory.");
    println!("{:?}", layout);
    unsafe { arch::stall() }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel panic! (╯‵□′)╯︵┻━┻");
    println!("{}", info);
    unsafe { arch::stall() }
}
