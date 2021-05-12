//**************************************************************************************************
// pmm.rs                                                                                          *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::arch::vmm;
use crate::frame::Frame;
use crate::spinlock::Spinlock;
use alloc::vec::Vec;
use core::fmt;
use core::ptr;
use kernel_interface::init::{Args, MemoryMap, MemorySection};

static STATE: Spinlock<Option<State>> = Spinlock::new(None);

pub unsafe fn init_stage_one(args: &Args) {
    assert!(!args.memory_map.ptr.is_null(), "Memory map is null.");
    assert_ne!(args.memory_map.len, 0, "Memory map count is 0.");

    let mut state_lock = STATE.lock();

    assert!(state_lock.is_none(), "PMM already initialized");

    for memory_section in args.memory_map.as_slice() {
        let segment = memory_section.as_segment();
        println!(
            "Memory from {:#X} to {:#X} is {:?}.",
            segment.start(),
            segment.end(),
            memory_section.memory_type,
        );
    }

    *state_lock = Some(State {
        memory_map: args.memory_map,
        next: 1,
        free: Vec::new(),
    });

    println!("PMM stage one initialized.");
}

pub unsafe fn init_stage_two() {
    let mut state_lock = STATE.lock();

    let mut state = state_lock.as_mut().expect("PMM not stage one initialized.");

    // Offset memory map physical addresses to use virtual mapping.

    state.memory_map.ptr = vmm::convert_physical_address_mut(state.memory_map.ptr);

    println!("PMM stage two initialized.");
}

pub unsafe fn init_stage_three() {
    let mut state_lock = STATE.lock();

    let mut state = state_lock.as_mut().expect("PMM not stage one initialized.");

    // First allocate a new memory map and copy from the old one.

    let mut new_memory_map = Vec::<MemorySection>::with_capacity(state.memory_map.len);

    new_memory_map.clone_from_slice(state.memory_map.as_slice());

    state.memory_map = MemoryMap::from_vec(new_memory_map);

    // Now reclaim boot memory, ACPI, and other temporary startup memory. This consumes the old
    // memory map area.

    println!("PMM stage three initialized.");
}

pub unsafe fn allocate_frame() -> Frame {
    STATE
        .lock()
        .as_mut()
        .expect("PMM not stage one initialized before allocating.")
        .allocate()
}

pub unsafe fn free_frame(frame: Frame) {
    STATE
        .lock()
        .as_mut()
        .expect("PMM not stage one initialized before freeing.")
        .free(frame);
}

#[derive(Debug)]
struct State {
    memory_map: MemoryMap,
    next: usize,
    free: Vec<Frame>,
}

impl State {
    pub unsafe fn allocate(&mut self) -> Frame {
        if let Some(freed_frame) = self.free.pop() {
            freed_frame
        } else {
            loop {
                let frame = Frame::new(self.next);
                self.next += 1;
                if self.is_free(frame) {
                    ptr::write_bytes(frame.segment().as_mut_ptr::<u8>(), 0, Frame::BYTE_WIDTH);
                    return frame;
                }
            }
        }
    }

    unsafe fn is_free(&self, frame: Frame) -> bool {
        for memory_section in self.memory_map.as_slice() {
            if memory_section.as_segment().intersects(frame.segment()) {
                return memory_section.memory_type.is_usable();
            }
        }

        panic!("Out of physical memory.");
    }

    pub unsafe fn free(&mut self, frame: Frame) {
        self.free.push(frame);
    }
}

unsafe impl Send for State {}

pub struct InitError;

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("Memory map is null or memory map count is 0.")
    }
}
