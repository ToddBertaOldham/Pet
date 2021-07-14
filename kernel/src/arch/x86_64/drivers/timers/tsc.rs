//**************************************************************************************************
// tsc.rs                                                                                          *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::drivers::timers::{Device, DeviceCalibration};
use crate::spinlock::Spinlock;
use units::Nanoseconds;
use x86;
use x86::cpuid;

static STATE: Spinlock<State> = Spinlock::new(State {
    time_per: None,
    calibration_start: None,
});

pub unsafe fn create_device() -> Option<Device> {
    let (_, _, features) = cpuid::leaf_1::read();

    if features.tsc() {
        let calibration = DeviceCalibration::new(start_calibration, finish_calibration);

        return Some(Device::new(
            "TSC",
            None,
            None,
            read_count,
            Some(calibration),
        ));
    }

    None
}

pub fn read_count() -> Nanoseconds<u64> {
    // unsafe {
    //     x86::tsc::read()
    // }
    todo!()
}

pub fn start_calibration() {
    // Record the current count of the TSC for completing the calibration later.

    let mut state = STATE.lock();

    assert!(
        state.calibration_start.is_none(),
        "TSC calibration was already started."
    );

    state.calibration_start = Some(unsafe { x86::tsc::read() });
}

pub fn finish_calibration(time_passed: Nanoseconds<u64>) {
    // Read the current count of the TSC and calculate the difference since
    // calibration was started. This produces a ratio of how many nanoseconds occur
    // everytime the TSC count is increased by 1.

    let mut state = STATE.lock();

    let previous_count = state
        .calibration_start
        .expect("TSC calibration was not started.");

    let current_count = unsafe { x86::tsc::read() };

    let difference = current_count - previous_count;

    state.time_per = Some(time_passed / difference);
    state.calibration_start = None;
}

struct State {
    time_per: Option<Nanoseconds<u64>>,
    calibration_start: Option<u64>,
}

unsafe impl Send for State {}
