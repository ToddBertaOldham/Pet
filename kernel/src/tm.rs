//**************************************************************************************************
// tm.rs                                                                                           *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::arch::drivers::timers::create_devices as create_arch_devices;
use crate::drivers::timers::{create_devices, Device as TimerDevice};
use crate::spinlock::Spinlock;
use alloc::vec::Vec;
use kernel_interface::init::Args;
use units::{Miliseconds, Minutes, Nanoseconds, Time};

static STATE: Spinlock<Option<State>> = Spinlock::new(None);

pub unsafe fn init_bp(args: &Args) {
    // Create all available timers and place them into a list for selection.

    let mut device_list = Vec::<TimerDevice>::new();
    create_arch_devices(args, &mut device_list);
    create_devices(args, &mut device_list);

    // Select and initialize timer for scheduling.

    let scheduler_timer =
        find_scheduler_timer(&device_list).expect("No available timer for scheduling.");

    println!("Using \"{}\" for scheduler timer.", scheduler_timer.id());

    scheduler_timer.init(args);

    // Select and initialize timer for the system clock.

    let clock_timer = find_clock_timer(&device_list).expect("No available timer for clock.");

    println!("Using \"{}\" for clock timer.", clock_timer.id());

    clock_timer.init(args);

    // Create calibration timer and calibrate if necessary.

    let mut calibration_timer_result = None;

    if scheduler_timer.calibration_required() || clock_timer.calibration_required() {
        // Select and initialize timer for calibration.

        let calibration_timer =
            find_calibration_timer(&device_list).expect("No available timer for calibration.");

        println!(
            "Using \"{}\" for calibration timer.",
            calibration_timer.id()
        );

        calibration_timer.init(args);

        calibration_timer_result = Some(calibration_timer);

        // Calibrate schedule and clock timer.

        if scheduler_timer.calibration_required() {
            calibrate_timer(&scheduler_timer, &calibration_timer);
        }

        if clock_timer.calibration_required() {
            calibrate_timer(&clock_timer, &calibration_timer);
        }
    }

    // Finish initializing state.

    *STATE.lock() = Some(State {
        scheduler_timer,
        clock_timer,
        calibration_timer: calibration_timer_result,
    });
}

pub unsafe fn init_ap() {
    todo!();
}

fn find_scheduler_timer(device_list: &Vec<TimerDevice>) -> Option<TimerDevice> {
    None
}

fn find_clock_timer(device_list: &Vec<TimerDevice>) -> Option<TimerDevice> {
    None
}

fn find_calibration_timer(device_list: &Vec<TimerDevice>) -> Option<TimerDevice> {
    None
}

fn calibrate_timer(timer: &TimerDevice, calibration_timer: &TimerDevice) {
    let calibration_time: Nanoseconds<u64> = Miliseconds::new(10).convert();

    calibration_timer.start(calibration_time);

    timer.calibration().start();

    while calibration_timer.read_count().into_inner() > 0 {}

    timer.calibration().finish(calibration_time);
}

struct State {
    scheduler_timer: TimerDevice,
    clock_timer: TimerDevice,
    calibration_timer: Option<TimerDevice>,
}
