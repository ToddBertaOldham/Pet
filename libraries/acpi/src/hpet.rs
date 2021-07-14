//**************************************************************************************************
// hpet.rs                                                                                         *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::{DescriptionHeader, Gas};

#[repr(C)]
pub struct Hpet {
    pub header: DescriptionHeader,
    pub event_timer_block_id: u32,
    pub base_address: Gas,
    pub hpet_number: u8,
    pub min_clock_tick: u16,
    pub attributes: u8,
}

impl Hpet {
    pub const SIGNATURE: &'static [u8; 4] = b"HPET";
    pub const REVISION: u32 = 1;
}

pub struct EventTimerBlockId(u32);
