use crate::maps::Maps;

use super::ListEntry64;
use super::base_types::{DeviceIoControl, UnicodeString};
use super::kernel_objects::KEVent;

#[derive(Debug)]
pub struct DriverObject {
    pub type_: u16,
    pub size: u16,
    pub device_object: u64,
    pub flags: u32,
    pub driver_start: u64,
    pub driver_size: u32,
    pub driver_section: u64,
    pub driver_extension: u64,
    pub driver_name: UnicodeString,
    pub hardware_database: u64,
    pub fast_io_dispatch: u64,
    pub driver_init: u64,
    pub driver_start_io: u64,
    pub driver_unload: u64,
    pub major_function: [u64; 28],
}

impl Default for DriverObject {
    fn default() -> Self {
        Self::new()
    }
}

impl DriverObject {
    pub fn size() -> u32 {
        0
    }

    pub fn new() -> DriverObject {
        DriverObject {
            type_: 0,
            size: 0,
            device_object: 0,
            flags: 0,
            driver_start: 0,
            driver_size: 0,
            driver_section: 0,
            driver_extension: 0,
            driver_name: UnicodeString::new(),
            hardware_database: 0,
            fast_io_dispatch: 0,
            driver_init: 0,
            driver_start_io: 0,
            driver_unload: 0,
            major_function: [0; 28],
        }
    }
}

#[derive(Debug)]
pub struct KDeviceQueue {
    pub type_: u16,
    pub size: u16,
    pub device_list_head: ListEntry64,
    pub lock: u64,
    pub busy: u8,
}

impl Default for KDeviceQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl KDeviceQueue {
    pub fn size() -> u32 {
        2 + 2 + 8 + 1 + 0x10
    }

    pub fn new() -> KDeviceQueue {
        KDeviceQueue {
            type_: 0,
            size: 0,
            device_list_head: ListEntry64::new(),
            lock: 0,
            busy: 0,
        }
    }
}

#[derive(Debug)]
pub struct KDPC {
    pub type_: u8,
    pub importance: u8,
    pub number: u16,
    pub dpc_list_entry: ListEntry64,
    pub deferred_routine: u64,
    pub deferred_context: u64,
    pub system_argument1: u64,
    pub system_argument2: u64,
    pub dpc_data: u64,
}

impl Default for KDPC {
    fn default() -> Self {
        Self::new()
    }
}

impl KDPC {
    pub fn size() -> u32 {
        1 + 1 + 2 + 0x10 + 8 + 8 + 8 + 8 + 8
    }

    pub fn new() -> KDPC {
        KDPC {
            type_: 0,
            importance: 0,
            number: 0,
            dpc_list_entry: ListEntry64::new(),
            deferred_routine: 0,
            deferred_context: 0,
            system_argument1: 0,
            system_argument2: 0,
            dpc_data: 0,
        }
    }
}

#[derive(Debug)]
pub struct DeviceObject {
    pub type_: u16,
    pub size: u16,
    pub reference_count: u32,
    pub driver_object: u64,
    pub next_device: u64,
    pub attached_device: u64,
    pub current_irp: u64,
    pub timer: u64,
    pub flags: u32,
    pub characteristics: u32,
    pub vpb: u64,
    pub device_extension: u64,
    pub device_type: u32,
    pub stack_size: u8,
    pub queue: ListEntry64,
    pub alignment_requirement: u32,
    pub device_queue: KDeviceQueue,
    pub dpc: KDPC,
    pub active_thread_count: u32,
    pub security_descriptor: u64,
    pub device_lock: KEVent,
    pub sector_size: u16,
    pub spare1: u16,
    pub device_object_extension: u64,
    pub reserved: u64,
}

impl Default for DeviceObject {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceObject {
    pub fn new() -> DeviceObject {
        DeviceObject {
            type_: 0,
            size: 0,
            reference_count: 0,
            driver_object: 0,
            next_device: 0,
            attached_device: 0,
            current_irp: 0,
            timer: 0,
            flags: 0,
            characteristics: 0,
            vpb: 0,
            device_extension: 0,
            device_type: 0,
            stack_size: 0,
            queue: ListEntry64::new(),
            alignment_requirement: 0,
            device_queue: KDeviceQueue::new(),
            dpc: KDPC::new(),
            active_thread_count: 0,
            security_descriptor: 0,
            device_lock: KEVent::new(),
            sector_size: 0,
            spare1: 0,
            device_object_extension: 0,
            reserved: 0,
        }
    }
}

#[derive(Debug)]
pub struct FileObject {
    pub type_: u16,
    pub size: u16,
    pub device_object: u64,
    pub vpb: u64,
    pub fs_context: u64,
    pub fs_context2: u64,
    pub section_object_pointer: u64,
    pub private_cache_map: u64,
    pub final_status: u32,
    pub related_file_object: u64,
    pub lock_operation: u8,
    pub delete_pending: u8,
    pub read_access: u8,
    pub write_access: u8,
    pub delete_access: u8,
    pub shared_read: u8,
    pub shared_write: u8,
    pub shared_delete: u8,
    pub flags: u32,
    pub file_name: UnicodeString,
    pub current_byte_offset: u64,
    pub waiters: u32,
    pub busy: u32,
    pub last_lock: u64,
    pub lock: KEVent,
    pub event: KEVent,
    pub completion_context: u64,
    pub irp_list_lock: u32,
    pub irp_list: ListEntry64,
    pub file_object_extension: u64,
}

impl Default for FileObject {
    fn default() -> Self {
        Self::new()
    }
}

impl FileObject {
    pub fn new() -> FileObject {
        FileObject {
            type_: 0,
            size: 0,
            device_object: 0,
            vpb: 0,
            fs_context: 0,
            fs_context2: 0,
            section_object_pointer: 0,
            private_cache_map: 0,
            final_status: 0,
            related_file_object: 0,
            lock_operation: 0,
            delete_pending: 0,
            read_access: 0,
            write_access: 0,
            delete_access: 0,
            shared_read: 0,
            shared_write: 0,
            shared_delete: 0,
            flags: 0,
            file_name: UnicodeString::new(),
            current_byte_offset: 0,
            waiters: 0,
            busy: 0,
            last_lock: 0,
            lock: KEVent::new(),
            event: KEVent::new(),
            completion_context: 0,
            irp_list_lock: 0,
            irp_list: ListEntry64::new(),
            file_object_extension: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IoParameters {
    pub device_io_control: DeviceIoControl,
}

impl IoParameters {
    pub fn new() -> IoParameters {
        IoParameters {
            device_io_control: DeviceIoControl::new(),
        }
    }
}

#[derive(Debug)]
pub struct IoStackLocation {
    pub major_function: u8,
    pub minor_function: u8,
    pub flags: u8,
    pub control: u8,
    pub _padding: [u8; 8],
    pub parameters: IoParameters,
    pub device_object: u64,
    pub file_object: u64,
    pub completion_routine: u64,
    pub context: u64,
}

impl Default for IoStackLocation {
    fn default() -> Self {
        Self::new()
    }
}

impl IoStackLocation {
    pub fn new() -> IoStackLocation {
        IoStackLocation {
            major_function: 0,
            minor_function: 0,
            flags: 0,
            control: 0,
            _padding: [0; 8],
            parameters: IoParameters::new(),
            device_object: 0,
            file_object: 0,
            completion_routine: 0,
            context: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> IoStackLocation {
        IoStackLocation {
            major_function: maps.read_byte(addr).unwrap(),
            minor_function: maps.read_byte(addr + 1).unwrap(),
            flags: maps.read_byte(addr + 2).unwrap(),
            control: maps.read_byte(addr + 3).unwrap(),
            _padding: maps.read_bytes_array::<8>(addr + 4),
            parameters: IoParameters {
                device_io_control: DeviceIoControl::load(addr + 12, maps),
            },
            device_object: maps
                .read_qword(addr + 12 + DeviceIoControl::size() as u64)
                .unwrap(),
            file_object: maps
                .read_qword(addr + 12 + DeviceIoControl::size() as u64 + 8)
                .unwrap(),
            completion_routine: maps
                .read_qword(addr + 12 + DeviceIoControl::size() as u64 + 16)
                .unwrap(),
            context: maps
                .read_qword(addr + 12 + DeviceIoControl::size() as u64 + 24)
                .unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_byte(addr, self.major_function);
        maps.write_byte(addr + 1, self.minor_function);
        maps.write_byte(addr + 2, self.flags);
        maps.write_byte(addr + 3, self.control);
        maps.write_bytes(addr + 4, &self._padding);
        self.parameters.device_io_control.save(addr + 12, maps);
        maps.write_qword(
            addr + 12 + DeviceIoControl::size() as u64,
            self.device_object,
        );
        maps.write_qword(
            addr + 12 + DeviceIoControl::size() as u64 + 8,
            self.file_object,
        );
        maps.write_qword(
            addr + 12 + DeviceIoControl::size() as u64 + 16,
            self.completion_routine,
        );
        maps.write_qword(
            addr + 12 + DeviceIoControl::size() as u64 + 24,
            self.context,
        );
    }
}

#[derive(Debug)]
pub struct IrpOverlay {
    pub user_apc_routine: u64,
    pub user_apc_context: u64,
}

impl Default for IrpOverlay {
    fn default() -> Self {
        Self::new()
    }
}

impl IrpOverlay {
    pub fn new() -> IrpOverlay {
        IrpOverlay {
            user_apc_routine: 0,
            user_apc_context: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> IrpOverlay {
        IrpOverlay {
            user_apc_routine: maps.read_qword(addr).unwrap(),
            user_apc_context: maps.read_qword(addr + 8).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.user_apc_routine);
        maps.write_qword(addr + 8, self.user_apc_context);
    }
}

#[derive(Debug)]
pub struct IoStatusBlock {
    pub status: u64,
    pub information: u64,
}

impl Default for IoStatusBlock {
    fn default() -> Self {
        Self::new()
    }
}

impl IoStatusBlock {
    pub fn new() -> IoStatusBlock {
        IoStatusBlock {
            status: 0,
            information: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> IoStatusBlock {
        IoStatusBlock {
            status: maps.read_qword(addr).unwrap(),
            information: maps.read_qword(addr + 8).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_qword(addr, self.status);
        maps.write_qword(addr + 8, self.information);
    }
}

#[derive(Debug)]
pub struct KDeviceQueueEntry {
    pub device_list_entry: ListEntry64,
    pub sort_key: u32,
    pub inserted: u8,
    pub _padding: [u8; 3],
}

impl Default for KDeviceQueueEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl KDeviceQueueEntry {
    pub fn new() -> KDeviceQueueEntry {
        KDeviceQueueEntry {
            device_list_entry: ListEntry64::new(),
            sort_key: 0,
            inserted: 0,
            _padding: [0; 3],
        }
    }
}

#[derive(Debug)]
pub struct TailOverlay {
    pub device_queue_entry: KDeviceQueueEntry,
    pub padding: [u8; 8],
    pub reserved1: [u64; 2],
    pub list_entry: ListEntry64,
    pub current_stack_location: u64,
    pub reserved2: u64,
}

impl Default for TailOverlay {
    fn default() -> Self {
        Self::new()
    }
}

impl TailOverlay {
    pub fn new() -> TailOverlay {
        TailOverlay {
            device_queue_entry: KDeviceQueueEntry::new(),
            padding: [0; 8],
            reserved1: [0; 2],
            list_entry: ListEntry64::new(),
            current_stack_location: 0,
            reserved2: 0,
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> TailOverlay {
        TailOverlay {
            device_queue_entry: KDeviceQueueEntry::new(),
            padding: maps.read_bytes_array::<8>(addr + 24),
            reserved1: [
                maps.read_qword(addr + 32).unwrap(),
                maps.read_qword(addr + 40).unwrap(),
            ],
            list_entry: ListEntry64::new(),
            current_stack_location: maps.read_qword(addr + 56).unwrap(),
            reserved2: maps.read_qword(addr + 64).unwrap(),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_bytes(addr + 24, &self.padding);
        maps.write_qword(addr + 32, self.reserved1[0]);
        maps.write_qword(addr + 40, self.reserved1[1]);
        self.list_entry.save(addr + 48, maps);
        maps.write_qword(addr + 56, self.current_stack_location);
        maps.write_qword(addr + 64, self.reserved2);
    }
}

#[derive(Debug)]
pub struct IrpTail {
    pub overlay: TailOverlay,
}

impl Default for IrpTail {
    fn default() -> Self {
        Self::new()
    }
}

impl IrpTail {
    pub fn new() -> IrpTail {
        IrpTail {
            overlay: TailOverlay::new(),
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> IrpTail {
        IrpTail {
            overlay: TailOverlay::load(addr, maps),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        self.overlay.save(addr, maps)
    }
}

#[derive(Debug)]
pub struct Irp {
    pub type_: u16,
    pub size: u16,
    pub mdl_address: u64,
    pub flags: u32,
    pub associated_irp: u64,
    pub thread_list_entry: ListEntry64,
    pub io_status: IoStatusBlock,
    pub requestor_mode: u8,
    pub pending_returned: u8,
    pub stack_count: u8,
    pub current_location: u8,
    pub cancel: u8,
    pub cancel_irql: u8,
    pub apc_environment: u8,
    pub allocation_flags: u8,
    pub user_iosb: u64,
    pub user_event: u64,
    pub overlay: IrpOverlay,
    pub cancel_routine: u64,
    pub user_buffer: u64,
    pub tail: IrpTail,
}

impl Default for Irp {
    fn default() -> Self {
        Self::new()
    }
}

impl Irp {
    pub fn new() -> Irp {
        Irp {
            type_: 0,
            size: 0,
            mdl_address: 0,
            flags: 0,
            associated_irp: 0,
            thread_list_entry: ListEntry64::new(),
            io_status: IoStatusBlock::new(),
            requestor_mode: 0,
            pending_returned: 0,
            stack_count: 0,
            current_location: 0,
            cancel: 0,
            cancel_irql: 0,
            apc_environment: 0,
            allocation_flags: 0,
            user_iosb: 0,
            user_event: 0,
            overlay: IrpOverlay::new(),
            cancel_routine: 0,
            user_buffer: 0,
            tail: IrpTail::new(),
        }
    }

    pub fn load(addr: u64, maps: &Maps) -> Irp {
        Irp {
            type_: maps.read_word(addr).unwrap(),
            size: maps.read_word(addr + 2).unwrap(),
            mdl_address: maps.read_qword(addr + 4).unwrap(),
            flags: maps.read_dword(addr + 12).unwrap(),
            associated_irp: maps.read_qword(addr + 16).unwrap(),
            thread_list_entry: ListEntry64::new(),
            io_status: IoStatusBlock::load(addr + 32, maps),
            requestor_mode: maps.read_byte(addr + 48).unwrap(),
            pending_returned: maps.read_byte(addr + 49).unwrap(),
            stack_count: maps.read_byte(addr + 50).unwrap(),
            current_location: maps.read_byte(addr + 51).unwrap(),
            cancel: maps.read_byte(addr + 52).unwrap(),
            cancel_irql: maps.read_byte(addr + 53).unwrap(),
            apc_environment: maps.read_byte(addr + 54).unwrap(),
            allocation_flags: maps.read_byte(addr + 55).unwrap(),
            user_iosb: maps.read_qword(addr + 56).unwrap(),
            user_event: maps.read_qword(addr + 64).unwrap(),
            overlay: IrpOverlay::load(addr + 72, maps),
            cancel_routine: maps.read_qword(addr + 88).unwrap(),
            user_buffer: maps.read_qword(addr + 96).unwrap(),
            tail: IrpTail::load(addr + 104, maps),
        }
    }

    pub fn save(&self, addr: u64, maps: &mut Maps) {
        maps.write_word(addr, self.type_);
        maps.write_word(addr + 2, self.size);
        maps.write_qword(addr + 4, self.mdl_address);
        maps.write_dword(addr + 12, self.flags);
        maps.write_qword(addr + 16, self.associated_irp);
        self.thread_list_entry.save(addr + 24, maps);
        self.io_status.save(addr + 32, maps);
        maps.write_byte(addr + 48, self.requestor_mode);
        maps.write_byte(addr + 49, self.pending_returned);
        maps.write_byte(addr + 50, self.stack_count);
        maps.write_byte(addr + 51, self.current_location);
        maps.write_byte(addr + 52, self.cancel);
        maps.write_byte(addr + 53, self.cancel_irql);
        maps.write_byte(addr + 54, self.apc_environment);
        maps.write_byte(addr + 55, self.allocation_flags);
        maps.write_qword(addr + 56, self.user_iosb);
        maps.write_qword(addr + 64, self.user_event);
        self.overlay.save(addr + 72, maps);
        maps.write_qword(addr + 88, self.cancel_routine);
        maps.write_qword(addr + 96, self.user_buffer);
        self.tail.save(addr + 104, maps);
    }
}
