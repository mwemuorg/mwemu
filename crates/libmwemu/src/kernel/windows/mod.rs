//! The Windows kernel (ntoskrnl / HAL / FltMgr / WdfLdr) surface for `.sys`
//! drivers.
//!
//! A WDM driver is driven through objects the I/O manager owns: it receives a
//! `DRIVER_OBJECT` in `DriverEntry`, creates `DEVICE_OBJECT`s, and is then
//! called back with `IRP`s. This module models those objects at the byte level
//! (the driver reads them directly, so the layout has to be the real x64 one),
//! implements the APIs a real driver imports, and logs the operations a
//! BYOVD (bring-your-own-vulnerable-driver) analyst is hunting for: MSR and
//! port I/O, physical-memory mappings, arbitrary copies.
//!
//! The pool allocator shares the ledger with the Linux surface, so an
//! `ExFreePoolWithTag` followed by a stale dereference reports exactly like a
//! `kfree` does.
//!
//! Dispatch is split by subsystem, each file with one job:
//! `pool` (Ex/Mm allocators, lookaside, SList, interlocked lists), `io` (I/O
//! manager), `mm` (memory manager, MDL, HAL), `rtl` (runtime library,
//! security descriptors), `crt` (C runtime, printf family), `sync`
//! (Ke/Ex synchronisation, DPC, timers, IRQL), `misc` (Ps/Ob/Se/Zw/Cm/Flt/Wdf/
//! Etw/Po) and `irp` (the harness: DriverEntry and IRP delivery).

pub mod crt;
pub mod io;
pub mod irp;
pub mod misc;
pub mod mm;
pub mod pool;
pub mod rtl;
pub mod structs;
pub mod sync;

use std::collections::HashMap;

use crate::emu::Emu;
use crate::kernel::heap::Region;

pub const STATUS_SUCCESS: u64 = 0;
pub const STATUS_PENDING: u64 = 0x103;
pub const STATUS_TIMEOUT: u64 = 0x102;
pub const STATUS_NO_MORE_ENTRIES: u64 = 0x8000001A;
pub const STATUS_NO_MORE_FILES: u64 = 0x80000006;
pub const STATUS_BUFFER_OVERFLOW: u64 = 0x80000005;
pub const STATUS_UNSUCCESSFUL: u64 = 0xC0000001;
pub const STATUS_NOT_IMPLEMENTED: u64 = 0xC0000002;
pub const STATUS_INVALID_INFO_CLASS: u64 = 0xC0000003;
pub const STATUS_INFO_LENGTH_MISMATCH: u64 = 0xC0000004;
pub const STATUS_ACCESS_VIOLATION: u64 = 0xC0000005;
pub const STATUS_INVALID_PARAMETER: u64 = 0xC000000D;
pub const STATUS_INVALID_DEVICE_REQUEST: u64 = 0xC0000010;
pub const STATUS_END_OF_FILE: u64 = 0xC0000011;
pub const STATUS_NO_MEMORY: u64 = 0xC0000017;
pub const STATUS_ACCESS_DENIED: u64 = 0xC0000022;
pub const STATUS_BUFFER_TOO_SMALL: u64 = 0xC0000023;
pub const STATUS_OBJECT_NAME_NOT_FOUND: u64 = 0xC0000034;
pub const STATUS_OBJECT_NAME_COLLISION: u64 = 0xC0000035;
pub const STATUS_NOT_SUPPORTED: u64 = 0xC00000BB;
pub const STATUS_NOT_FOUND: u64 = 0xC0000225;

/// A device object the driver created through `IoCreateDevice`.
#[derive(Debug, Clone)]
pub struct KernelDevice {
    /// Address of the `DEVICE_OBJECT`.
    pub addr: u64,
    /// `\Device\Name`, empty for an unnamed device.
    pub name: String,
    pub device_type: u32,
    pub extension_size: u32,
}

/// A deferred or registered callback the driver handed to the kernel that is
/// not a work item (process/thread/image notify routines, registry callbacks,
/// power callbacks, ...).
#[derive(Debug, Clone)]
pub struct RegisteredCallback {
    /// Which API registered it.
    pub api: String,
    pub func: u64,
    pub context: u64,
}

/// Windows-specific state of the emulated kernel.
#[derive(Debug, Default)]
pub struct WinEnv {
    /// The `DRIVER_OBJECT` handed to `DriverEntry`.
    pub driver_object: u64,
    /// The `UNICODE_STRING` registry path handed to `DriverEntry`.
    pub registry_path: u64,
    /// Devices created by the driver, in creation order.
    pub devices: Vec<KernelDevice>,
    /// `\DosDevices\X -> \Device\Y` links the driver created.
    pub symlinks: Vec<(String, String)>,
    /// Handles handed out: handle -> what it stands for.
    pub handles: HashMap<u64, String>,
    /// Callbacks the driver registered (notify routines, Cm/Ex callbacks).
    pub callbacks: Vec<RegisteredCallback>,
    /// `MmMapIoSpace` windows still mapped: virtual -> (physical, size).
    pub io_mappings: HashMap<u64, (u64, u64)>,
    /// IRPs completed through `IofCompleteRequest`.
    pub completed_irps: Vec<u64>,
    /// `FILE_OBJECT` used for the harness's requests (created lazily).
    pub file_object: u64,
    /// Fake `EPROCESS` standing for the System process.
    pub system_process: u64,
    /// Fake `KTHREAD` standing for the current thread.
    pub current_thread: u64,
    /// Fake `OBJECT_TYPE` every `*ObjectType` export points at.
    pub object_type: u64,
    /// `SE_EXPORTS`: every member points at the world SID.
    pub se_exports: u64,
    /// `S-1-1-0` blob.
    pub world_sid: u64,
    /// Next address for user-mode buffers the harness creates.
    pub user_next: u64,
    next_handle: u64,
}

impl WinEnv {
    /// First device the driver created: the one an IOCTL goes to by default.
    pub fn default_device(&self) -> Option<u64> {
        self.devices.first().map(|d| d.addr)
    }
}

/// Render a four-character pool tag the way the debugger does.
pub fn tag_name(tag: u64) -> String {
    let bytes = (tag as u32).to_le_bytes();
    let s: String = bytes
        .iter()
        .map(|b| {
            if b.is_ascii_graphic() {
                *b as char
            } else {
                '.'
            }
        })
        .collect();
    format!("pool-{}", s)
}

/// Read a `UNICODE_STRING` (`Length`, `MaximumLength`, `Buffer`).
pub fn read_unicode_string(emu: &Emu, us: u64) -> String {
    if us == 0 {
        return String::new();
    }
    let len = emu.maps.read_word(us).unwrap_or(0) as usize;
    let buf = emu.maps.read_qword(us + 8).unwrap_or(0);
    if buf == 0 || len == 0 {
        return String::new();
    }
    emu.maps.read_wide_string_n(buf, len / 2)
}

/// Read an `ANSI_STRING`.
pub fn read_ansi_string(emu: &Emu, s: u64) -> String {
    if s == 0 {
        return String::new();
    }
    let len = emu.maps.read_word(s).unwrap_or(0) as usize;
    let buf = emu.maps.read_qword(s + 8).unwrap_or(0);
    if buf == 0 || len == 0 {
        return String::new();
    }
    let bytes: Vec<u8> = (0..len)
        .map(|i| emu.maps.read_byte(buf + i as u64).unwrap_or(0))
        .collect();
    String::from_utf8_lossy(&bytes).to_string()
}

/// Write a `UNICODE_STRING` header at `us` describing the wide string at
/// `buf` (`len_bytes` of characters, `max_bytes` of capacity).
pub fn write_unicode_header(emu: &mut Emu, us: u64, buf: u64, len_bytes: u64, max_bytes: u64) {
    emu.maps.write_word(us, len_bytes as u16);
    emu.maps.write_word(us + 2, max_bytes as u16);
    emu.maps.write_dword(us + 4, 0);
    emu.maps.write_qword(us + 8, buf);
}

/// Allocate a pool-backed `UNICODE_STRING` body for `s`, returning the buffer.
/// The caller writes the header; the buffer is a tracked chunk so
/// `RtlFreeUnicodeString` returns it through the ledger.
pub fn alloc_wide(emu: &mut Emu, s: &str, api: &str) -> (u64, u64) {
    let bytes = (s.encode_utf16().count() as u64 + 1) * 2;
    let buf = emu.kernel_alloc(Region::Slab, bytes, "unicode-string", api, true);
    if buf != 0 {
        emu.maps.write_wide_string(buf, s);
    }
    (buf, bytes - 2)
}

/// Hand out a fresh handle value and remember what it stands for.
pub fn new_handle(emu: &mut Emu, what: String) -> u64 {
    let Some(k) = emu.kernel.as_mut() else {
        return 0;
    };
    if k.win.next_handle == 0 {
        k.win.next_handle = 0x100;
    }
    let h = k.win.next_handle;
    k.win.next_handle += 4;
    k.win.handles.insert(h, what);
    h
}

/// Write `value` through an optional out-pointer.
pub fn write_out_qword(emu: &mut Emu, ptr: u64, value: u64) {
    if ptr != 0 {
        emu.maps.write_qword(ptr, value);
    }
}

pub fn write_out_dword(emu: &mut Emu, ptr: u64, value: u32) {
    if ptr != 0 {
        emu.maps.write_dword(ptr, value);
    }
}

/// Current time as a Windows `FILETIME` (100ns ticks since 1601).
pub fn system_time_now() -> u64 {
    const EPOCH_DIFF_SECS: u64 = 11_644_473_600;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    (now.as_secs() + EPOCH_DIFF_SECS) * 10_000_000 + (now.subsec_nanos() as u64) / 100
}

/// Route one kernel API call. Returns false when nothing implements it.
pub fn gateway(symbol: &str, emu: &mut Emu) -> bool {
    pool::dispatch(symbol, emu)
        || io::dispatch(symbol, emu)
        || mm::dispatch(symbol, emu)
        || rtl::dispatch(symbol, emu)
        || sync::dispatch(symbol, emu)
        || crt::dispatch(symbol, emu)
        || misc::dispatch(symbol, emu)
}

/// Kernel exports that name *variables*, with the storage they need.
///
/// On x64 a data import's IAT slot holds the variable's address, exactly like
/// a function import holds the function's, so resolving one to storage in the
/// data region is enough: `mov rax, [__imp_IoDeviceObjectType]` then reads it.
pub fn data_symbol_size(name: &str) -> Option<u64> {
    let size = match name {
        "IoDeviceObjectType"
        | "IoDriverObjectType"
        | "IoFileObjectType"
        | "IoAdapterObjectType"
        | "ExEventObjectType"
        | "ExSemaphoreObjectType"
        | "ExDesktopObjectType"
        | "ExWindowStationObjectType"
        | "PsProcessType"
        | "PsThreadType"
        | "PsJobType"
        | "SeTokenObjectType"
        | "CmKeyObjectType"
        | "LpcPortObjectType"
        | "MmSectionObjectType"
        | "TmTransactionManagerObjectType"
        | "TmResourceManagerObjectType"
        | "TmEnlistmentObjectType"
        | "TmTransactionObjectType" => 8,
        "PsInitialSystemProcess" | "SeExports" | "KeLoaderBlock" | "KdComPortInUse" => 8,
        "MmUserProbeAddress" | "MmHighestUserAddress" | "MmSystemRangeStart" | "MmBadPointer" => 8,
        "KeNumberProcessors"
        | "KdDebuggerNotPresent"
        | "KdDebuggerEnabled"
        | "KdEnteredDebugger"
        | "InitSafeBootMode"
        | "NtBuildNumber"
        | "NtGlobalFlag"
        | "NlsMbCodePageTag"
        | "NlsMbOemCodePageTag"
        | "NlsOemLeadByteInfo"
        | "KeNumberNodes"
        | "KeMaximumIncrement"
        | "KdIgnoreUmExceptions"
        | "MmHighestPhysicalPage"
        | "KeActiveProcessors" => 8,
        "KeTickCount" | "PsLoadedModuleList" | "IoStatisticsLock" => 0x10,
        "KiBugCheckData" => 0x28,
        "PsLoadedModuleResource" | "HalDispatchTable" => 0x100,
        "HalPrivateDispatchTable" | "FsRtlLegalAnsiCharacterArray" => 0x200,
        "IoReadOperationCount"
        | "IoWriteOperationCount"
        | "IoOtherOperationCount"
        | "IoReadTransferCount"
        | "IoWriteTransferCount"
        | "IoOtherTransferCount" => 8,
        _ => return None,
    };
    Some(size)
}

/// Give a freshly resolved data import the value a driver expects to find.
pub fn init_data_symbol(emu: &mut Emu, name: &str, addr: u64) {
    let (object_type, system_process, se_exports) = match emu.kernel.as_ref() {
        Some(k) => (k.win.object_type, k.win.system_process, k.win.se_exports),
        None => return,
    };
    match name {
        n if n.ends_with("ObjectType")
            || n == "PsProcessType"
            || n == "PsThreadType"
            || n == "PsJobType" =>
        {
            emu.maps.write_qword(addr, object_type);
        }
        "PsInitialSystemProcess" => {
            emu.maps.write_qword(addr, system_process);
        }
        "SeExports" => {
            emu.maps.write_qword(addr, se_exports);
        }
        "MmUserProbeAddress" => {
            emu.maps.write_qword(addr, structs::MM_USER_PROBE_ADDRESS);
        }
        "MmHighestUserAddress" => {
            emu.maps.write_qword(addr, structs::MM_HIGHEST_USER_ADDRESS);
        }
        "MmSystemRangeStart" => {
            emu.maps.write_qword(addr, structs::MM_SYSTEM_RANGE_START);
        }
        "KeNumberProcessors" | "KeNumberNodes" | "KdDebuggerNotPresent" | "KeActiveProcessors" => {
            emu.maps.write_qword(addr, 1);
        }
        "NtBuildNumber" => {
            emu.maps.write_word(addr, structs::NT_BUILD_NUMBER);
        }
        "KeMaximumIncrement" => {
            emu.maps.write_dword(addr, 156_250);
        }
        "PsLoadedModuleList" => {
            // An empty, self-linked list: a walk terminates immediately.
            emu.maps.write_qword(addr, addr);
            emu.maps.write_qword(addr + 8, addr);
        }
        "MmHighestPhysicalPage" => {
            emu.maps.write_qword(addr, 0x100000);
        }
        _ => {}
    }
}

/// Create the kernel objects every driver expects to exist before
/// `DriverEntry`: the System process, a current thread, an object type, the
/// `SE_EXPORTS` table, and the harness's user-mode buffer area.
pub fn bootstrap(emu: &mut Emu) {
    if emu
        .kernel
        .as_ref()
        .is_some_and(|k| k.win.system_process != 0)
    {
        return;
    }
    let Some(process) = emu.kernel_data_for("win.system_process", structs::EPROCESS_SIZE) else {
        return;
    };
    emu.maps
        .write_string(process + structs::EPROCESS_IMAGE_FILE_NAME, "System");
    emu.maps
        .write_qword(process + structs::EPROCESS_UNIQUE_PROCESS_ID, 4);
    emu.maps.write_qword(
        process + structs::EPROCESS_ACTIVE_PROCESS_LINKS,
        process + structs::EPROCESS_ACTIVE_PROCESS_LINKS,
    );
    emu.maps.write_qword(
        process + structs::EPROCESS_ACTIVE_PROCESS_LINKS + 8,
        process + structs::EPROCESS_ACTIVE_PROCESS_LINKS,
    );

    let thread = emu
        .kernel_data_for("win.current_thread", structs::KTHREAD_SIZE)
        .unwrap_or(0);
    if thread != 0 {
        // KTHREAD.Process / ApcState.Process point back at the System process.
        emu.maps
            .write_qword(thread + structs::KTHREAD_PROCESS, process);
        emu.maps
            .write_qword(thread + structs::KTHREAD_APCSTATE_PROCESS, process);
        emu.maps
            .write_byte(thread + structs::KTHREAD_PREVIOUS_MODE, 1);
    }
    let object_type = emu.kernel_data_for("win.object_type", 0x100).unwrap_or(0);

    let sid = emu.kernel_data_for("win.world_sid", 0x10).unwrap_or(0);
    if sid != 0 {
        // S-1-1-0: revision 1, one sub-authority, authority 1 (WORLD), RID 0.
        emu.maps
            .write_bytes(sid, &[1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0]);
    }
    let se_exports = emu.kernel_data_for("win.se_exports", 0x200).unwrap_or(0);
    if se_exports != 0 {
        // Every PSID member points at the world SID; the LUID members are 0.
        for i in 0..0x40 {
            emu.maps.write_qword(se_exports + i * 8, sid);
        }
    }

    if let Some(k) = emu.kernel.as_mut() {
        k.win.system_process = process;
        k.win.current_thread = thread;
        k.win.object_type = object_type;
        k.win.se_exports = se_exports;
        k.win.world_sid = sid;
        k.win.user_next = structs::USER_BUFFER_BASE;
    }
}

/// Map a buffer in the user-mode address range, the way an IOCTL caller's
/// buffer would be. Drivers that validate against `MmUserProbeAddress` or
/// call `ProbeForRead` only accept addresses from here.
pub fn alloc_user_buffer(emu: &mut Emu, size: u64, label: &str) -> u64 {
    let size = size.max(1).next_multiple_of(0x1000);
    let Some(k) = emu.kernel.as_mut() else {
        return 0;
    };
    if k.win.user_next == 0 {
        k.win.user_next = structs::USER_BUFFER_BASE;
    }
    let addr = k.win.user_next;
    // A guard page between buffers so an overrun faults instead of bleeding.
    k.win.user_next += size + 0x1000;
    let name = format!("user.{}.{:x}", label, addr);
    match emu.maps.create_map(
        &name,
        addr,
        size,
        crate::maps::mem64::Permission::READ_WRITE,
    ) {
        Ok(m) => {
            m.write_bytes(addr, &vec![0u8; size as usize]);
            addr
        }
        Err(e) => {
            log::error!("cannot map user buffer: {}", e);
            0
        }
    }
}

/// Every symbol this surface implements, grouped by subsystem. Everything
/// listed is handled by a real implementation or a modelled success; an
/// import outside this list still resolves to a stub that returns 0 and is
/// reported in `KernelEnv::unimplemented` when called.
pub const SURFACE: &[(&str, &[&str])] = &[
    ("alloc", pool::SURFACE),
    ("io", io::SURFACE),
    ("mm", mm::SURFACE),
    ("rtl", rtl::SURFACE),
    ("sync", sync::SURFACE),
    ("crt", crt::SURFACE),
    ("misc", misc::SURFACE),
];

/// True when the symbol has an implementation (or a deliberate model).
pub fn is_implemented(name: &str) -> bool {
    SURFACE.iter().any(|(_, names)| names.contains(&name))
}
