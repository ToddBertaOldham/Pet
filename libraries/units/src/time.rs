//**************************************************************************************************
// time.rs                                                                                         *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**********************************/****************************************************************

unit_type!(Time);

unit!(Femtoseconds("fs"): Time = 1, 1_000_000_000_000_000, 0);
unit!(Picoseconds("ps"): Time = 1, 1_000_000_000_000, 0);
unit!(Nanoseconds("ns"): Time = 1, 1_000_000_000, 0);
unit!(Microseconds("µs"): Time = 1, 1_000_000, 0);
unit!(Miliseconds("ms"): Time = 1, 1_000, 0);
unit!(Seconds("s"): Time = 1, 1, 0);
unit!(Minutes("min"): Time = 60, 1, 0);
