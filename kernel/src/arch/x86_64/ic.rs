//**************************************************************************************************
// ic.rs                                                                                           *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::arch::vmm;
use crate::spinlock::Spinlock;
use acpi::madt::{Madt, MadtEntry};
use acpi::RootEntry;
use core::ptr;
use kernel_interface::init::{Args, SystemInfo};
use memory::Address64;
use x86::apic::local::CommonRegisters;
use x86::cpuid;
use x86::{apic, PhysicalAddress52};

static DEVICE: Spinlock<Device> = Spinlock::new(Device::None);

pub unsafe fn init(args: &Args) {
    assert!(DEVICE.lock().is_none(), "IC has already been initialized.");

    if init_with_apic_device(args) {
        return;
    }

    panic!("Failed to initialize IC.");
}

unsafe fn init_with_apic_device(args: &Args) -> bool {
    // Check the capabilities of the system first. Mainly ensure that APIC is supported.

    let (_, _, features) = cpuid::leaf_1::read();

    if !features.apic() {
        return false;
    }

    // Create device.

    if features.x2apic() {
        println!("Using x2APIC for IC.");
        init_with_x2apic();
    } else {
        println!("Using xAPIC for IC.");
        init_with_xapic(args);
    };

    println!("IC initialized.");

    //TODO Start APs. See if that needs to be part of another method or can be added here.

    return true;
}

unsafe fn init_with_x2apic() {
    // Enable x2APIC using base MSR and then create device.

    let mut base_value = apic::base::read();

    base_value.set_x2apic_enabled(true);

    apic::base::write(base_value);

    *DEVICE.lock() = Device::X2apic(apic::x2apic::Registers);
}

unsafe fn init_with_xapic(args: &Args) {
    // Try to find MADT by checking XSDT first and then RSDT.

    let acpi_interface = AcpiInterface;

    let madt_result = args
        .system_info
        .iter_acpi(&acpi_interface)
        .find_map(|entry| match entry {
            RootEntry::Madt(mdt_ptr) => Some(mdt_ptr),
            _ => None,
        });

    // Find MMIO address for xAPIC.

    let mut xapic_ptr = ptr::null_mut();

    if let Some(madt_ptr) = madt_result {
        let madt = &mut *madt_ptr;

        // Prefer 64-bit local APIC address override from MADT entries if available.
        // Use 32-bit MADT address otherwise.

        if let Some(address_override_ptr) = madt.iter().find_map(|entry| match entry {
            MadtEntry::LocalApicAddressOverride(address_override_ptr) => Some(address_override_ptr),
            _ => None,
        }) {
            let address_override = &*address_override_ptr;
            xapic_ptr = address_override.address.as_mut_ptr();
        } else {
            xapic_ptr = madt.lic_address.as_mut_ptr();
        }
    } else {
        // No options left. Use base MSR for xAPIC address.

        xapic_ptr = apic::base::read().address().as_mut_ptr();
    }

    // Create device.

    xapic_ptr = vmm::convert_physical_ptr_mut(xapic_ptr);

    let xapic = apic::xapic::Registers::new(xapic_ptr);

    *DEVICE.lock() = Device::Xapic(xapic);
}

pub fn device() -> Device {
    *DEVICE.lock()
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Device {
    None,
    X2apic(apic::x2apic::Registers),
    Xapic(apic::xapic::Registers),
}

impl Device {
    pub fn is_none(self) -> bool {
        match self {
            Device::None => true,
            _ => false,
        }
    }
}

unsafe impl Send for Device {}

//TODO Move

struct AcpiInterface;

impl acpi::Interface for AcpiInterface {}
