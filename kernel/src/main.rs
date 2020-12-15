//**************************************************************************************************
// main.rs                                                                                         *
// Copyright (c) 2018-2020 Aurora Berta-Oldham                                                     *
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
pub mod arch;
pub mod memory;
mod spinlock;
pub mod tasks;

use crate::spinlock::Spinlock;
use core::alloc::Layout;
use core::panic::PanicInfo;
use kernel_init;

#[global_allocator]
static ALLOCATOR: Spinlock<memory::Allocator> = Spinlock::new(memory::Allocator::uninitialized());

pub unsafe fn main(args: &'static kernel_init::Args) -> ! {
    memory::physical::init(&args.memory_info);

    println!("Initializing allocator");

    ALLOCATOR.lock().init(
        arch::KERNEL_VIRTUAL_START + args.memory_info.kernel_length,
        2,
    );

    println!("Allocator initialized");

    loop {}
}

pub fn print_header() {
    println!("Verdure Kernel");
    println!("Copyright (c) 2018-2020 Aurora Berta-Oldham");

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
