//**************************************************************************************************
// local_apic.rs                                                                                   *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::arch::vmm;
use crate::spinlock::{Spinlock, SpinlockGuard};
use crate::AcpiInterface;
use acpi::madt::MadtEntry;
use acpi::RootEntry;
use core::lazy::OnceCell;
use kernel_interface::init::Args;
use x86::msr::ia32_apic_base;
use x86::{apic, cpuid};

//TODO How should registers safely be stored and accessed from multiple CPUs?
pub static REGISTERS: Spinlock<Registers> = Spinlock::new(Registers::NotAvailable);

pub unsafe fn init(args: &Args) {
    let (_, _, features_1) = cpuid::leaf_1::read();

    if !features_1.apic() {
        return;
    }

    if features_1.x2apic() {
        init_x2apic(args);
    } else {
        init_apic(args);
    }
}

unsafe fn init_apic(args: &Args) {
    // Try to find MADT by checking XSDT first and then RSDT.

    let acpi_interface = AcpiInterface;

    let madt_result = args
        .system_info
        .iter_acpi(&acpi_interface)
        .find_map(|entry| match entry {
            RootEntry::Madt(mdt_ptr) => Some(mdt_ptr),
            _ => None,
        });

    // Find MMIO address for APIC/xAPIC.

    let mut apic_ptr;

    if let Some(madt_ptr) = madt_result {
        let madt = &mut *madt_ptr;

        // Prefer 64-bit local APIC address override from MADT entries if available.
        // Use 32-bit MADT address otherwise.

        if let Some(address_override_ptr) = madt.iter().find_map(|entry| match entry {
            MadtEntry::LocalApicAddressOverride(address_override_ptr) => Some(address_override_ptr),
            _ => None,
        }) {
            let address_override = &*address_override_ptr;
            apic_ptr = address_override.address.as_mut_ptr();
        } else {
            apic_ptr = madt.lic_address.as_mut_ptr();
        }
    } else {
        // No options left. Use base MSR for address.
        apic_ptr = ia32_apic_base::read().address().as_mut_ptr();
    }

    // Create device.

    apic_ptr = vmm::convert_physical_ptr_mut(apic_ptr);

    let apic = apic::local::Registers::new(apic_ptr);

    *REGISTERS.lock() = Registers::Apic(apic);
}

unsafe fn init_x2apic(args: &Args) {
    let mut base_value = ia32_apic_base::read();
    base_value.set_x2apic_enabled(true);
    ia32_apic_base::write(base_value);

    *REGISTERS.lock() = Registers::X2Apic(apic::local::X2Registers)
}

pub fn registers() -> SpinlockGuard<'static, Registers> {
    REGISTERS.lock()
}

pub enum Registers {
    NotAvailable,
    Apic(apic::local::Registers),
    X2Apic(apic::local::X2Registers),
}

impl Registers {
    pub fn apic(&mut self) -> &mut apic::local::Registers {
        match self {
            Registers::Apic(registers) => registers,
            _ => panic!("APIC registers not available."),
        }
    }

    pub fn x2apic(&mut self) -> &mut apic::local::X2Registers {
        match self {
            Registers::X2Apic(registers) => registers,
            _ => panic!("APIC registers not available."),
        }
    }
}

unsafe impl Send for Registers {}
