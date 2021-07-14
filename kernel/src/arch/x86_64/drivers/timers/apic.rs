//**************************************************************************************************
// apic.rs                                                                                         *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::arch::local_apic;
use crate::drivers::timers::Device;
use crate::spinlock::Spinlock;
use core::convert::TryInto;
use core::lazy::OnceCell;
use core::ops::Deref;
use units::Nanoseconds;
use x86::apic::local::{CommonRegisters, DivideValue, TimerLvt, TimerMode};
use x86::msr::ia32_tsc_deadline;
use x86::{apic, cpuid};

pub unsafe fn create_device() -> Option<Device> {
    let (_, _, features_1) = cpuid::leaf_1::read();

    match local_apic::registers().deref() {
        local_apic::Registers::X2Apic(_) => {
            if features_1.tsc_deadline() {
                Some(Device::new(
                    "x2APIC (TSC Deadline)",
                    None,
                    Some(start_tsc_deadline),
                    read_count_tsc_deadline,
                    None,
                ))
            } else {
                Some(Device::new(
                    "x2APIC (One Shot)",
                    None,
                    Some(start_x2apic),
                    read_count_x2apic,
                    None,
                ))
            }
        }
        local_apic::Registers::Apic(_) => {
            if features_1.tsc_deadline() {
                Some(Device::new(
                    "APIC (TSC Deadline)",
                    None,
                    Some(start_tsc_deadline),
                    read_count_tsc_deadline,
                    None,
                ))
            } else {
                Some(Device::new(
                    "APIC (One Shot)",
                    None,
                    Some(start_apic),
                    read_count_apic,
                    None,
                ))
            }
        }
        local_apic::Registers::NotAvailable => None,
    }
}

// Init

pub fn init() {}

// Start

pub fn start_apic(count: Nanoseconds<u64>) {
    let mut registers = local_apic::registers().apic();
    // let value: u32 = count
    //     .try_into()
    //     .expect("Requested time is to too large for x2APIC.");
    // unsafe { registers.write_initial_count_register(value) }
    todo!()
}

pub fn start_x2apic(count: Nanoseconds<u64>) {
    let mut registers = local_apic::registers().x2apic();
    // let value: u32 = count
    //     .try_into()
    //     .expect("Requested time is to too large for APIC.");
    // unsafe { registers.write_initial_count_register(value) }
    todo!()
}

pub fn start_tsc_deadline(count: Nanoseconds<u64>) {
    // unsafe { ia32_tsc_deadline::write(count) }
    todo!()
}

// Read Count

pub fn read_count_apic() -> Nanoseconds<u64> {
    let registers = local_apic::registers().apic();
    // unsafe { registers.read_current_count_register() as u64 }
    todo!()
}

pub fn read_count_x2apic() -> Nanoseconds<u64> {
    let registers = local_apic::registers().x2apic();
    //unsafe { registers.read_current_count_register() as u64 }
    todo!()
}

pub fn read_count_tsc_deadline() -> Nanoseconds<u64> {
    //unsafe { ia32_tsc_deadline::read() }
    todo!()
}

// Calibration

// pub fn run_calibration_countdown() {
//     let mut timer_lvt = TimerLvt::new();
//     timer_lvt.set_timer_mode(TimerMode::ONE_SHOT);
//     timer_lvt.apic_registers.write_lvt_time_register(timer_lvt);
//
//     apic_registers.write_dcr(DivideValue::BY_1);
//
//     apic_registers.write_initial_count_register(u32::MAX);
// }
//
// pub fn run_calibration_countdown_tsc_deadline() {
//     let mut timer_lvt = TimerLvt::new();
//     timer_lvt.set_timer_mode(TimerMode::TSC_DEADLINE);
//     timer_lvt.apic_registers.write_lvt_time_register(timer_lvt);
//
//     apic_registers.write_dcr(DivideValue::BY_1);
//
//     ia32_tsc_deadline::write(u64::MAX);
// }
//
// pub fn calibrate() {}
