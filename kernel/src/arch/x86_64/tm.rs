//**************************************************************************************************
// tm.rs                                                                                           *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::arch::ic;
use kernel_interface::init::{Args, SystemInfo};
use x86::apic;

pub fn init(args: &Args) {
    match ic::device() {
        ic::Device::X2apic(x2apic) => int_apic_timer(x2apic),
        ic::Device::Xapic(xapic) => int_apic_timer(xapic),
        _ => {}
    }
}

pub fn init_aps() {}

fn int_apic_timer<T: apic::local::CommonRegisters>(apic_registers: T) {}

#[derive(Copy, Clone, Debug)]
pub enum ScheduleTimer {
    None,
    X2apic(apic::x2apic::Registers),
    Xapic(apic::xapic::Registers),
}

pub enum GeneralPurposeTimer {}
