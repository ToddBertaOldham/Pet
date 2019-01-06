// *************************************************************************
// display.rs
// Copyright 2018 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use ffi::*;
use core::ffi::c_void;
use core::ptr::null_mut;
use drawing::*;

pub struct GraphicsOutputProvider {
    image_handle : Handle,
    system_table : *mut SystemTable,
    gop_handles : *mut Handle,
    gop_handle_count : usize
}

impl GraphicsOutputProvider {
    pub fn new(image_handle : Handle, system_table : *mut SystemTable) -> Self {
        unsafe {
            let boot_services : &BootServices = &*(*system_table).boot_services;

            let mut guid = GOP_GUID;
            let mut handle_count : usize = 0;
            let mut handle_buffer : *mut Handle = null_mut();

            (boot_services.locate_handle_buffer)(LocateSearchType::ByProtocol, &mut guid as *mut GUID, core::ptr::null_mut::<c_void>(), &mut handle_count as *mut usize, &mut handle_buffer as *mut *mut Handle);

            GraphicsOutputProvider { image_handle : image_handle, system_table : system_table, gop_handles : handle_buffer, gop_handle_count : handle_count }
        }
    }

    pub fn count(&self) -> usize {
        self.gop_handle_count
    }

    pub fn get(&self, id : usize) -> GraphicsOutput {
        unsafe {
            GraphicsOutput::new(self.image_handle, self.system_table, *(self.gop_handles.offset(id as isize)))
        }
    }
}

impl Drop for GraphicsOutputProvider {
    fn drop(&mut self) {
        unsafe {
            if (*self.system_table).boot_services == null_mut() { return; }
            ((*(*self.system_table).boot_services).free_pool)(self.gop_handles as *mut c_void);
        }
    }
}

pub struct GraphicsOutput {
    image_handle : Handle,
    system_table : *mut SystemTable,
    handle : Handle,
    gop : *mut GOP
}

impl GraphicsOutput {
    pub fn new(image_handle : Handle, system_table : *mut SystemTable, handle : Handle) -> Self {
        unsafe {
            let boot_services : &BootServices = &*(*system_table).boot_services;

            let mut guid = GOP_GUID;
            let mut interface : *mut c_void = null_mut();

            (boot_services.open_protocol)(handle, &mut guid as *mut GUID, &mut interface as *mut *mut c_void, image_handle, Handle::with_null_value(), OPEN_PROTOCOL_BY_HANDLE_PROTOCOL);

            GraphicsOutput { image_handle : image_handle, system_table : system_table, handle : handle, gop : interface as *mut GOP }
        }
    }

    pub fn set_mode(&self, mode : u32) {
        unsafe {
            ((*self.gop).set_mode)(self.gop, mode);  
        }
    }

    pub fn mode_count(&self) -> u32 {
        unsafe {
            (*(*self.gop).mode).max_mode
        }
    }

    pub fn linear_framebuffer(&self) -> PhysicalAddress {
        unsafe {
            (*(*self.gop).mode).frame_buffer_base
        }
    }

    pub fn draw_rectangle(&self, rectangle : Rectangle, color : Color) {
        unsafe {
            let mut pixel = BltPixel::new((color.b * 255.0) as u8, (color.g * 255.0) as u8, (color.r * 255.0) as u8);
            
            ((*self.gop).blt)(self.gop, &mut pixel as *mut BltPixel, BltOperation::VideoFill, 0, 0, rectangle.x as usize, rectangle.y as usize, rectangle.width as usize, rectangle.height as usize, 0);
        }
    }
}

impl Drop for GraphicsOutput {
    fn drop(&mut self) {
        unsafe {      
            if (*self.system_table).boot_services == null_mut() { return; }
            let mut guid = GOP_GUID;
            ((*(*self.system_table).boot_services).close_protocol)(self.handle, &mut guid as *mut GUID, self.image_handle, Handle::with_null_value());
        }
    }
}