//**************************************************************************************************
// divide_config.rs                                                                                *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use enums::c_enum;

c_enum!(
    pub enum DivideValue : u32 {
        BY_2 = 0b0000,
        BY_4 = 0b0001,
        BY_8 = 0b0010,
        BY_16 = 0b0011,
        BY_32 = 0b1000,
        BY_64 = 0b1001,
        BY_128 = 0b1010,
        BY_1 = 0b1011,
    }
);
