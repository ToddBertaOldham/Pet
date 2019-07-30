// *************************************************************************
// gdt.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use x86::ProtectionRing;
use x86::segmentation::{ Descriptor, DescriptorType, CodeDescriptorType, DataDescriptorType, 
    Selector, size_64::{ load_gdt, load_cs }, load_data_selectors };
use core::convert::TryInto;

static mut TABLE : [Descriptor; 5] = [Descriptor::new(); 5];

pub unsafe fn install() {
    println!("Installing GDT...");

    //TODO In future versions of Rust it will likely be possible to make the descriptor initialization constant.

    // Kernel code.
    TABLE[1].set_is_present(true);
    TABLE[1].set_is_long(true);
    TABLE[1].set_privilege_level(ProtectionRing::Level0);
    TABLE[1].set_descriptor_type(DescriptorType::Code(CodeDescriptorType::ExecuteRead));

    // Kernel data.
    TABLE[2].set_is_present(true);
    TABLE[2].set_privilege_level(ProtectionRing::Level0);
    TABLE[2].set_descriptor_type(DescriptorType::Data(DataDescriptorType::ReadWrite));

    // User code.
    TABLE[3].set_is_present(true);
    TABLE[3].set_is_long(true);
    TABLE[3].set_privilege_level(ProtectionRing::Level3);
    TABLE[3].set_descriptor_type(DescriptorType::Code(CodeDescriptorType::ExecuteRead));

    // User data.
    TABLE[4].set_is_present(true);
    TABLE[4].set_privilege_level(ProtectionRing::Level3);
    TABLE[4].set_descriptor_type(DescriptorType::Data(DataDescriptorType::ReadWrite));

    load_gdt(&TABLE[..].try_into().expect("GDT too large."));
    load_cs(kernel_code_selector());
    load_data_selectors(kernel_data_selector());

    println!("GDT installed successfully.");
}

pub fn kernel_code_selector() -> Selector { Selector::new(1, false, ProtectionRing::Level0) }

pub fn kernel_data_selector() -> Selector { Selector::new(2, false, ProtectionRing::Level0) }