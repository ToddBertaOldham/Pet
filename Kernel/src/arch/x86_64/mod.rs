// *************************************************************************
// mod.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

// Re-export from Shared/Arch/x86_64.
pub use x86::sixty_four::*;
pub use x86::control_registers;

#[no_mangle]
pub unsafe extern fn main(test_buffer : u64, test_width : usize, test_height : usize) {
    let framebuffer = test_buffer as *mut u32;
    let size = test_width * test_height;
    for i in 0..size {
        *(framebuffer.add(i)) = 0xFFFFFFFF;
    } 
}