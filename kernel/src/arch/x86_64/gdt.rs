// *************************************************************************
// gdt.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use x86::size_64::segmentation;
use core::convert::TryInto;

static TABLE : [segmentation::Descriptor; 5] = [
    segmentation::Descriptor::new(),
    segmentation::Descriptor::new(),
    segmentation::Descriptor::new(),
    segmentation::Descriptor::new(),
    segmentation::Descriptor::new()
];

pub unsafe fn install() {
    println!("Installing GDT...");
    let gdt = &TABLE[..].try_into().expect("GDT too large.");
    segmentation::load_gdt(gdt);
    println!("GDT installed.");
}