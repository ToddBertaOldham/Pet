//**************************************************************************************************
// xapic.rs                                                                                        *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::apic::{LocalApic, TaskPriority};
use memory::Address64;

pub struct XApic {
    address: *mut u8,
}

impl XApic {
    pub unsafe fn new(address: *mut u8) -> Self {
        XApic { address }
    }

    unsafe fn get_register(&self, offset: usize) -> *mut u32 {
        self.address.add(offset) as *mut u32
    }

    pub fn write_local_destination(&mut self, value: u32) {
        unimplemented!()
    }
}

impl LocalApic for XApic {
    fn id(&self) -> u32 {
        unsafe { *self.get_register(0x20) }
    }

    fn version(&self) -> u32 {
        unsafe { *self.get_register(0x30) }
    }

    fn task_priority(&self) -> TaskPriority {
        unsafe { TaskPriority::from(*self.get_register(0x80)) }
    }

    fn write_task_priority(&mut self, task_priority: TaskPriority) {
        unsafe { *self.get_register(0x80) = u32::from(task_priority) }
    }

    fn process_priority(&self) -> u32 {
        unimplemented!()
    }

    fn write_eoi(&mut self, value: u32) {
        unimplemented!()
    }

    fn local_destination(&self) -> u32 {
        unimplemented!()
    }
}
