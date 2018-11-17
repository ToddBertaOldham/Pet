// *************************************************************************
// system_table.rs
// Copyright 2018 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

// Reference available at http://wiki.phoenix.com/wiki/index.php/EFI_SYSTEM_TABLE.

use ffi::primitives::*;
use core::ffi::c_void;
use ffi::text_io::*;

#[repr(C)]
pub struct SystemTable {
    pub hdr : SystemTableHeader,
    pub firmware_vendor : *mut u16,
    pub firmware_revision : u32,
    pub console_in_handle : *mut c_void,
    pub con_in : *mut SimpleTextInputProtocol,
    pub console_out_handle : *mut c_void,
    pub con_out : *mut SimpleTextOutputProtocol,
    pub standard_error_handle : *mut c_void,
    pub std_error : *mut SimpleTextOutputProtocol,
    pub runtime_services : *mut RuntimeServices,
    pub boot_services : *mut BootServices,
    pub number_of_table_entries : usize,
    pub configuration_table : *mut ConfigurationTable
}

#[repr(C)]
pub struct SystemTableHeader {
    pub signature : u64,
    pub revision : u32,
    pub header_size : u32,
    pub crc_32 : u32,
    reserved : u32
}

pub const OPTIONALPOINTERFLAG : usize = 0x00000001;

#[repr(C)]
pub struct RuntimeServices {
    pub hdr : SystemTableHeader,
    pub get_time : extern "win64" fn(time : *mut Time, capabilities : *mut TimeCapabilities) -> Status,
    pub set_time : extern "win64" fn(time : *mut Time) -> Status,
    pub get_wakeup_time : extern "win64" fn(enabled : *mut bool, pending : *mut bool, time : *mut Time) -> Status,
    pub set_wakeup_time : extern "win64" fn(enabled : bool, time : *mut Time) -> Status,
    pub convert_pointer : extern "win64" fn(debug_disposition : usize, address : *mut *mut c_void) -> Status,
    pub reset_system : extern "win64" fn(reset_type : ResetType, status : Status, data_size : usize, reset_data : *mut c_void),
    pub get_next_high_monotonic_count : extern "win64" fn(high_count : *mut u32) -> Status,
    pub update_capsule : extern "win64" fn(capsule_header_array : *mut *mut CapsuleHeader, capsule_count : usize, scatter_gather_list : u64) -> Status,
    pub query_capsule_capabilities : extern "win64" fn(capsule_header_array : *mut *mut CapsuleHeader, capsule_count : usize, maximum_capsule_size : *mut u64, reset_type : *mut ResetType) -> Status,
    pub query_variable_info : extern "win64" fn(attributes : u32, maximum_variable_storage_size : *mut u64, remaining_variable_storage_size : *mut u64, maximum_variable_size : *mut u64) -> Status,
}

#[repr(C)]
pub enum ResetType {
    Cold, 
    Warm,
    Shutdown
}

#[repr(C)]
pub struct Time {
    pub year : u16,
    pub month : u8,
    pub day : u8,
    pub hour : u8,
    pub minute : u8,
    pub second : u8,
    pad1 : u8,
    pub nanosecond : u32,
    pub time_zone : i16,
    pub daylight : u8,
    pad2 : u8
}

#[repr(C)]
pub struct TimeCapabilities {
    pub resolution : u32,
    pub accuracy : u32,
    pub sets_to_zero : bool
}

#[repr(C)]
pub struct CapsuleHeader {
    pub capsule_guid : GUID,
    pub header_size : u32,
    pub flags : u32, 
    pub capsule_image_size : u32
}

#[repr(C)]
pub struct BootServices {

}

#[repr(C)]
pub struct ConfigurationTable {

}