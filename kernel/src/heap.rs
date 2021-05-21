//**************************************************************************************************
// heap.rs                                                                                         *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::arch;
use crate::spinlock::Spinlock;
use core::alloc::{GlobalAlloc, Layout};
use memory::allocators::{buddy, AllocatorInterface};

const LEVELS: usize = 17 - buddy::BASE_LEVEL;

#[global_allocator]
static ALLOCATOR: Spinlock<buddy::Allocator<VmmAllocatorInterface, LEVELS>> =
    Spinlock::new(buddy::Allocator::new(VmmAllocatorInterface::new()));

unsafe impl GlobalAlloc for Spinlock<buddy::Allocator<VmmAllocatorInterface, LEVELS>> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.lock().alloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.lock().dealloc(ptr, layout.size());
    }
}

#[derive(Debug)]
struct VmmAllocatorInterface {
    next: usize,
}

impl VmmAllocatorInterface {
    pub const fn new() -> Self {
        Self {
            next: arch::vmm::HEAP_VIRTUAL_START as usize,
        }
    }
}

unsafe impl AllocatorInterface for VmmAllocatorInterface {
    const PAGE_SIZE: usize = arch::PAGE_SIZE;

    unsafe fn get_pages(&mut self, amount: usize) -> *mut u8 {
        let address = self.next as *mut u8;

        arch::vmm::allocate_pages(address as usize, amount);
        self.next += amount * arch::PAGE_SIZE;

        address
    }

    unsafe fn return_pages(&mut self, ptr: *mut u8, amount: usize) {
        todo!()
    }
}
