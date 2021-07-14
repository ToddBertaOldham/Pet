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
    Spinlock::new(buddy::Allocator::new(VmmAllocatorInterface::new(0)));

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
    offset: usize,
}

impl VmmAllocatorInterface {
    pub const fn new(offset: usize) -> Self {
        Self { offset }
    }
}

unsafe impl AllocatorInterface for VmmAllocatorInterface {
    const PAGE_SIZE: usize = arch::PAGE_SIZE;

    unsafe fn get_pages(&mut self, amount: usize) -> *mut u8 {
        let start_address: *mut u8 = arch::vmm::heap_start().as_mut_ptr();
        let current_address = start_address.add(self.offset);

        arch::vmm::allocate_pages(current_address as usize, amount);
        self.offset += amount * arch::PAGE_SIZE;

        current_address
    }

    unsafe fn return_pages(&mut self, ptr: *mut u8, amount: usize) {
        todo!()
    }
}
