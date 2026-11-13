//! Byte layout of the Windows x64 kernel structures a driver reads directly.
//!
//! `IoGetCurrentIrpStackLocation`, `IoGetNextIrpStackLocation`,
//! `IoSetCompletionRoutine`, `ExInitializeFastMutex`, `MmGetSystemAddressForMdlSafe`
//! and friends are inline in the DDK headers: the driver's own code reads and
//! writes these offsets, so the objects the emulator builds must have the real
//! layout, not a convenient one. Offsets are from the public Windows 10 x64
//! symbols; they have been stable since Vista for everything used here.

/// `UNICODE_STRING` / `ANSI_STRING`: `Length` u16, `MaximumLength` u16, pad, `Buffer`.
pub const UNICODE_STRING_SIZE: u64 = 0x10;

// --- DRIVER_OBJECT ----------------------------------------------------------
pub const DRIVER_OBJECT_SIZE: u64 = 0x150;
pub const DRVOBJ_TYPE: u64 = 0x00; // CSHORT, IO_TYPE_DRIVER = 4
pub const DRVOBJ_SIZE: u64 = 0x02;
pub const DRVOBJ_DEVICE_OBJECT: u64 = 0x08;
pub const DRVOBJ_FLAGS: u64 = 0x10;
pub const DRVOBJ_DRIVER_START: u64 = 0x18;
pub const DRVOBJ_DRIVER_SIZE: u64 = 0x20;
pub const DRVOBJ_DRIVER_SECTION: u64 = 0x28;
pub const DRVOBJ_DRIVER_EXTENSION: u64 = 0x30;
pub const DRVOBJ_DRIVER_NAME: u64 = 0x38;
pub const DRVOBJ_HARDWARE_DATABASE: u64 = 0x48;
pub const DRVOBJ_FAST_IO_DISPATCH: u64 = 0x50;
pub const DRVOBJ_DRIVER_INIT: u64 = 0x58;
pub const DRVOBJ_DRIVER_START_IO: u64 = 0x60;
pub const DRVOBJ_DRIVER_UNLOAD: u64 = 0x68;
pub const DRVOBJ_MAJOR_FUNCTION: u64 = 0x70;
pub const IRP_MJ_MAXIMUM_FUNCTION: u64 = 0x1b;
pub const IO_TYPE_DRIVER: u16 = 4;

/// `DRIVER_EXTENSION`: `DriverObject`, `AddDevice`, `Count`, `ServiceKeyName`.
pub const DRIVER_EXTENSION_SIZE: u64 = 0x40;
pub const DRVEXT_DRIVER_OBJECT: u64 = 0x00;
pub const DRVEXT_ADD_DEVICE: u64 = 0x08;
pub const DRVEXT_SERVICE_KEY_NAME: u64 = 0x18;

// --- DEVICE_OBJECT ----------------------------------------------------------
pub const DEVICE_OBJECT_SIZE: u64 = 0x150;
pub const DEVOBJ_TYPE: u64 = 0x00; // IO_TYPE_DEVICE = 3
pub const DEVOBJ_SIZE: u64 = 0x02;
pub const DEVOBJ_REFERENCE_COUNT: u64 = 0x04;
pub const DEVOBJ_DRIVER_OBJECT: u64 = 0x08;
pub const DEVOBJ_NEXT_DEVICE: u64 = 0x10;
pub const DEVOBJ_ATTACHED_DEVICE: u64 = 0x18;
pub const DEVOBJ_CURRENT_IRP: u64 = 0x20;
pub const DEVOBJ_FLAGS: u64 = 0x30;
pub const DEVOBJ_CHARACTERISTICS: u64 = 0x34;
pub const DEVOBJ_DEVICE_EXTENSION: u64 = 0x40;
pub const DEVOBJ_DEVICE_TYPE: u64 = 0x48;
pub const DEVOBJ_STACK_SIZE: u64 = 0x4c;
pub const DEVOBJ_ALIGNMENT_REQUIREMENT: u64 = 0x98;
pub const DEVOBJ_SECURITY_DESCRIPTOR: u64 = 0x110;
pub const DEVOBJ_SECTOR_SIZE: u64 = 0x130;
pub const IO_TYPE_DEVICE: u16 = 3;
pub const DO_DEVICE_INITIALIZING: u32 = 0x80;
pub const DO_EXCLUSIVE: u32 = 0x8;

// --- FILE_OBJECT ------------------------------------------------------------
pub const FILE_OBJECT_SIZE: u64 = 0xd8;
pub const FILEOBJ_TYPE: u64 = 0x00; // IO_TYPE_FILE = 5
pub const FILEOBJ_SIZE: u64 = 0x02;
pub const FILEOBJ_DEVICE_OBJECT: u64 = 0x08;
pub const FILEOBJ_FS_CONTEXT: u64 = 0x18;
pub const FILEOBJ_FS_CONTEXT2: u64 = 0x20;
pub const FILEOBJ_FLAGS: u64 = 0x50;
pub const FILEOBJ_FILE_NAME: u64 = 0x58;
pub const IO_TYPE_FILE: u16 = 5;

// --- IRP ---------------------------------------------------------------------
pub const IRP_SIZE: u64 = 0xd0;
pub const IRP_TYPE: u64 = 0x00; // IO_TYPE_IRP = 6
pub const IRP_SIZE_FIELD: u64 = 0x02;
pub const IRP_MDL_ADDRESS: u64 = 0x08;
pub const IRP_FLAGS: u64 = 0x10;
pub const IRP_SYSTEM_BUFFER: u64 = 0x18; // AssociatedIrp.SystemBuffer
pub const IRP_IOSTATUS_STATUS: u64 = 0x30;
pub const IRP_IOSTATUS_INFORMATION: u64 = 0x38;
pub const IRP_REQUESTOR_MODE: u64 = 0x40;
pub const IRP_PENDING_RETURNED: u64 = 0x41;
pub const IRP_STACK_COUNT: u64 = 0x42;
pub const IRP_CURRENT_LOCATION: u64 = 0x43;
pub const IRP_CANCEL: u64 = 0x44;
pub const IRP_USER_IOSB: u64 = 0x48;
pub const IRP_USER_EVENT: u64 = 0x50;
pub const IRP_CANCEL_ROUTINE: u64 = 0x68;
pub const IRP_USER_BUFFER: u64 = 0x70;
pub const IRP_TAIL_THREAD: u64 = 0x98;
pub const IRP_TAIL_LIST_ENTRY: u64 = 0xa8;
pub const IRP_CURRENT_STACK_LOCATION: u64 = 0xb8;
pub const IRP_ORIGINAL_FILE_OBJECT: u64 = 0xc0;
pub const IO_TYPE_IRP: u16 = 6;
pub const IRP_BUFFERED_IO: u32 = 0x10;
pub const IRP_DEALLOCATE_BUFFER: u32 = 0x20;
pub const IRP_INPUT_OPERATION: u32 = 0x40;

// --- IO_STACK_LOCATION -------------------------------------------------------
pub const IO_STACK_LOCATION_SIZE: u64 = 0x48;
pub const IOSL_MAJOR_FUNCTION: u64 = 0x00;
pub const IOSL_MINOR_FUNCTION: u64 = 0x01;
pub const IOSL_FLAGS: u64 = 0x02;
pub const IOSL_CONTROL: u64 = 0x03;
pub const IOSL_PARAMETERS: u64 = 0x08;
/// `Parameters.DeviceIoControl`
pub const IOSL_IOCTL_OUTPUT_LENGTH: u64 = 0x08;
pub const IOSL_IOCTL_INPUT_LENGTH: u64 = 0x10;
pub const IOSL_IOCTL_CODE: u64 = 0x18;
pub const IOSL_IOCTL_TYPE3_INPUT: u64 = 0x20;
/// `Parameters.Read` / `Parameters.Write`
pub const IOSL_RW_LENGTH: u64 = 0x08;
pub const IOSL_RW_KEY: u64 = 0x10;
pub const IOSL_RW_BYTE_OFFSET: u64 = 0x18;
pub const IOSL_DEVICE_OBJECT: u64 = 0x28;
pub const IOSL_FILE_OBJECT: u64 = 0x30;
pub const IOSL_COMPLETION_ROUTINE: u64 = 0x38;
pub const IOSL_CONTEXT: u64 = 0x40;

pub const IRP_MJ_CREATE: u8 = 0x00;
pub const IRP_MJ_CLOSE: u8 = 0x02;
pub const IRP_MJ_READ: u8 = 0x03;
pub const IRP_MJ_WRITE: u8 = 0x04;
pub const IRP_MJ_DEVICE_CONTROL: u8 = 0x0e;
pub const IRP_MJ_INTERNAL_DEVICE_CONTROL: u8 = 0x0f;
pub const IRP_MJ_SHUTDOWN: u8 = 0x10;
pub const IRP_MJ_CLEANUP: u8 = 0x12;
pub const IRP_MJ_POWER: u8 = 0x16;
pub const IRP_MJ_PNP: u8 = 0x1b;

/// Names of the `MajorFunction` slots, by index.
pub const IRP_MJ_NAMES: [&str; 28] = [
    "IRP_MJ_CREATE",
    "IRP_MJ_CREATE_NAMED_PIPE",
    "IRP_MJ_CLOSE",
    "IRP_MJ_READ",
    "IRP_MJ_WRITE",
    "IRP_MJ_QUERY_INFORMATION",
    "IRP_MJ_SET_INFORMATION",
    "IRP_MJ_QUERY_EA",
    "IRP_MJ_SET_EA",
    "IRP_MJ_FLUSH_BUFFERS",
    "IRP_MJ_QUERY_VOLUME_INFORMATION",
    "IRP_MJ_SET_VOLUME_INFORMATION",
    "IRP_MJ_DIRECTORY_CONTROL",
    "IRP_MJ_FILE_SYSTEM_CONTROL",
    "IRP_MJ_DEVICE_CONTROL",
    "IRP_MJ_INTERNAL_DEVICE_CONTROL",
    "IRP_MJ_SHUTDOWN",
    "IRP_MJ_LOCK_CONTROL",
    "IRP_MJ_CLEANUP",
    "IRP_MJ_CREATE_MAILSLOT",
    "IRP_MJ_QUERY_SECURITY",
    "IRP_MJ_SET_SECURITY",
    "IRP_MJ_POWER",
    "IRP_MJ_SYSTEM_CONTROL",
    "IRP_MJ_DEVICE_CHANGE",
    "IRP_MJ_QUERY_QUOTA",
    "IRP_MJ_SET_QUOTA",
    "IRP_MJ_PNP",
];

/// `IOCTL` transfer method, the low two bits of the control code.
pub const METHOD_BUFFERED: u32 = 0;
pub const METHOD_IN_DIRECT: u32 = 1;
pub const METHOD_OUT_DIRECT: u32 = 2;
pub const METHOD_NEITHER: u32 = 3;

// --- MDL ---------------------------------------------------------------------
pub const MDL_SIZE: u64 = 0x30;
pub const MDL_NEXT: u64 = 0x00;
pub const MDL_SIZE_FIELD: u64 = 0x08;
pub const MDL_FLAGS: u64 = 0x0a;
pub const MDL_PROCESS: u64 = 0x10;
pub const MDL_MAPPED_SYSTEM_VA: u64 = 0x18;
pub const MDL_START_VA: u64 = 0x20;
pub const MDL_BYTE_COUNT: u64 = 0x28;
pub const MDL_BYTE_OFFSET: u64 = 0x2c;
pub const MDL_MAPPED_TO_SYSTEM_VA: u16 = 0x1;
pub const MDL_PAGES_LOCKED: u16 = 0x2;
pub const MDL_SOURCE_IS_NONPAGED_POOL: u16 = 0x4;
pub const MDL_ALLOCATED_FIXED_SIZE: u16 = 0x8;

// --- dispatcher objects -------------------------------------------------------
/// `DISPATCHER_HEADER`: `Type` u8 at 0, `SignalState` i32 at 4, `WaitListHead` at 8.
pub const DISPATCHER_TYPE: u64 = 0x00;
pub const DISPATCHER_SIGNAL_STATE: u64 = 0x04;
pub const KEVENT_SIZE: u64 = 0x18;
pub const NOTIFICATION_EVENT: u8 = 0;
pub const SYNCHRONIZATION_EVENT: u8 = 1;
pub const MUTANT_OBJECT: u8 = 2;
pub const SEMAPHORE_OBJECT: u8 = 5;
pub const NOTIFICATION_TIMER: u8 = 8;
pub const SYNCHRONIZATION_TIMER: u8 = 9;

/// `KTIMER`: header, `DueTime` 0x18, `TimerListEntry` 0x20, `Dpc` 0x30, `Period` 0x38.
pub const KTIMER_DPC: u64 = 0x30;
pub const KTIMER_PERIOD: u64 = 0x38;

/// `KDPC`: `Type` 0, `Importance` 1, `Number` 2, `DpcListEntry` 8,
/// `ProcessorHistory` 0x10, `DeferredRoutine` 0x18, `DeferredContext` 0x20,
/// `SystemArgument1` 0x28, `SystemArgument2` 0x30, `DpcData` 0x38.
pub const KDPC_SIZE: u64 = 0x40;
pub const KDPC_TYPE: u64 = 0x00;
pub const KDPC_DEFERRED_ROUTINE: u64 = 0x18;
pub const KDPC_DEFERRED_CONTEXT: u64 = 0x20;
pub const KDPC_TYPE_VALUE: u8 = 0x13;

/// `FAST_MUTEX`: `Count` i32 at 0 (1 = free), `Owner` 8, `Contention` 0x10, `Event` 0x18.
pub const FAST_MUTEX_COUNT: u64 = 0x00;
pub const FAST_MUTEX_OWNER: u64 = 0x08;

/// `WORK_QUEUE_ITEM`: `List` 0, `WorkerRoutine` 0x10, `Parameter` 0x18.
pub const WORK_QUEUE_ITEM_ROUTINE: u64 = 0x10;
pub const WORK_QUEUE_ITEM_PARAMETER: u64 = 0x18;

/// `IO_WORKITEM` is opaque; the emulator stores the owning device object at 0.
pub const IO_WORKITEM_SIZE: u64 = 0x40;
pub const IO_WORKITEM_DEVICE: u64 = 0x00;

/// `GENERAL_LOOKASIDE_POOL` (x64, 0x80 bytes).
pub const LOOKASIDE_SIZE: u64 = 0x80;
pub const LOOKASIDE_LIST_HEAD: u64 = 0x00;
pub const LOOKASIDE_DEPTH: u64 = 0x10;
pub const LOOKASIDE_MAX_DEPTH: u64 = 0x12;
pub const LOOKASIDE_TYPE: u64 = 0x24;
pub const LOOKASIDE_TAG: u64 = 0x28;
pub const LOOKASIDE_SIZE_FIELD: u64 = 0x2c;
pub const LOOKASIDE_ALLOCATE: u64 = 0x30;
pub const LOOKASIDE_FREE: u64 = 0x38;

/// `WDF_BIND_INFO`: `Size` 0, `Component` 8, `Version` 0x10 (3 u32),
/// `FuncCount` 0x1c, `FuncTable` 0x20, `Module` 0x28.
pub const WDF_BIND_INFO_FUNC_COUNT: u64 = 0x1c;
pub const WDF_BIND_INFO_FUNC_TABLE: u64 = 0x20;

/// `OBJECT_ATTRIBUTES`: `Length` 0, `RootDirectory` 8, `ObjectName` 0x10,
/// `Attributes` 0x18, `SecurityDescriptor` 0x20, `SecurityQualityOfService` 0x28.
pub const OBJATTR_OBJECT_NAME: u64 = 0x10;

/// `SECURITY_DESCRIPTOR` (absolute, x64): `Revision` u8 0, `Sbz1` 1,
/// `Control` u16 2, `Owner` 8, `Group` 0x10, `Sacl` 0x18, `Dacl` 0x20.
pub const SD_ABSOLUTE_SIZE: u64 = 0x28;
pub const SD_REVISION: u64 = 0x00;
pub const SD_CONTROL: u64 = 0x02;
pub const SD_OWNER: u64 = 0x08;
pub const SD_GROUP: u64 = 0x10;
pub const SD_SACL: u64 = 0x18;
pub const SD_DACL: u64 = 0x20;
pub const SE_OWNER_DEFAULTED: u16 = 0x1;
pub const SE_GROUP_DEFAULTED: u16 = 0x2;
pub const SE_DACL_PRESENT: u16 = 0x4;
pub const SE_DACL_DEFAULTED: u16 = 0x8;
pub const SE_SACL_PRESENT: u16 = 0x10;
pub const SE_SELF_RELATIVE: u16 = 0x8000;

/// `ACL`: `AclRevision` u8 0, `Sbz1` 1, `AclSize` u16 2, `AceCount` u16 4, `Sbz2` 6.
pub const ACL_HEADER_SIZE: u64 = 8;
pub const ACL_SIZE: u64 = 0x02;
pub const ACL_ACE_COUNT: u64 = 0x04;
/// `ACE_HEADER` + `Mask`: `AceType` 0, `AceFlags` 1, `AceSize` u16 2, `Mask` u32 4, `SidStart` 8.
pub const ACE_HEADER_SIZE: u64 = 8;

// --- fake objects -------------------------------------------------------------
pub const EPROCESS_SIZE: u64 = 0x1000;
pub const EPROCESS_UNIQUE_PROCESS_ID: u64 = 0x440;
pub const EPROCESS_ACTIVE_PROCESS_LINKS: u64 = 0x448;
pub const EPROCESS_IMAGE_FILE_NAME: u64 = 0x5a8;
pub const KTHREAD_SIZE: u64 = 0x800;
pub const KTHREAD_APCSTATE_PROCESS: u64 = 0x220;
pub const KTHREAD_PREVIOUS_MODE: u64 = 0x232;
pub const KTHREAD_PROCESS: u64 = 0x220;

pub const MM_USER_PROBE_ADDRESS: u64 = 0x0000_7fff_ffff_0000;
pub const MM_HIGHEST_USER_ADDRESS: u64 = 0x0000_7fff_fffe_ffff;
pub const MM_SYSTEM_RANGE_START: u64 = 0xffff_8000_0000_0000;
pub const NT_BUILD_NUMBER: u16 = 19045;
pub const OS_MAJOR: u32 = 10;
pub const OS_MINOR: u32 = 0;

/// Where the harness maps the "user-mode" buffers of the requests it sends.
pub const USER_BUFFER_BASE: u64 = 0x0000_0000_1000_0000;

/// `RTL_PROCESS_MODULE_INFORMATION` (0x128 bytes) and `RTL_PROCESS_MODULES`.
pub const RTL_PROCESS_MODULE_INFO_SIZE: u64 = 0x128;
pub const RTL_PM_IMAGE_BASE: u64 = 0x10;
pub const RTL_PM_IMAGE_SIZE: u64 = 0x18;
pub const RTL_PM_LOAD_COUNT: u64 = 0x24;
pub const RTL_PM_OFFSET_TO_FILE_NAME: u64 = 0x26;
pub const RTL_PM_FULL_PATH_NAME: u64 = 0x28;
