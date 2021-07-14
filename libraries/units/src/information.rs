//**************************************************************************************************
// information.rs                                                                                  *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

unit_type!(Information);

// Base

const BITS_IN_BYTE: u128 = 8;

unit!(Bits("b"): Information = 1, 1, 0);
unit!(Bytes("B"): Information = BITS_IN_BYTE, 1, 0);

// Binary

const BITS_IN_KIBIBYTE: u128 = BITS_IN_BYTE * 1024;
const BITS_IN_MEBIBYTE: u128 = BITS_IN_KIBIBYTE * 1024;
const BITS_IN_GIBIBYTE: u128 = BITS_IN_MEBIBYTE * 1024;
const BITS_IN_TEBIBYTE: u128 = BITS_IN_GIBIBYTE * 1024;
const BITS_IN_PEBIBYTE: u128 = BITS_IN_TEBIBYTE * 1024;
const BITS_IN_EXBIBYTE: u128 = BITS_IN_PEBIBYTE * 1024;
const BITS_IN_ZEBIBYTE: u128 = BITS_IN_EXBIBYTE * 1024;
const BITS_IN_YOBIBYTE: u128 = BITS_IN_ZEBIBYTE * 1024;

unit!(Kibibytes("KiB"): Information = BITS_IN_KIBIBYTE, 1, 0);
unit!(Mebibytes("MiB"): Information = BITS_IN_MEBIBYTE, 1, 0);
unit!(Gibibytes("GiB"): Information = BITS_IN_GIBIBYTE, 1, 0);
unit!(Tebibytes("TiB"): Information = BITS_IN_TEBIBYTE, 1, 0);
unit!(Pebibytes("PiB"): Information = BITS_IN_PEBIBYTE, 1, 0);
unit!(Exbibytes("EiB"): Information = BITS_IN_EXBIBYTE, 1, 0);
unit!(Zebibytes("ZiB"): Information = BITS_IN_ZEBIBYTE, 1, 0);
unit!(Yobibytes("YiB"): Information = BITS_IN_YOBIBYTE, 1, 0);

// Decimal

const BITS_IN_KILOBYTE: u128 = BITS_IN_BYTE * 1000;
const BITS_IN_MEGABYTE: u128 = BITS_IN_KILOBYTE * 1000;
const BITS_IN_GIGABYTE: u128 = BITS_IN_MEGABYTE * 1000;
const BITS_IN_TERABYTE: u128 = BITS_IN_GIGABYTE * 1000;
const BITS_IN_PETABYTE: u128 = BITS_IN_TERABYTE * 1000;
const BITS_IN_EXABYTE: u128 = BITS_IN_PETABYTE * 1000;
const BITS_IN_ZETTABYTE: u128 = BITS_IN_EXABYTE * 1000;
const BITS_IN_YOTTABYTE: u128 = BITS_IN_ZETTABYTE * 1000;

unit!(Kilobytes("kB"): Information = BITS_IN_KILOBYTE, 1, 0);
unit!(Megabytes("MB"): Information = BITS_IN_MEGABYTE, 1, 0);
unit!(Gigabytes("GB"): Information = BITS_IN_GIGABYTE, 1, 0);
unit!(Terabytes("TB"): Information = BITS_IN_TERABYTE, 1, 0);
unit!(Petabytes("PB"): Information = BITS_IN_PETABYTE, 1, 0);
unit!(Exabytes("EB"): Information = BITS_IN_EXABYTE, 1, 0);
unit!(Zettabytes("ZB"): Information = BITS_IN_ZETTABYTE, 1, 0);
unit!(Yottabytes("YB"): Information = BITS_IN_YOTTABYTE, 1, 0);
