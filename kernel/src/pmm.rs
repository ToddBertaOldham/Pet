//**************************************************************************************************
// pmm.rs                                                                                          *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::frame::Frame;
use crate::spinlock::Spinlock;
use alloc::vec::Vec;
use core::fmt;
use kernel_init;

static STATE: Spinlock<Option<State>> = Spinlock::new(None);

pub unsafe fn init(info: &kernel_init::MemoryInfo) {
    assert!(!info.memory_map.is_null(), "Memory map is null.");

    let mut state = STATE.lock();

    assert!(state.is_none(), "PMM has already been initialized.");

    println!("Initializing PMM...");

    println!("Provided memory map:");
    for index in 0..info.memory_map_count {
        let entry = &*info.memory_map.add(index);
        let segment = entry.segment();
        println!(
            "{}: Start: {:#X} End: {:#X} Type: {:?}",
            index,
            segment.start(),
            segment.end(),
            entry.entry_type()
        );
    }

    *state = Some(State::new_unchecked(*info));
    println!("PMM initialized.");
}

pub unsafe fn allocate_frame() -> Frame {
    STATE
        .lock()
        .as_mut()
        .expect("Physical memory manager was not initialized before allocating.")
        .allocate()
}

pub unsafe fn free_frame(frame: Frame) {
    STATE
        .lock()
        .as_mut()
        .expect("Physical memory manager was not initialized before freeing.")
        .free(frame);
}

#[derive(Debug)]
struct State {
    memory_info: kernel_init::MemoryInfo,
    next: usize,
    free: Vec<Frame>,
}

impl State {
    pub fn new(memory_info: kernel_init::MemoryInfo) -> Result<Self, InitError> {
        if memory_info.memory_map.is_null() || memory_info.memory_map_count == 0 {
            return Err(InitError);
        }

        Ok(Self::new_unchecked(memory_info))
    }

    pub fn new_unchecked(memory_info: kernel_init::MemoryInfo) -> Self {
        Self {
            memory_info,
            next: 0,
            free: Vec::new(),
        }
    }

    pub unsafe fn allocate(&mut self) -> Frame {
        if let Some(freed_frame) = self.free.pop() {
            freed_frame
        } else {
            loop {
                let frame = Frame::new(self.next);
                if self.is_free(frame) {
                    return frame;
                }
                self.next += 1;
            }
        }
    }

    unsafe fn is_free(&self, frame: Frame) -> bool {
        if self
            .memory_info
            .kernel_physical_segment()
            .intersects(frame.segment())
        {
            return false;
        }

        if self
            .memory_info
            .memory_map_segment()
            .intersects(frame.segment())
        {
            return false;
        }

        self.check_memory_map(frame)
    }

    unsafe fn check_memory_map(&self, frame: Frame) -> bool {
        for index in 0..self.memory_info.memory_map_count {
            let entry = &*self.memory_info.memory_map.add(index);
            if entry.segment().intersects(frame.segment()) {
                return entry.entry_type().is_usable();
            }
        }

        panic!("Out of physical memory.");
    }

    pub unsafe fn free(&mut self, frame: Frame) {
        self.free.push(frame);
    }
}

pub struct InitError;

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("Memory map is null or memory map count is 0.")
    }
}
