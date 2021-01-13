//**************************************************************************************************
// interrupt_controller.rs                                                                         *
// Copyright (c) 2021 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use x86::apic;
use x86::cpuid;

use crate::spinlock::Spinlock;
use acpi::madt::MadtEntry;
use acpi::{RootEntry, Xsdt};
use core::convert::TryInto;
use core::ptr;
use kernel_init::Args;
use memory::Address64;
use x86::apic::{LocalApicCommon, LocalX2apic, LocalXapic};

static DEVICE: Spinlock<Device> = Spinlock::new(Device::None);

pub unsafe fn init(args: &kernel_init::Args) {
    let device = DEVICE.lock();

    let (_, _, features) = cpuid::leaf_1::read();

    if features.apic() {
        let mut base_value = apic::base::read();

        if features.x2apic() {
            base_value.set_x2apic_enabled(true);
            *device = Device::X2apic(LocalX2apic);
        } else {
            let mut xapic_base_address = find_xapic_base_address(args);
            if (xapic_base_address.is_null()) {}
            *device = Device::Xapic(LocalXapic::new(xapic_base_address));
        }

        apic::base::write(base_value);
    } else {
        panic!("APIC is required for now.");
    }
}

pub fn device() {}

unsafe fn find_xapic_base_address(args: &kernel_init::Args) -> Address64 {
    for root_entry in args.configuration_info.iter_acpi_entries() {
        if let RootEntry::Madt(madt_ptr) = root_entry {
            if let Some(madt) = madt_ptr.as_mut() {
                for madt_entry in madt.iter_interrupt_controllers() {
                    if let MadtEntry::LocalApicAddress(address_table_ptr) = madt_entry {
                        if let Some(address_table) = address_table_ptr.as_mut() {
                            address_table.address
                        }
                    }
                }
            }
        }
    }
    Address64::null()
}

pub fn send_sipis() {}

pub enum Device {
    None,
    X2apic(LocalX2apic),
    Xapic(LocalXapic),
}
