//**************************************************************************************************
// local.rs                                                                                        *
// Copyright (c) 2020-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

mod common;

pub use common::*;

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
