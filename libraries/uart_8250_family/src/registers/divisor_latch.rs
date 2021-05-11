//**************************************************************************************************
// divisor_latch.rs                                                                                *
// Copyright (c) 2020-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

c_enum!(
    pub enum BaudDivisor : u16 {
        RATE_50 = 2304,
        RATE_75 = 1536,
        RATE_110 = 1047,
        RATE_134 = 857,
        RATE_150 = 768,
        RATE_220 = 524,
        RATE_300 = 384,
        RATE_600 = 192,
        RATE_1200 = 96,
        RATE_1800 = 64,
        RATE_2000 = 58,
        RATE_2400 = 48,
        RATE_3600 = 32,
        RATE_4800 = 24,
        RATE_7200 = 16,
        RATE_9600 = 12,
        RATE_14400 = 8,
        RATE_19200 = 6,
        RATE_38400 = 3,
        RATE_57600 = 2,
        RATE_115200 = 1,
    }
);
