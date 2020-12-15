//**************************************************************************************************
// local.rs                                                                                        *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub trait LocalApic {
    fn id(&self) -> u32;
    fn version(&self) -> u32;
    fn task_priority(&self) -> TaskPriority;
    fn write_task_priority(&mut self, value: TaskPriority);
    fn process_priority(&self) -> u32;
    fn write_eoi(&mut self, value: u32);
    fn local_destination(&self) -> u32;
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct TaskPriority(u32);

impl From<u32> for TaskPriority {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<TaskPriority> for u32 {
    fn from(value: TaskPriority) -> Self {
        value.0
    }
}
