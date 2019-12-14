//**************************************************************************************************
// boot.rs                                                                                         *
// Copyright (c) 2018-2019 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::primitives::{
    Event, Guid, Handle, PhysicalAddress, Status, TableHeader, VirtualAddress,
};
use super::system;
use core::ffi::c_void;

#[repr(C)]
pub struct Services {
    pub hdr: TableHeader,

    pub raise_tpl: extern "efiapi" fn(new_tpl: TPL) -> TPL,
    pub restore_tpl: extern "efiapi" fn(old_tpl: TPL),

    pub allocate_pages: extern "efiapi" fn(
        allocate_type: AllocateType,
        memory_type: MemoryType,
        pages: usize,
        memory: *mut PhysicalAddress,
    ) -> Status,
    pub free_pages: extern "efiapi" fn(memory: PhysicalAddress, pages: usize) -> Status,
    pub get_memory_map: extern "efiapi" fn(
        memory_map_size: *mut usize,
        memory_map: *mut MemoryDescriptor,
        map_key: *mut usize,
        descriptor_size: *mut usize,
        descriptor_version: *mut u32,
    ) -> Status,
    pub allocate_pool:
        extern "efiapi" fn(pool_type: MemoryType, size: usize, buffer: *mut *mut c_void) -> Status,
    pub free_pool: extern "efiapi" fn(buffer: *mut c_void) -> Status,

    pub create_event: extern "efiapi" fn(
        event_type: u32,
        notify_tpl: TPL,
        notify_function: *mut EventNotify,
        notify_context: *mut c_void,
        event: *mut Event,
    ) -> Status,
    pub set_timer:
        extern "efiapi" fn(event: Event, timer_type: TimerDelay, trigger_time: u64) -> Status,
    pub wait_for_event:
        extern "efiapi" fn(number_of_events: usize, event: *mut Event, index: *mut usize) -> Status,
    pub signal_event: extern "efiapi" fn(event: Event) -> Status,
    pub close_event: extern "efiapi" fn(event: Event) -> Status,
    pub check_event: extern "efiapi" fn(event: Event) -> Status,

    pub install_protocol_interface: extern "efiapi" fn(
        handle: *mut Handle,
        protocol: *mut Guid,
        interface_type: InterfaceType,
        interface: *mut c_void,
    ) -> Status,
    pub reinstall_protocol_interface: extern "efiapi" fn(
        handle: Handle,
        protocol: *mut Guid,
        old_interface: *mut c_void,
        new_interface: *mut c_void,
    ) -> Status,
    pub uninstall_protocol_interface:
        extern "efiapi" fn(handle: Handle, protocol: *mut Guid, interface: *mut c_void) -> Status,
    pub handle_protocol: extern "efiapi" fn(
        handle: Handle,
        protocol: *mut Guid,
        interface: *mut *mut c_void,
    ) -> Status,
    reserved: *mut c_void,
    pub register_protocol_notify: extern "efiapi" fn(
        protocol: *mut Guid,
        event: Event,
        registration: *mut *mut c_void,
    ) -> Status,
    pub locate_handle: extern "efiapi" fn(
        search_type: LocateSearchType,
        protocol: *mut Guid,
        search_key: *mut c_void,
        buffer_size: *mut usize,
        buffer: *mut Handle,
    ) -> Status,
    pub locate_device_path: extern "efiapi" fn(
        protocol: *mut Guid,
        device_path: *mut *mut DevicePathProtocol,
        device: *mut Handle,
    ) -> Status,
    pub install_configuration_table:
        extern "efiapi" fn(guid: *mut Guid, table: *mut c_void) -> Status,

    pub load_image: extern "efiapi" fn(
        boot_policy: bool,
        parent_image_handle: Handle,
        device_path: *mut DevicePathProtocol,
        source_buffer: *mut c_void,
        source_size: usize,
        image_handle: *mut Handle,
    ) -> Status,
    pub start_image: extern "efiapi" fn(
        image_handle: Handle,
        exit_data_size: *mut usize,
        exit_data: *mut *mut u16,
    ) -> Status,
    pub exit: extern "efiapi" fn(
        image_handle: Handle,
        exit_status: Status,
        exit_data_size: usize,
        exit_data: *mut u16,
    ) -> Status,
    pub unload_image: extern "efiapi" fn(image_handle: Handle) -> Status,
    pub exit_boot_services: extern "efiapi" fn(image_handle: Handle, map_key: usize) -> Status,

    pub get_next_monotonic_count: extern "efiapi" fn(count: *mut u64) -> Status,
    pub stall: extern "efiapi" fn(microseconds: usize) -> Status,
    pub set_watchdog_timer: extern "efiapi" fn(
        timeout: usize,
        watchdog_code: u64,
        data_size: usize,
        watchdog_data: *mut u16,
    ) -> Status,

    pub connect_controller: extern "efiapi" fn(
        controller_handle: Handle,
        driver_image_handle: *mut Handle,
        remaining_device_path: *mut DevicePathProtocol,
        recursive: bool,
    ) -> Status,
    pub disconnect_controller: extern "efiapi" fn(
        controller_handle: Handle,
        driver_image_handle: Handle,
        child_handle: Handle,
    ) -> Status,

    pub open_protocol: extern "efiapi" fn(
        handle: Handle,
        protocol: *mut Guid,
        interface: *mut *mut c_void,
        agent_handle: Handle,
        controller_handle: Handle,
        attributes: OpenProtocolAttributes,
    ) -> Status,
    pub close_protocol: extern "efiapi" fn(
        handle: Handle,
        protocol: *mut Guid,
        agent_handle: Handle,
        controller_handle: Handle,
    ) -> Status,
    pub open_protocol_information: extern "efiapi" fn(
        handle: Handle,
        protocol: *mut Guid,
        entry_buffer: *mut *mut OpenProtocolInformationEntry,
        entry_count: *mut usize,
    ) -> Status,
    pub protocols_per_handle: extern "efiapi" fn(
        handle: Handle,
        protocol_buffer: *mut *mut Guid,
        protocol_buffer_count: *mut usize,
    ) -> Status,
    pub locate_handle_buffer: extern "efiapi" fn(
        search_type: LocateSearchType,
        protocol: *mut Guid,
        search_key: *mut c_void,
        no_handles: *mut usize,
        buffer: *mut *mut Handle,
    ) -> Status,
    pub locate_protocol: extern "efiapi" fn(
        protocol: *mut Guid,
        registration: *mut c_void,
        interface: *mut *mut c_void,
    ) -> Status,

    //TODO InstallMultipleProtocolInterfaces and UninstallMultipleProtocolInterfaces once variadic functions are supported for the efiapi abi.
    pub install_multiple_protocol_interfaces: *mut c_void,
    pub uninstall_multiple_protocol_interfaces: *mut c_void,

    pub calculate_crc_32:
        extern "efiapi" fn(data: *mut c_void, data_size: usize, crc_32: *mut u32) -> Status,

    pub copy_mem:
        extern "efiapi" fn(destination: *mut c_void, source: *mut c_void, length: usize) -> Status,
    pub set_mem: extern "efiapi" fn(destination: *mut c_void, size: usize, value: u8) -> Status,

    pub create_event_ex: extern "efiapi" fn(
        event_type: u32,
        notify_tpl: TPL,
        notify_function: EventNotify,
        notify_context: *const c_void,
        event_group: *const Guid,
        event: *mut Event,
    ) -> Status,
}

impl Services {
    pub const SIGNATURE: u64 = 0x56524553544f4f42;
    pub const REVISION: u32 = system::Table::LATEST_REVISION;
}

flags!(
    pub struct OpenProtocolAttributes : u32 {
        BY_HANDLE_PROTOCOL = 0x00000001;
        GET_PROTOCOL = 0x00000002;
        TEST_PROTOCOL = 0x00000004;
        BY_CHILD_CONTROLLER = 0x00000008;
        BY_DRIVER = 0x00000010;
        OPEN_PROTOCOL_EXCLUSIVE = 0x00000020;
    }
);

#[repr(C)]
pub struct OpenProtocolInformationEntry {
    pub agent_handle: Handle,
    pub controller_handle: Handle,
    pub attributes: OpenProtocolAttributes,
    pub open_count: u32,
}

#[repr(C)]
pub struct DevicePathProtocol {
    pub node_type: u8,
    pub sub_node_type: u8,
    pub length: [u8; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocateSearchType {
    AllHandles,
    ByRegisterNotify,
    ByProtocol,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterfaceType {
    NativeInterface,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerDelay {
    Cancel,
    Periodic,
    Relative,
}

#[repr(C)]
pub struct EventNotify {
    pub event: Event,
    pub context: *mut c_void,
}

#[repr(C)]
pub struct MemoryDescriptor {
    pub region_type: MemoryType,
    pub physical_start: PhysicalAddress,
    pub virtual_start: VirtualAddress,
    pub number_of_pages: u64,
    pub attribute: MemoryAttributes,
}

impl MemoryDescriptor {
    pub const VERSION: u64 = 1;
}

flags!(
    pub struct MemoryAttributes : u64 {
        UC = 0x0000000000000001;
        WC = 0x0000000000000002;
        WT = 0x0000000000000004;
        WB = 0x0000000000000008;
        UCE = 0x0000000000000010;
        WP = 0x0000000000001000;
        RP = 0x0000000000002000;
        XP = 0x0000000000004000;
        NV = 0x0000000000008000;
        MORE_RELIABLE = 0x0000000000010000;
        RO = 0x0000000000020000;
        RUNTIME = 8000000000000000;
    }
);

#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPL {
    Application = 4,
    Callback = 8,
    Notify = 16,
    HighLevel = 31,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AllocateType {
    AnyPages,
    MaxAddress,
    Addresses,
}

c_enum!(
    pub enum MemoryType : u32 {
        RESERVED = 0;
        LOADER_CODE = 1;
        LOADER_DATA = 2;
        BOOT_SERVICES_CODE = 3;
        BOOT_SERVICES_DATA = 4;
        RUNTIME_SERVICES_CODE = 5;
        RUNTIME_SERVICES_DATA = 6;
        CONVENTIONAL = 7;
        UNUSABLE = 8;
        ACPI_RECLAIM = 9;
        ACPI_MEMORY_NVS = 10;
        MEMORY_MAPPED_IO = 11;
        MEMORY_MAPPED_IO_PORT_SPACE = 12;
        PAL_CODE = 13;
        PERSISTENT_MEMORY = 14;
    }
);
