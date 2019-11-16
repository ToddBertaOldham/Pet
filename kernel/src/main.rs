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
#![feature(optin_builtin_traits)]

extern crate alloc;

#[macro_use]
pub mod arch;
pub mod memory;
mod spinlock;

use crate::spinlock::Spinlock;
use core::alloc::Layout;
use core::panic::PanicInfo;
use kernel_init::KernelArgs;

#[global_allocator]
static ALLOCATOR: Spinlock<memory::Allocator> = Spinlock::new(memory::Allocator::uninitialized());

pub unsafe fn main(args: &'static KernelArgs) -> ! {
    memory::physical_manager::init(&args.memory_info);
    ALLOCATOR.lock().init(0, 2);
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
