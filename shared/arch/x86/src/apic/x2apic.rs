//**************************************************************************************************
// x2apic.rs                                                                                       *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::apic::{LocalApic, TaskPriority};
use crate::ModelSpecificRegister;
use split::Halves;

pub struct X2Apic;

impl LocalApic for X2Apic {
    fn id(&self) -> u32 {
        unsafe { ModelSpecificRegister::new(0x802).read().lower_half() }
    }

    fn version(&self) -> u32 {
        unsafe { ModelSpecificRegister::new(0x803).read().lower_half() }
    }

    fn task_priority(&self) -> TaskPriority {
        unsafe { ModelSpecificRegister::new(0x808).read().lower_half().into() }
    }

    fn write_task_priority(&mut self, task_priority: TaskPriority) {
        unsafe {
            let value = u64::from_halves(task_priority.into(), 0);
            ModelSpecificRegister::new(0x808).write(value);
        }
    }
}
