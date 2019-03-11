// *************************************************************************
// system_table.rs
// Copyright 2018 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

// Reference available at http://wiki.phoenix.com/wiki/index.php/EFI_SYSTEM_TABLE.

use core::ffi::c_void;
use super::text_io::{SimpleTextInputProtocol, SimpleTextOutputProtocol};
use super::primitives::{Event, Handle, PhysicalAddress, Status, Time, VirtualAddress, GUID};

#[repr(C)]
pub struct SystemTable {
    pub hdr : SystemTableHeader,
    pub firmware_vendor : *mut u16,
    pub firmware_revision : u32,
    pub console_in_handle : Handle,
    pub con_in : *mut SimpleTextInputProtocol,
    pub console_out_handle : Handle,
    pub con_out : *mut SimpleTextOutputProtocol,
    pub standard_error_handle : Handle,
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
    pub update_capsule : extern "win64" fn(capsule_header_array : *mut *mut CapsuleHeader, capsule_count : usize, scatter_gather_list : PhysicalAddress) -> Status,
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
    pub hdr : SystemTableHeader,
    
    pub raise_tpl : extern "win64" fn(new_tpl : TPL) -> TPL,
    pub restore_tpl : extern "win64" fn(old_tpl : TPL),

    pub allocate_pages : extern "win64" fn(allocate_type : AllocateType, memory_type : MemoryType, pages : usize, memory : *mut PhysicalAddress) -> Status,
    pub free_pages : extern "win64" fn(memory : PhysicalAddress, pages : usize) -> Status,
    pub get_memory_map : extern "win64" fn(memory_map_size : *mut usize, memory_map : *mut MemoryDescriptor, map_key : *mut usize, descriptor_size : *mut usize, descriptor_version : *mut u32) -> Status,
    pub allocate_pool : extern "win64" fn(pool_type : MemoryType, size : usize, buffer : *mut *mut c_void) -> Status,
    pub free_pool : extern "win64" fn(buffer : *mut c_void) -> Status,

    pub create_event : extern "win64" fn(event_type : u32, notify_tpl : TPL, notify_function : *mut EventNotify, notify_context : *mut c_void, event : *mut Event) -> Status,
    pub set_timer : extern "win64" fn(event : Event, timer_type : TimerDelay, trigger_time : u64) -> Status,
    pub wait_for_event : extern "win64" fn(number_of_events : usize, event : *mut Event, index : *mut usize) -> Status,
    pub signal_event : extern "win64" fn(event : Event) -> Status,
    pub close_event : extern "win64" fn(event : Event) -> Status,
    pub check_event : extern "win64" fn(event : Event) -> Status,

    pub install_protocol_interface : extern "win64" fn(handle : *mut Handle, protocol : *mut GUID, interface_type : InterfaceType, interface : *mut c_void) -> Status,
    pub reinstall_protocol_interface : extern "win64" fn(handle : Handle, protocol : *mut GUID, old_interface : *mut c_void, new_interface : *mut c_void) -> Status,
    pub uninstall_protocol_interface : extern "win64" fn(handle : Handle, protocol : *mut GUID, interface : *mut c_void) -> Status,
    pub handle_protocol : extern "win64" fn(handle : Handle, protocol : *mut GUID, interface : *mut *mut c_void) -> Status,
    reserved : *mut c_void,
    pub register_protocol_notify : extern "win64" fn(protocol : *mut GUID, event : Event, registration : *mut *mut c_void) -> Status,
    pub locate_handle : extern "win64" fn(search_type : LocateSearchType, protocol : *mut GUID, search_key : *mut c_void, buffer_size : *mut usize, buffer : *mut Handle) -> Status,
    pub locate_device_path : extern "win64" fn(protocol : *mut GUID, device_path : *mut *mut DevicePathProtocol, device : *mut Handle) -> Status,
    pub install_configuration_table : extern "win64" fn(guid : *mut GUID, table : *mut c_void) -> Status,
    
    pub load_image : extern "win64" fn(boot_policy : bool, parent_image_handle : Handle, device_path : *mut DevicePathProtocol, source_buffer : *mut c_void, source_size : usize, image_handle : *mut Handle) -> Status,
    pub start_image : extern "win64" fn(image_handle : Handle, exit_data_size : *mut usize, exit_data : *mut *mut u16) -> Status,
    pub exit : extern "win64" fn(image_handle : Handle, exit_status : Status, exit_data_size : usize, exit_data : *mut u16) -> Status,
    pub unload_image : extern "win64" fn(image_handle : Handle) -> Status,
    pub exit_boot_services : extern "win64" fn(image_handle : Handle, map_key : usize) -> Status,

    pub get_next_monotonic_count : extern "win64" fn(count : *mut u64) -> Status,
    pub stall : extern "win64" fn(microseconds : usize) -> Status,
    pub set_watchdog_timer : extern "win64" fn(timeout : usize, watchdog_code : u64, data_size : usize, watchdog_data : *mut u16) -> Status,
    
    pub connect_controller : extern "win64" fn(controller_handle : Handle, driver_image_handle : *mut Handle, remaining_device_path : *mut DevicePathProtocol, recursive : bool) -> Status,
    pub disconnect_controller : extern "win64" fn(controller_handle : Handle, driver_image_handle : Handle, child_handle : Handle) -> Status,
    
    pub open_protocol : extern "win64" fn(handle : Handle, protocol : *mut GUID, interface : *mut *mut c_void, agent_handle : Handle, controller_handle : Handle, attributes : u32) -> Status,
    pub close_protocol : extern "win64" fn(handle : Handle, protocol : *mut GUID, agent_handle : Handle, controller_handle : Handle) -> Status,
    pub open_protocol_information : extern "win64" fn(handle : Handle, protocol : *mut GUID, entry_buffer : *mut *mut OpenProtocolInformationEntry, entry_count : *mut usize) -> Status,
    pub protocols_per_handle : extern "win64" fn(handle : Handle, protocol_buffer : *mut *mut GUID, protocol_buffer_count : *mut usize) -> Status,
    pub locate_handle_buffer : extern "win64" fn(search_type : LocateSearchType, protocol : *mut GUID, search_key : *mut c_void, no_handles : *mut usize, buffer : *mut *mut Handle) -> Status,
    pub locate_protocol : extern "win64" fn(protocol : *mut GUID, registration : *mut c_void, interface : *mut *mut c_void) -> Status,
    //TODO Install/UninstallMultipleProtocolInterfaces https://github.com/rust-lang/rust/pull/49878
    pub install_multiple_protocol_interfaces : *mut c_void,
    pub uninstall_multiple_protocol_interfaces : *mut c_void,

    pub calculate_crc_32 : extern "win64" fn(data : *mut c_void, data_size : usize, crc_32 : *mut u32) -> Status,
    
    pub copy_mem : extern "win64" fn(destination : *mut c_void, source : *mut c_void, length : usize) -> Status,
    pub set_mem : extern "win64" fn(destination : *mut c_void, size : usize, value : u8) -> Status,

    pub create_event_ex : extern "win64" fn(event_type : u32, notify_tpl : TPL, notify_function : EventNotify, notify_context : *const c_void, event_group : *const GUID, event : *mut Event) -> Status,
}

pub const OPEN_PROTOCOL_BY_HANDLE_PROTOCOL : u32 = 0x00000001;

pub const OPEN_PROTOCOL_GET_PROTOCOL : u32 = 0x00000002;

pub const OPEN_PROTOCOL_TEST_PROTOCOL : u32 = 0x00000004;

pub const OPEN_PROTOCOL_BY_CHILD_CONTROLLER : u32 = 0x00000008;

pub const OPEN_PROTOCOL_BY_DRIVER: u32 = 0x00000010;

pub const OPEN_PROTOCOL_EXCLUSIVE: u32 = 0x00000020;

#[repr(C)]
pub struct OpenProtocolInformationEntry {
    pub agent_handle : Handle,
    pub controller_handle : Handle,
    pub attributes : u32,
    pub open_count : u32
}

#[repr(C)]
pub struct DevicePathProtocol {
    pub node_type : u8,
    pub sub_node_type : u8,
    pub length : [u8; 2]
}

#[repr(C)]
pub enum LocateSearchType {
    AllHandles,
    ByRegisterNotify,
    ByProtocol
}

#[repr(C)]
pub enum InterfaceType {
    NativeInterface
}

#[repr(C)]
pub enum TimerDelay {
    Cancel, Periodic, Relative
}

#[repr(C)]
pub struct EventNotify {
    event : Event,
    context : *mut c_void
}

#[repr(C)]
pub struct MemoryDescriptor {
    region_type : u32,
    physical_start : PhysicalAddress,
    virtual_start : VirtualAddress,
    number_of_pages : u64,
    attribute : u64
}

#[repr(usize)]
pub enum TPL {
    Application = 4,
    Callback = 8,
    Notify = 16,
    HighLevel = 31
}

#[repr(C)]
pub enum AllocateType {
    AnyPages,
    MaxAddress,
    Addresses
}

#[repr(C)]
pub enum MemoryType {
    Reserved,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    Conventional,
    Unusable,
    ACPIReclaim,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode
}

#[repr(C)]
pub struct ConfigurationTable {
    pub vendor_guid : GUID,
    pub vendor_table : *mut c_void
}