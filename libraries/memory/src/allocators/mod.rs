//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub mod buddy;

pub unsafe trait AllocatorInterface {
    const PAGE_SIZE: usize;

    unsafe fn get_pages(&mut self, amount: usize) -> *mut u8;
    unsafe fn return_pages(&mut self, ptr: *mut u8, amount: usize);
}
