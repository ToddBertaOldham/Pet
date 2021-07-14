//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::drivers::timers::Device;
use alloc::vec::Vec;
use kernel_interface::init::Args;

pub mod apic;
pub mod hpet;
pub mod tsc;

pub unsafe fn create_devices(args: &Args, vec: &mut Vec<Device>) {
    if let Some(apic_device) = apic::create_device() {
        vec.push(apic_device);
    }

    if let Some(tsc_device) = tsc::create_device() {
        vec.push(tsc_device);
    }

    if let Some(hpet_device) = hpet::create_device(args) {
        vec.push(hpet_device);
    }
}
