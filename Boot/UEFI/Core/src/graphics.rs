// *************************************************************************
// display.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::ffi::c_void;
use core::ptr::null_mut;
use core::option::Option;
use core::result::Result;
use alloc::vec::Vec;
use super::drawing::*;
use super::ffi::*;
use super::error::UEFIError;
use super::system as uefi_system;

pub struct GraphicsOutputProvider {
    gop_handles : *mut Handle,
    gop_handle_count : usize
}

impl GraphicsOutputProvider {
    pub fn new() -> Result<Self, UEFIError> {
        unsafe {
            let system_table = &*uefi_system::system_table()?;

            if system_table.boot_services.is_null() {
                return Err(UEFIError::BootServicesUnavailable);
            }

            let boot_services = &*system_table.boot_services;

            let mut guid = GOP_GUID;
            let mut handle_count : usize = 0;
            let mut handle_buffer : *mut Handle = null_mut();

            let status = (boot_services.locate_handle_buffer)(LocateSearchType::ByProtocol, &mut guid, null_mut(), &mut handle_count, &mut handle_buffer);
            
            match status {
                Status::Success => Ok(GraphicsOutputProvider { gop_handles : handle_buffer, gop_handle_count : handle_count }),
                Status::OutOfResources => Err(UEFIError::OutOfMemory),
                Status::NotFound => Err(UEFIError::NotSupported),
                _ => Err(UEFIError::UnexpectedFFIStatus(status))
            }
        }
    }

    pub fn count(&self) -> usize {
        self.gop_handle_count
    }

    pub fn get(&self, id : usize) -> Result<GraphicsOutput, UEFIError> {
        unsafe {
            if id >= self.gop_handle_count {
                return Err(UEFIError::InvalidArgument("id"));
            }

            let handle = *(self.gop_handles.add(id));

            GraphicsOutput::new(handle)
        }
    }

    pub fn collect(&self) -> Result<Vec<GraphicsOutput>, UEFIError> {
        let mut vec = Vec::with_capacity(self.gop_handle_count);
        
        for id in 0..self.gop_handle_count {
            let output = self.get(id)?;
            vec.push(output);
        }

        Ok(vec)
    }
}

impl Drop for GraphicsOutputProvider {
    fn drop(&mut self) {
        unsafe {
            let system_table = &*uefi_system::system_table().unwrap();

            if system_table.boot_services.is_null() { 
                return; 
            }

            let boot_services = &*system_table.boot_services;
            
            (boot_services.free_pool)(self.gop_handles as *mut c_void);
        }
    }
}

pub struct GraphicsOutput {
    handle : Handle,
    gop : *mut GOP
}

impl GraphicsOutput {
    pub unsafe fn new(handle : Handle) -> Result<Self, UEFIError> {
        let system_table = &*uefi_system::system_table()?;
        let boot_services = &*system_table.boot_services;
        let image_handle = uefi_system::handle().unwrap();

        let mut guid = GOP_GUID;
        let mut interface = null_mut();

        let status = (boot_services.open_protocol)(handle, &mut guid, &mut interface, image_handle, null_mut(), OPEN_PROTOCOL_BY_HANDLE_PROTOCOL);

        match status {
            Status::Success => Ok(GraphicsOutput { handle, gop : interface as *mut GOP }),
            Status::InvalidParameter => Err(UEFIError::InvalidArgument("handle")),
            _ => Err(UEFIError::UnexpectedFFIStatus(status))
        }
    }

    pub fn mode(&self) -> u32 {
        unsafe {
            let gop = &*self.gop;
            let gop_mode = &*gop.mode;

            gop_mode.mode
        }
    }

    pub fn set_mode(&self, mode : u32) -> Result<(), UEFIError> {
        unsafe {
            let gop = &*self.gop;
            let gop_mode = &*gop.mode;

            if mode == gop_mode.mode {
                return Ok(());
            }

            let status = (gop.set_mode)(self.gop, mode);

            match status {
                Status::Success => Ok(()),
                Status::Unsupported => Err(UEFIError::InvalidArgument("mode")),
                Status::DeviceError => Err(UEFIError::HardwareFailure),
                _ => Err(UEFIError::UnexpectedFFIStatus(status))
            }
        }
    }

    pub fn mode_count(&self) -> u32 {
        unsafe {            
            let gop = &*self.gop;
            let gop_mode = &*gop.mode;
            
            gop_mode.max_mode
        }
    }

    pub fn query_mode(&self, mode : u32) -> Result<GraphicsOutModeInfo, UEFIError> {
        unsafe {
            let gop = &*self.gop;

            let mut info_size = 0;
            let mut info_ptr = null_mut(); 

            let status = (gop.query_mode)(self.gop, mode, &mut info_size, &mut info_ptr);

            match status {
                Status::Success => {
                    let info = &*info_ptr;
                    Ok(GraphicsOutModeInfo::new(info.horizontal_resolution, info.vertical_resolution, info.pixel_format != PixelFormat::BltOnly))
                }
                Status::InvalidParameter => Err(UEFIError::InvalidArgument("mode")),
                Status::DeviceError => Err(UEFIError::HardwareFailure),
                _ => Err(UEFIError::UnexpectedFFIStatus(status))
            }
        }
    }

    pub fn maximize(&self, require_framebuffer_address : bool)-> Result<(), UEFIError> {
        let mut best_mode = self.mode();
        let mut largest_width = self.width();
        let mut largest_height = self.height();

        for mode in 0..self.mode_count() {
            let info = self.query_mode(mode)?;

            if require_framebuffer_address && !info.supports_framebuffer_address() {
                continue;
            }

            if info.width() > largest_width || info.height() > largest_height {
                best_mode = mode;
                largest_width = info.width();
                largest_height = info.height();
            }
        }

        self.set_mode(best_mode)
    }

    pub fn width(&self) -> u32 {
        unsafe {
            let gop = &*self.gop;
            let gop_mode = &*gop.mode;
            let gop_mode_info = &*gop_mode.info;

            gop_mode_info.horizontal_resolution
        }
    }

    pub fn height(&self) -> u32 {
        unsafe {
            let gop = &*self.gop;
            let gop_mode = &*gop.mode;
            let gop_mode_info = &*gop_mode.info;

            gop_mode_info.vertical_resolution
        }
    }

    pub fn framebuffer_address(&self) -> Option<PhysicalAddress> {
        unsafe {
            let gop = &*self.gop;
            let gop_mode = &*gop.mode;
            let gop_mode_info = &*gop_mode.info;

            if gop_mode_info.pixel_format == PixelFormat::BltOnly {
                return None;
            }

            Some(gop_mode.frame_buffer_base)
        }
    }

    pub fn draw_rectangle(&self, rectangle : Rectangle, color : Color) {
        unsafe {
            let mut pixel = BltPixel::new((color.b * 255.0) as u8, (color.g * 255.0) as u8, (color.r * 255.0) as u8);
            
            ((*self.gop).blt)(self.gop, &mut pixel, BltOperation::VideoFill, 0, 0, rectangle.x as usize, rectangle.y as usize, rectangle.width as usize, rectangle.height as usize, 0);
        }
    }
}

impl Drop for GraphicsOutput {
    fn drop(&mut self) {
        unsafe {      
            let system_table = &*uefi_system::system_table().unwrap();

            if system_table.boot_services.is_null() { 
                return; 
            }

            let boot_services = &*system_table.boot_services;
            let image_handle = uefi_system::handle().unwrap();
            let mut guid = GOP_GUID;

            (boot_services.close_protocol)(self.handle, &mut guid, image_handle, null_mut());
        }
    }
}

pub struct GraphicsOutModeInfo {
    width : u32,
    height : u32,
    //TODO Change this later to pixel format but keep method for it.
    supports_framebuffer_address : bool
}

impl GraphicsOutModeInfo {
    pub fn new(width : u32, height : u32, supports_framebuffer_address : bool) -> Self {
        GraphicsOutModeInfo { width, height, supports_framebuffer_address }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, value : u32) {
        self.width = value;
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, value : u32) {
        self.height = value;
    }

    pub fn supports_framebuffer_address(&self) -> bool {
        self.supports_framebuffer_address
    }
}