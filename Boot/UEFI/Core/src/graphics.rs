// *************************************************************************
// display.rs
// Copyright 2018 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::ffi::c_void;
use core::ptr::null_mut;
use core::option::Option;
use super::drawing::*;
use super::ffi::*;

pub struct GraphicsOutputProvider {
    image_handle : Handle,
    system_table : *mut SystemTable,
    gop_handles : *mut Handle,
    gop_handle_count : usize
}

impl GraphicsOutputProvider {
    pub unsafe fn new(image_handle : Handle, system_table : *mut SystemTable) -> Self {
        let boot_services = &*(*system_table).boot_services;

        let mut guid = GOP_GUID;
        let mut handle_count : usize = 0;
        let mut handle_buffer : *mut Handle = null_mut();

        (boot_services.locate_handle_buffer)(LocateSearchType::ByProtocol, &mut guid as *mut GUID, core::ptr::null_mut::<c_void>(), &mut handle_count as *mut usize, &mut handle_buffer as *mut *mut Handle);

        GraphicsOutputProvider { image_handle : image_handle, system_table : system_table, gop_handles : handle_buffer, gop_handle_count : handle_count }        
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
    pub unsafe fn new(image_handle : Handle, system_table : *mut SystemTable, handle : Handle) -> Self {
        let table = &*system_table;
        let boot_services = &*table.boot_services;

        let mut guid = GOP_GUID;
        let mut interface = null_mut::<c_void>();

        (boot_services.open_protocol)(handle, &mut guid as *mut GUID, &mut interface as *mut *mut c_void, image_handle, null_mut(), OPEN_PROTOCOL_BY_HANDLE_PROTOCOL);

        GraphicsOutput { image_handle : image_handle, system_table : system_table, handle : handle, gop : interface as *mut GOP }      
    }

    pub fn mode(&self) -> u32 {
        unsafe {
            let gop = &*self.gop;
            let gop_mode = &*gop.mode;

            gop_mode.mode
        }
    }

    pub fn set_mode(&self, mode : u32) {
        unsafe {
            let gop = &*self.gop;
            let gop_mode = &*gop.mode;

            if mode == gop_mode.mode {
                return;
            }

            (gop.set_mode)(self.gop, mode);  
        }
    }

    pub fn mode_count(&self) -> u32 {
        unsafe {            
            let gop = &*self.gop;
            let gop_mode = &*gop.mode;
            
            gop_mode.max_mode
        }
    }

    pub fn query_mode(&self, mode : u32) -> GraphicsOutModeInfo {
        unsafe {
            let gop = &*self.gop;

            let mut info_size : usize = 0;
            let mut info_ptr = null_mut::<GOPModeInfo>(); 

            let status = (gop.query_mode)(self.gop, mode, &mut info_size as *mut usize, &mut info_ptr as *mut *mut GOPModeInfo);

            let info = &*info_ptr;

            GraphicsOutModeInfo::new(info.horizontal_resolution, info.vertical_resolution, info.pixel_format != PixelFormat::BltOnly)
        }
    }

    pub fn maximize(&self, require_framebuffer_address : bool) {
        let mut best_mode = self.mode();
        let mut largest_width = self.width();
        let mut largest_height = self.height();

        for mode in 0..self.mode_count() {
            let info = self.query_mode(mode);

            if require_framebuffer_address && !info.supports_framebuffer_address() {
                continue;
            }

            if info.width() > largest_width || info.height() > largest_height {
                best_mode = mode;
                largest_width = info.width();
                largest_height = info.height();
            }
        }

        self.set_mode(best_mode);
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
            
            ((*self.gop).blt)(self.gop, &mut pixel as *mut BltPixel, BltOperation::VideoFill, 0, 0, rectangle.x as usize, rectangle.y as usize, rectangle.width as usize, rectangle.height as usize, 0);
        }
    }
}

impl Drop for GraphicsOutput {
    fn drop(&mut self) {
        unsafe {      
            let system_table = &*self.system_table;
            let boot_services = &*system_table.boot_services;

            if system_table.boot_services == null_mut() { 
                return; 
            }

            let mut guid = GOP_GUID;

            (boot_services.close_protocol)(self.handle, &mut guid as *mut GUID, self.image_handle, null_mut());
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
        GraphicsOutModeInfo { width : width, height : height, supports_framebuffer_address : supports_framebuffer_address }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn supports_framebuffer_address(&self) -> bool {
        self.supports_framebuffer_address
    }
}