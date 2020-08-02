//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2018-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub const PAGE_SIZE: usize = 4096;

pub use ::memory::Segment;

#[macro_export]
macro_rules! memory_pool {
    ($size:expr) => {{
        let mut vector = alloc::vec::Vec::<u8>::with_capacity($size);
        vector.resize($size, 0);
        vector.into_boxed_slice()
    }};
}

mod allocator;
mod map;
mod pages;

pub use allocator::*;
pub use map::*;
pub use pages::*;
