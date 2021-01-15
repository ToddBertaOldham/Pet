//**************************************************************************************************
// io_port.rs                                                                                      *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct IoPort(u16);

impl IoPort {
    pub const fn new(address: u16) -> Self {
        Self(address)
    }

    pub const fn address(self) -> u16 {
        self.0
    }

    pub unsafe fn out_u8(self, value: u8) {
        llvm_asm!("outb %al, %dx" :: "{dx}"(self.0), "{al}"(value) :: "volatile");
    }

    pub unsafe fn out_16(self, value: u16) {
        llvm_asm!("outw %ax, %dx" :: "{dx}"(self.0), "{ax}"(value) :: "volatile");
    }

    pub unsafe fn out_32(self, value: u32) {
        llvm_asm!("outl %eax, %dx" :: "{dx}"(self.0), "{eax}"(value) :: "volatile");
    }

    pub unsafe fn in_u8(self) -> u8 {
        let value: u8;
        llvm_asm!("inb %dx, %al" : "={al}"(value) : "{dx}"(self.0) :: "volatile");
        value
    }

    pub unsafe fn in_u16(self) -> u16 {
        let value: u16;
        llvm_asm!("inw %dx, %ax" : "={ax}"(value) : "{dx}"(self.0) :: "volatile");
        value
    }

    pub unsafe fn in_u32(self) -> u32 {
        let value: u32;
        llvm_asm!("inl %dx, %eax" : "={eax}"(value) : "{dx}"(self.0) :: "volatile");
        value
    }
}
