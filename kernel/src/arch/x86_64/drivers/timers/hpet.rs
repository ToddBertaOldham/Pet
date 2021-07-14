//**************************************************************************************************
// hpet.rs                                                                                         *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::drivers::timers::Device;
use crate::AcpiInterface;
use acpi::RootEntry;
use hpet::Registers;
use kernel_interface::init::Args;
use units::Nanoseconds;

pub unsafe fn create_device(args: &Args) -> Option<Device> {
    let acpi_interface = AcpiInterface;

    // Search for HPET descriptor tables in ACPI (max of 8).

    let mut hpet_iter =
        args.system_info
            .iter_acpi(&acpi_interface)
            .filter_map(|entry| match entry {
                RootEntry::Hpet(hpet_ptr) => Some(hpet_ptr),
                _ => None,
            });

    Some(Device::new("HPET", None, Some(start), read_count, None))
}

pub fn start(count: Nanoseconds<u64>) {}

pub fn read_count() -> Nanoseconds<u64> {
    unimplemented!()
}
