//**************************************************************************************************
// interrupt_controller.rs                                                                         *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use x86::apic;
use x86::cpuid;

use crate::arch::vmm;
use crate::spinlock::Spinlock;
use acpi::madt::MadtEntry;
use acpi::{RootEntry, Rsdt, Xsdt};
use core::convert::TryInto;
use core::ptr;
use kernel_init::Args;
use memory::Address64;
use x86::apic::{LocalApicCommon, LocalX2apic, LocalXapic};

static DEVICE: Spinlock<Device> = Spinlock::new(Device::None);

pub unsafe fn init(args: &kernel_init::Args) {
    let device = DEVICE.lock();

    assert!(
        device.is_none(),
        "Interrupt controller has already been initialized."
    );

    let (_, _, features) = cpuid::leaf_1::read();

    if features.apic() {
        let mut base_value = apic::base::read();

        if features.x2apic() {
            base_value.set_x2apic_enabled(true);
            *device = Device::X2apic(LocalX2apic);
            println!("Interrupt controller initialized in x2APIC mode.");
        } else {
            let mut xapic_base_address = find_xapic_base_address(args);

            if xapic_base_address.is_null() {}

            let xapic_ptr = vmm::convert_physical_address_mut(xapic_base_address.as_mut_ptr());
            *device = Device::Xapic(LocalXapic::new(xapic_ptr));

            println!("Interrupt controller initialized in xAPIC mode.");
        }

        apic::base::write(base_value);
    } else {
        panic!("APIC is required for now.");
    }
}

pub fn device() {}

unsafe fn find_xapic_base_address(args: &kernel_init::Args) -> Address64 {
    let mut address = Address64::null();

    iter_acpi_entries(args, |root_entry| {
        if let RootEntry::Madt(madt_ptr) = root_entry {
            if let Some(madt) = madt_ptr.as_mut() {
                for madt_entry in madt.iter_interrupt_controllers() {
                    if let MadtEntry::LocalApicAddress(address_table_ptr) = madt_entry {
                        if let Some(address_table) = address_table_ptr.as_mut() {
                            address = address_table.address;
                        }
                    }
                }
            }
        }
    });

    address
}

unsafe fn iter_acpi_entries<T: Fn(RootEntry)>(args: &kernel_init::Args, function: T) {
    let xsdt_ptr = args.configuration.xsdt.as_mut_ptr::<Xsdt>();
    let rsdt_ptr = args.configuration.rsdt.as_mut_ptr::<Rsdt>();

    if let Some(xsdt) = vmm::convert_physical_address_mut(xsdt_ptr).as_mut() {
        for entry in xsdt.entry_iter() {
            function(entry);
        }
    } else if let Some(rsdt) = vmm::convert_physical_address_mut(rsdt_ptr).as_mut() {
        for entry in rsdt.entry_iter() {
            function(entry);
        }
    }
}

pub fn send_sipis() {}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Device {
    None,
    X2apic(LocalX2apic),
    Xapic(LocalXapic),
}

impl Device {
    pub fn is_none(self) -> bool {
        match self {
            Device::None => true,
            _ => false,
        }
    }
}
