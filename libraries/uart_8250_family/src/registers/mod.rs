//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

mod divisor_latch;
mod fifo_control;
mod interrupt_enable;
mod interrupt_id;
mod line_control;
mod line_status;
mod modem_control;
mod modem_status;
mod port;

pub use divisor_latch::*;
pub use fifo_control::*;
pub use interrupt_enable::*;
pub use interrupt_id::*;
pub use line_control::*;
pub use line_status::*;
pub use modem_control::*;
pub use modem_status::*;
pub use port::*;
