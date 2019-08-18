//**************************************************************************************************
// size_64.rs                                                                                      *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use encapsulation::GetterSetters;

#[repr(C, packed)]
#[derive(GetterSetters, Default)]
pub struct Tss {
    reserved_0: u32,
    #[field_access(set = true)]
    rsp_0: u64,
    #[field_access(set = true)]
    rsp_1: u64,
    #[field_access(set = true)]
    rsp_2: u64,
    reserved_1: u64,
    #[field_access(set = true)]
    ist_1: u64,
    #[field_access(set = true)]
    ist_2: u64,
    #[field_access(set = true)]
    ist_3: u64,
    #[field_access(set = true)]
    ist_4: u64,
    #[field_access(set = true)]
    ist_5: u64,
    #[field_access(set = true)]
    ist_6: u64,
    #[field_access(set = true)]
    ist_7: u64,
    reserved_2: u64,
    reserved_3: u16,
    #[field_access(set = true)]
    io_map_base_address: u16,
}

impl Tss {
    pub const fn new() -> Self {
        Tss {
            reserved_0: 0,
            rsp_0: 0,
            rsp_1: 0,
            rsp_2: 0,
            reserved_1: 0,
            ist_1: 0,
            ist_2: 0,
            ist_3: 0,
            ist_4: 0,
            ist_5: 0,
            ist_6: 0,
            ist_7: 0,
            reserved_2: 0,
            reserved_3: 0,
            io_map_base_address: 0,
        }
    }
}
