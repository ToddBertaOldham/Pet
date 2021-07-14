//**************************************************************************************************
// apic.rs                                                                                         *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::drivers::ic::Device;
use kernel_interface::init::Args;

pub unsafe fn create_device() -> Option<Device> {
    Some(Device::new("IO APIC", None))
}
