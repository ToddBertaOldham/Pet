//**************************************************************************************************
// gdt.rs                                                                                          *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::tss;
use core::convert::TryInto;
use x86::descriptors::size_64::{
    code_data_segment,
    code_data_segment::{CodeDescriptorType, DataDescriptorType},
    tss_ldt,
};
use x86::segmentation::{
    load_data_selectors,
    size_64::{load_cs, load_gdt},
};
use x86::{ProtectionRing, Selector};

// GDT mixes 8 byte (1 entry) and 16 byte (2 entries) descriptors so values are stored in
// an 8 byte buffer.
static mut ENTRIES: [u64; 5] = [0; 5];

pub unsafe fn install() {
    println!("Installing GDT...");

    //TODO In future versions of Rust it will likely be possible to make the
    // entries initialization constant.

    // ENTRIES[0] is the null segment and is left at 0;

    let mut kernel_code = code_data_segment::Descriptor::default();
    kernel_code.set_is_present(true);
    kernel_code.set_privilege_level(ProtectionRing::Level0);
    kernel_code.set_descriptor_type(code_data_segment::DescriptorType::LongCode(
        CodeDescriptorType::ExecuteRead,
    ));
    ENTRIES[1] = u64::from(kernel_code);

    let mut kernel_data = code_data_segment::Descriptor::default();
    kernel_data.set_is_present(true);
    kernel_data.set_privilege_level(ProtectionRing::Level0);
    kernel_data.set_descriptor_type(code_data_segment::DescriptorType::Data(
        DataDescriptorType::ReadWrite,
    ));
    ENTRIES[2] = u64::from(kernel_data);

    let mut user_code = code_data_segment::Descriptor::default();
    user_code.set_is_present(true);
    user_code.set_privilege_level(ProtectionRing::Level3);
    user_code.set_descriptor_type(code_data_segment::DescriptorType::LongCode(
        CodeDescriptorType::ExecuteRead,
    ));
    ENTRIES[3] = u64::from(user_code);

    let mut user_data = code_data_segment::Descriptor::default();
    user_data.set_is_present(true);
    user_data.set_privilege_level(ProtectionRing::Level3);
    user_data.set_descriptor_type(code_data_segment::DescriptorType::Data(
        DataDescriptorType::ReadWrite,
    ));
    ENTRIES[4] = u64::from(user_data);

    let mut tss = tss_ldt::Descriptor::default();
    tss.set_is_present(true);
    tss.set_privilege_level(ProtectionRing::Level0);
    tss.set_descriptor_type(tss_ldt::DescriptorType::TssAvailable);
    tss.set_base_address(tss::offset());

    load_gdt(&ENTRIES[..].try_into().expect("GDT too large."));
    load_cs(kernel_code_selector());
    load_data_selectors(kernel_data_selector());

    println!("GDT installed.");
}

pub fn kernel_code_selector() -> Selector {
    Selector::new(1, false, ProtectionRing::Level0)
}

pub fn kernel_data_selector() -> Selector {
    Selector::new(2, false, ProtectionRing::Level0)
}

pub fn user_code_selector() -> Selector {
    Selector::new(3, false, ProtectionRing::Level0)
}

pub fn user_data_selector() -> Selector {
    Selector::new(4, false, ProtectionRing::Level0)
}
