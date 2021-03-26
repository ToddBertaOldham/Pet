//**************************************************************************************************
// gdt.rs                                                                                          *
// Copyright (c) 2019-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::tss;
use core::convert::TryInto;
use x86::segmentation::{
    load_data_selectors, segment,
    size_64::{load_cs, load_gdt},
};
use x86::tasks::size_64::tss_ldt;
use x86::{ProtectionRing, Selector};

// GDT mixes 8 byte (1 entry) and 16 byte (2 entries) descriptors so values are stored in
// an 8 byte buffer.
static mut ENTRIES: [u64; 5] = [0; 5];

pub unsafe fn install() {
    // ENTRIES[0] is the null segment and is left at 0;

    let mut kernel_code = segment::Descriptor::new();
    kernel_code.set_is_present(true);
    kernel_code.set_privilege_level(ProtectionRing::Level0);
    kernel_code.set_descriptor_type(segment::DescriptorType::LongCode(
        segment::CodeDescriptorType::ExecuteRead,
    ));
    ENTRIES[1] = kernel_code.into();

    let mut kernel_data = segment::Descriptor::new();
    kernel_data.set_is_present(true);
    kernel_data.set_privilege_level(ProtectionRing::Level0);
    kernel_data.set_descriptor_type(segment::DescriptorType::Data(
        segment::DataDescriptorType::ReadWrite,
    ));
    ENTRIES[2] = kernel_data.into();

    let mut user_code = segment::Descriptor::new();
    user_code.set_is_present(true);
    user_code.set_privilege_level(ProtectionRing::Level3);
    user_code.set_descriptor_type(segment::DescriptorType::LongCode(
        segment::CodeDescriptorType::ExecuteRead,
    ));
    ENTRIES[3] = user_code.into();

    let mut user_data = segment::Descriptor::new();
    user_data.set_is_present(true);
    user_data.set_privilege_level(ProtectionRing::Level3);
    user_data.set_descriptor_type(segment::DescriptorType::Data(
        segment::DataDescriptorType::ReadWrite,
    ));
    ENTRIES[4] = user_data.into();

    let mut tss = tss_ldt::Descriptor::new();
    tss.set_is_present(true);
    tss.set_privilege_level(ProtectionRing::Level0);
    tss.set_descriptor_type(tss_ldt::DescriptorType::TssAvailable);
    tss.set_base_address(tss::offset());

    load_gdt(&ENTRIES[..].try_into().unwrap());
    load_cs(kernel_code_selector());
    load_data_selectors(kernel_data_selector());

    println!("GDT installed.");
}

pub fn kernel_code_selector() -> Selector {
    Selector::with_values(1, false, ProtectionRing::Level0)
}

pub fn kernel_data_selector() -> Selector {
    Selector::with_values(2, false, ProtectionRing::Level0)
}

pub fn user_code_selector() -> Selector {
    Selector::with_values(3, false, ProtectionRing::Level0)
}

pub fn user_data_selector() -> Selector {
    Selector::with_values(4, false, ProtectionRing::Level0)
}
