//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

mod scheduler;
mod task;

pub use scheduler::*;
pub use task::*;

use crate::spinlock::Spinlock;

pub static SCHEDULER : Spinlock<Option<Scheduler>> = Spinlock::new(None);