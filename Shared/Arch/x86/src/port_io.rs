// *************************************************************************
// port_io.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

pub unsafe fn out_u8(port : u16, value : u8) {
    asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) :: "volatile");
}

pub unsafe fn out_u16(port : u16, value : u16) {
    asm!("outw %ax, %dx" :: "{dx}"(port), "{ax}"(value) :: "volatile");
}

pub unsafe fn out_u32(port : u16, value : u32) {
    asm!("outl %eax, %dx" :: "{dx}"(port), "{eax}"(value) :: "volatile");
}

pub unsafe fn in_u8(port : u16) -> u8 {
    let value : u8;
    asm!("inb %dx, %al" : "={al}"(value) : "{dx}"(port) :: "volatile");
    value
}

pub unsafe fn in_u16(port : u16) -> u16 {
    let value : u16;
    asm!("inw %dx, %ax" : "={ax}"(value) : "{dx}"(port) :: "volatile");
    value
}

pub unsafe fn in_u32(port : u16) -> u32 {
    let value : u32;
    asm!("inl %dx, %eax" : "={eax}"(value) : "{dx}"(port) :: "volatile");
    value
}