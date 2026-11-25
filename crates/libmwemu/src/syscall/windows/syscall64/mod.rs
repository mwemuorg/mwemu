use std::collections::HashMap;
use std::sync::LazyLock;

use crate::api::abi::windows::WindowsAbi::WinSyscall;
use crate::api::windows::common::{handles, ntdll, wait};
use crate::api::windows::policy;
use crate::emu::Emu;
use crate::emu::object_handle::NativeObject;
use crate::emu::winver::{MACHINE_AMD64, ensure_dll};
use crate::syscall::windows;
use crate::syscall::windows::syscall64::memory::{
    read_object_attributes_name, read_object_attributes_root_directory,
};
use crate::syscall::windows::syscall64::sync::next_handle;
use crate::winapi::registry::native;
use crate::windows::constants::errors::*;
use crate::windows::constants::files::*;
use crate::windows::constants::process::{OBJ_CASE_INSENSITIVE, OBJ_INHERIT, OBJ_OPENIF};
use crate::windows::constants::syscalls::*;

mod alpc;
pub(crate) mod memory;
mod nls;
mod process;
mod registry;
mod sync;
mod system;
mod win32k;
pub(crate) mod wnf;
pub(crate) use system::query_system_information;

/// Resolve a system DLL `basename` (lowercase, e.g. "kernelbase.dll") to a host
/// file inside the maps folder. When `--winver` is active and the file isn't
/// cached yet, fetch it from the symbol server on demand and cache it. Returns
/// `None` only when the file genuinely can't be obtained.
pub(crate) fn resolve_maps_dll(emu: &Emu, basename: &str) -> Option<std::path::PathBuf> {
    if basename.is_empty() {
        return None;
    }
    let p = std::path::Path::new(&emu.cfg.maps_folder).join(basename);
    if p.exists() {
        return Some(p);
    }
    // Lazy symbol-server fetch (only DLLs; data files were seeded at setup).
    if let Some(build) = emu.cfg.winver.clone()
        && basename.ends_with(".dll")
    {
        let cache = std::path::Path::new(&emu.cfg.maps_folder);
        // syscall64 is the x64 Windows path.
        match ensure_dll(cache, &build, basename, MACHINE_AMD64) {
            Ok(path) => return Some(path),
            Err(e) => log::trace!("--winver: {} not fetched: {}", basename, e),
        }
    }
    None
}

/// Translate loaded ntdll service numbers to the static x64 handler identifiers.
pub fn build_syscall_translation_table(emu: &mut Emu) {
    static NUMBERS_BY_NAME: LazyLock<HashMap<String, u64>> = LazyLock::new(|| {
        (0..=WIN64_NTWAITLOWEVENTPAIR)
            .map(|number| (what_syscall(number), number))
            .collect()
    });
    let name_map = windows::index_syscall_names(emu);
    let map: HashMap<u64, u64> = name_map
        .iter()
        .filter_map(|(&number, name)| {
            NUMBERS_BY_NAME
                .get(name.as_str())
                .map(|&canonical| (number, canonical))
        })
        .collect();
    log::trace!(
        "syscall translation table built: {} entries (real_nr → canonical), {} names indexed (from loaded ntdll)",
        map.len(),
        name_map.len(),
    );
    emu.syscall_number_map = map;
    emu.syscall_name_by_real = name_map;
}

pub fn gateway(emu: &mut Emu) {
    let number = emu.regs().get_eax();
    if win32k::is_gui_service(number) {
        return win32k::gateway(emu, number);
    }
    let name = emu.syscall_name_by_real.get(&number).cloned();
    let canonical = emu.syscall_number_map.get(&number).copied().or_else(|| {
        if let Some(name) = name.as_ref() {
            (what_syscall(number) == *name).then_some(number)
        } else if emu.syscall_name_by_real.is_empty()
            && emu.cfg.winver.is_none()
            && emu.maps.get_map_by_name("ntdll.pe").is_none()
            && emu.export_indexes.get_by_name("ntdll").is_none()
        {
            // Preserve the original static profile when no guest ntdll is loaded.
            Some(number)
        } else {
            None
        }
    });
    let name = name.unwrap_or_else(|| {
        canonical
            .map(what_syscall)
            .unwrap_or_else(|| format!("syscall 0x{number:x}"))
    });
    windows::dispatch(emu, "ntdll", number, &name, |emu| {
        if handles::dispatch_nt(&name, emu, WinSyscall) || wait::dispatch_nt(&name, emu, WinSyscall)
        {
            return;
        }
        if let Some(canonical) = canonical {
            dispatch(emu, canonical, number);
        } else {
            policy::unsupported_with_abi(emu, "ntdll", &name, emu.pc(), WinSyscall);
        }
    });
}

fn dispatch(emu: &mut Emu, nr: u64, real_nr: u64) {
    match nr {
        WIN64_NTMANAGEHOTPATCH => ntdll::manage_hot_patch(emu, WinSyscall),
        WIN64_NTQUERYLICENSEVALUE => system::query_license_value(emu),
        // User APC queueing is unsupported. No APC is pending on this thread.
        WIN64_NTTESTALERT => WinSyscall.set_return(emu, STATUS_SUCCESS),
        WIN64_NTALLOCATEVIRTUALMEMORY => memory::nt_allocate_virtual_memory(emu),
        WIN64_NTALLOCATEVIRTUALMEMORYEX => memory::nt_allocate_virtual_memory_ex(emu),
        WIN64_NTFREEVIRTUALMEMORY => memory::nt_free_virtual_memory(emu),
        WIN64_NTPROTECTVIRTUALMEMORY => memory::nt_protect_virtual_memory(emu),
        WIN64_NTQUERYVIRTUALMEMORY => memory::nt_query_virtual_memory(emu),
        WIN64_NTREADVIRTUALMEMORY => memory::nt_read_virtual_memory(emu),
        WIN64_NTWRITEVIRTUALMEMORY => memory::nt_write_virtual_memory(emu),
        WIN64_NTCREATESECTION => memory::nt_create_section(emu),
        WIN64_NTOPENSECTION => memory::nt_open_section(emu),
        WIN64_NTMAPVIEWOFSECTION => memory::nt_map_view_of_section(emu),
        WIN64_NTUNMAPVIEWOFSECTION => memory::nt_unmap_view_of_section(emu),
        WIN64_NTQUERYSYSTEMINFORMATION => system::nt_query_system_information(emu),
        WIN64_NTQUERYINFORMATIONPROCESS => process::nt_query_information_process(emu),
        WIN64_NTQUERYINFORMATIONTHREAD => process::nt_query_information_thread(emu),
        WIN64_NTTERMINATEPROCESS => process::nt_terminate_process(emu),
        WIN64_NTQUERYPERFORMANCECOUNTER => process::nt_query_performance_counter(emu),
        WIN64_NTOPENKEY => native::open_key(emu, WinSyscall, false),
        WIN64_NTOPENKEYEX => native::open_key(emu, WinSyscall, true),
        WIN64_NTQUERYKEY => native::query_key(emu, WinSyscall),
        WIN64_NTNOTIFYCHANGEKEY => native::notify_key(emu, WinSyscall),
        WIN64_NTSETINFORMATIONKEY => native::set_key(emu, WinSyscall),
        WIN64_NTQUERYVALUEKEY => native::query_value(emu, WinSyscall),
        WIN64_NTOPENDIRECTORYOBJECT => registry::nt_open_directory_object(emu),
        WIN64_NTOPENSYMBOLICLINKOBJECT => registry::nt_open_symbolic_link_object(emu),
        WIN64_NTQUERYSYMBOLICLINKOBJECT => registry::nt_query_symbolic_link_object(emu),
        WIN64_NTQUERYDEBUGFILTERSTATE => system::nt_query_debug_filter_state(emu),
        WIN64_NTRAISEEXCEPTION => process::nt_raise_exception(emu),
        WIN64_NTRAISEHARDERROR => process::nt_raise_hard_error(emu),
        WIN64_NTQUERYSECURITYATTRIBUTESTOKEN => process::nt_query_security_attributes_token(emu),
        WIN64_NTCONTINUE => process::nt_continue(emu),
        WIN64_NTCREATEPROFILEEX => process::nt_create_profile_ex(emu),
        WIN64_NTALPCCONNECTPORT => alpc::nt_alpc_connect_port(emu),
        WIN64_NTALPCSENDWAITRECEIVEPORT => alpc::nt_alpc_send_wait_receive_port(emu),
        // `NtInitializeNlsFiles`: map `locale.nls` so ntdll's
        // `RtlInitNlsTables` gets the real Unicode upcase/casing tables.
        // Without this, every byte→wide conversion in the loader yields zeros
        // and dependency lookups fail with `STATUS_DLL_NOT_FOUND`.
        WIN64_NTINITIALIZENLSFILES => nls::nt_initialize_nls_files(emu),
        // `NtGetNlsSectionPtr`: map `C_<n>.NLS` for the requested
        // codepage (1252 ANSI, 437 OEM, etc.). Same purpose as above —
        // populates the byte→wide / wide→byte tables consumed by
        // `RtlAnsiStringToUnicodeString`.
        WIN64_NTGETNLSSECTIONPTR => nls::nt_get_nls_section_ptr(emu),
        // `NtQueryAttributesFile`: ntdll's loader stats DLL files on disk
        // to validate the module cache after mapping them via KnownDlls. For real
        // DLLs (kernelbase.dll, kernel32.dll, …) returning OBJECT_NAME_NOT_FOUND
        // makes the loader treat the module as corrupt and call
        // `NtTerminateProcess(STATUS_DLL_NOT_FOUND)`. Resolve the NT path against
        // `cfg.maps_folder` and fill a FILE_BASIC_INFORMATION when the file
        // exists; only fall back to NOT_FOUND for genuinely missing files
        // (manifests, .local, policy probes).
        WIN64_NTQUERYATTRIBUTESFILE => {
            let obj_attr = emu.regs().rcx;
            let file_info_ptr = emu.regs().rdx;
            let nt_name = read_object_attributes_name(emu, obj_attr);
            // Extract the filename (last path segment) from the NT path.
            let basename = nt_name
                .rsplit(['\\', '/'])
                .next()
                .unwrap_or("")
                .to_lowercase();

            let resolved = resolve_maps_dll(emu, &basename);
            if let Some(path) = resolved {
                if file_info_ptr != 0 && emu.maps.is_mapped(file_info_ptr) {
                    // FILE_BASIC_INFORMATION: 4× LARGE_INTEGER + ULONG FileAttributes.
                    // Real timestamps would require unix→FILETIME conversion; the
                    // loader only checks them against its own cache, so a fixed
                    // recent value is good enough for any single emulation.
                    const FAKE_FILETIME: u64 = 0x01DA_0000_0000_0000; // ~2023
                    let _ = emu.maps.write_qword(file_info_ptr, FAKE_FILETIME);
                    let _ = emu.maps.write_qword(file_info_ptr + 0x08, FAKE_FILETIME);
                    let _ = emu.maps.write_qword(file_info_ptr + 0x10, FAKE_FILETIME);
                    let _ = emu.maps.write_qword(file_info_ptr + 0x18, FAKE_FILETIME);
                    let _ = emu
                        .maps
                        .write_dword(file_info_ptr + 0x20, FILE_ATTRIBUTE_NORMAL as u32);
                }
                log_orange!(
                    emu,
                    "syscall 0x{:x}: NtQueryAttributesFile name={:?} → resolved {:?} (SUCCESS)",
                    WIN64_NTQUERYATTRIBUTESFILE,
                    nt_name,
                    path,
                );
                emu.regs_mut().rax = STATUS_SUCCESS;
            } else if basename.ends_with(".dll")
                && nt_name.to_ascii_lowercase().contains("system32")
            {
                // Virtual SYSTEM32 DLL: not in maps_folder but under
                // SYSTEM32.  Return SUCCESS so the loader proceeds to
                // map it (map_file_section will produce a stub PE).
                if file_info_ptr != 0 && emu.maps.is_mapped(file_info_ptr) {
                    const FAKE_FILETIME: u64 = 0x01DA_0000_0000_0000;
                    let _ = emu.maps.write_qword(file_info_ptr, FAKE_FILETIME);
                    let _ = emu.maps.write_qword(file_info_ptr + 0x08, FAKE_FILETIME);
                    let _ = emu.maps.write_qword(file_info_ptr + 0x10, FAKE_FILETIME);
                    let _ = emu.maps.write_qword(file_info_ptr + 0x18, FAKE_FILETIME);
                    let _ = emu
                        .maps
                        .write_dword(file_info_ptr + 0x20, FILE_ATTRIBUTE_NORMAL as u32);
                }
                log_orange!(
                    emu,
                    "syscall 0x{:x}: NtQueryAttributesFile name={:?} → virtual SYSTEM32 DLL",
                    WIN64_NTQUERYATTRIBUTESFILE,
                    nt_name,
                );
                emu.regs_mut().rax = STATUS_SUCCESS;
            } else {
                log_orange!(
                    emu,
                    "syscall 0x{:x}: NtQueryAttributesFile name={:?} → OBJECT_NAME_NOT_FOUND",
                    WIN64_NTQUERYATTRIBUTESFILE,
                    nt_name,
                );
                emu.regs_mut().rax = STATUS_OBJECT_NAME_NOT_FOUND;
            }
        }
        WIN64_NTAREMAPPEDFILESTHESAME => memory::nt_are_mapped_files_the_same(emu),
        WIN64_NTCONNECTPORT => alpc::nt_connect_port(emu),
        // `NtQuerySystemEnvironmentValueEx`: newer ntdll enumerates
        // UEFI variables during init. NOT_IMPLEMENTED and VARIABLE_NOT_FOUND
        // both cause a tight retry loop (~3K iterations). Return SUCCESS with
        // ValueLength=0 and ValueBuffer untouched so the caller treats it as
        // "the variable is empty" and moves on.
        WIN64_NTQUERYSYSTEMENVIRONMENTVALUEEX => {
            // NT syscall ABI: rcx=VarName, rdx=VendorGuid, r8=ValueBuffer,
            // r9=ValueLengthPtr, [rsp+0x28]=Attributes (out, optional).
            let value_len_ptr = emu.regs().r9;
            if value_len_ptr != 0 && emu.maps.is_mapped(value_len_ptr) {
                let _ = emu.maps.write_dword(value_len_ptr, 0);
            }
            log_orange!(
                emu,
                "syscall 0x{:x}: NtQuerySystemEnvironmentValueEx (stub → SUCCESS, len=0)",
                nr
            );
            emu.regs_mut().rax = STATUS_SUCCESS;
        }
        // `NtQuerySystemInformationEx`: newer ntdll uses this during
        // `LdrpInitializeProcess`. Returning STATUS_SUCCESS with `ReturnLength=0`
        // is enough to keep init going — the caller treats "no data" as "no
        // extra info". We do NOT zero the output buffer: callers sometimes
        // pass wildly inflated `out_len` values and a bulk memset overruns
        // the destination map, corrupting adjacent regions.
        WIN64_NTQUERYSYSTEMINFORMATIONEX => {
            let rsp = emu.regs().rsp;
            let info_class = emu.regs().rcx;
            let in_buf = emu.regs().rdx;
            let sysinfo = emu.regs().r9;
            let sysinfo_len = emu.maps.read_qword(rsp + 0x28).unwrap_or(0) as u32;
            let ret_len_ptr = emu.maps.read_qword(rsp + 0x30).unwrap_or(0);

            // class 0x6b = SystemLogicalProcessorAndGroupInformation. ntdll's
            // parallel loader queries this (RelationGroup) very early: it calls
            // once with a NULL/short buffer expecting STATUS_INFO_LENGTH_MISMATCH
            // plus the required size, allocates, and retries. Answering SUCCESS
            // the first time breaks that pattern and ntdll dereferences a NULL
            // result buffer. We model one processor group with one active core.
            let relationship = if in_buf != 0 {
                emu.maps.read_dword(in_buf).unwrap_or(0xffff_ffff)
            } else {
                0xffff_ffff
            };
            if info_class == 0x6b && relationship == 4 {
                // SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX (RelationGroup): a
                // 0x20-byte header + one 0x30-byte PROCESSOR_GROUP_INFO = 0x50.
                const NEED: u32 = 0x50;
                if ret_len_ptr != 0 && emu.maps.is_mapped(ret_len_ptr) {
                    let _ = emu.maps.write_dword(ret_len_ptr, NEED);
                }
                if sysinfo == 0 || sysinfo_len < NEED || !emu.maps.is_mapped(sysinfo) {
                    log_orange!(
                        emu,
                        "syscall 0x{:x}: NtQuerySystemInformationEx SystemLogicalProcessorAndGroupInformation (need 0x{:x}, got 0x{:x}) → STATUS_INFO_LENGTH_MISMATCH",
                        nr,
                        NEED,
                        sysinfo_len,
                    );
                    emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                } else {
                    for off in 0..NEED as u64 {
                        let _ = emu.maps.write_byte(sysinfo + off, 0);
                    }
                    let _ = emu.maps.write_dword(sysinfo, 4); // Relationship = RelationGroup
                    let _ = emu.maps.write_dword(sysinfo + 0x04, NEED); // Size
                    let _ = emu.maps.write_word(sysinfo + 0x08, 1); // MaximumGroupCount
                    let _ = emu.maps.write_word(sysinfo + 0x0a, 1); // ActiveGroupCount
                    let _ = emu.maps.write_byte(sysinfo + 0x20, 1); // GroupInfo[0].MaximumProcessorCount
                    let _ = emu.maps.write_byte(sysinfo + 0x21, 1); // GroupInfo[0].ActiveProcessorCount
                    let _ = emu.maps.write_qword(sysinfo + 0x48, 1); // GroupInfo[0].ActiveProcessorMask
                    log_orange!(
                        emu,
                        "syscall 0x{:x}: NtQuerySystemInformationEx SystemLogicalProcessorAndGroupInformation → 1 group/1 cpu",
                        nr,
                    );
                    emu.regs_mut().rax = STATUS_SUCCESS;
                }
            } else {
                // Generic fallback: SUCCESS with ReturnLength=0. We do NOT zero the
                // output buffer — callers sometimes pass wildly inflated `out_len`
                // and a bulk memset would overrun the destination map.
                if ret_len_ptr != 0 && emu.maps.is_mapped(ret_len_ptr) {
                    let _ = emu.maps.write_dword(ret_len_ptr, 0);
                }
                log_orange!(
                    emu,
                    "syscall 0x{:x}: NtQuerySystemInformationEx class=0x{:x} (stub → SUCCESS, len=0)",
                    nr,
                    info_class,
                );
                emu.regs_mut().rax = STATUS_SUCCESS;
            }
        }
        // `NtQueryQuotaInformationFile`: kernelbase's TLS-callback /
        // process-init path queries this for each loaded module. Returning
        // STATUS_NOT_IMPLEMENTED here causes ntdll to retry indefinitely
        // (we observed ~500K calls eating through the entire stack reserve).
        // STATUS_NOT_SUPPORTED tells the caller "this volume doesn't track
        // quotas" and is treated as a terminal, non-retryable answer.
        WIN64_NTQUERYQUOTAINFORMATIONFILE => {
            log_orange!(
                emu,
                "syscall 0x{:x}: NtQueryQuotaInformationFile (stub → NOT_SUPPORTED)",
                nr,
            );
            emu.regs_mut().rax = STATUS_NOT_SUPPORTED;
        }
        // `NtOpenFile`: ntdll's loader opens KnownDll files on disk to
        // verify them after mapping (timestamps, share-mode locks, …). For real
        // DLLs returning OBJECT_NAME_NOT_FOUND makes the loader treat them as
        // missing and terminate with STATUS_DLL_NOT_FOUND. Resolve the NT path
        // against `cfg.maps_folder` and hand out a fake handle when the file
        // exists; only return NOT_FOUND for genuinely absent paths.
        WIN64_NTOPENFILE => {
            let file_handle_out = emu.regs().rcx;
            let obj_attr = emu.regs().r8;
            let io_status_block = emu.regs().r9;
            let nt_name = read_object_attributes_name(emu, obj_attr);
            let basename = nt_name
                .rsplit(['\\', '/'])
                .next()
                .unwrap_or("")
                .to_lowercase();
            let resolved = resolve_maps_dll(emu, &basename).is_some();
            let is_virtual_dll = !resolved
                && basename.ends_with(".dll")
                && nt_name.to_ascii_lowercase().contains("system32");
            if resolved || is_virtual_dll {
                let h = next_handle();
                if file_handle_out != 0 && emu.maps.is_mapped(file_handle_out) {
                    let _ = emu.maps.write_qword(file_handle_out, h);
                }
                if io_status_block != 0 && emu.maps.is_mapped(io_status_block) {
                    // IO_STATUS_BLOCK: Status (NTSTATUS, 4) + pad(4) + Information (ULONG_PTR, 8).
                    // Information = FILE_OPENED (1).
                    let _ = emu.maps.write_dword(io_status_block, STATUS_SUCCESS as u32);
                    let _ = emu.maps.write_qword(io_status_block + 0x08, 1);
                }
                // Track the handle → basename so a follow-up NtCreateSection(file: h)
                // can mark the new section as backed by this DLL and NtMapViewOfSection
                // will load the real PE (or build a stub for virtual DLLs).
                emu.file_handles.insert(h, basename.clone());
                if is_virtual_dll {
                    emu.virtual_system32_dlls.insert(basename.clone());
                }
                log_orange!(
                    emu,
                    "syscall 0x{:x}: NtOpenFile name={:?} → handle 0x{:x} (SUCCESS{})",
                    nr,
                    nt_name,
                    h,
                    if is_virtual_dll { ", virtual" } else { "" },
                );
                emu.regs_mut().rax = STATUS_SUCCESS;
            } else {
                log_orange!(
                    emu,
                    "syscall 0x{:x}: NtOpenFile name={:?} → OBJECT_NAME_NOT_FOUND",
                    nr,
                    nt_name,
                );
                emu.regs_mut().rax = STATUS_OBJECT_NAME_NOT_FOUND;
            }
        }
        // `NtCreateFile`: ntdll's loader opens DLL files on disk with
        // NtCreateFile (FILE_OPEN disposition) to back a section before mapping.
        // Same arg layout as NtOpenFile for the handles we care about:
        // FileHandle=rcx, ObjectAttributes=r8, IoStatusBlock=r9. Resolve the NT
        // path against `cfg.maps_folder`; hand out a fake handle when the file
        // exists and only return OBJECT_NAME_NOT_FOUND for genuinely absent paths.
        WIN64_NTCREATEFILE => {
            let file_handle_out = emu.regs().rcx;
            let obj_attr = emu.regs().r8;
            let io_status_block = emu.regs().r9;
            let nt_name = read_object_attributes_name(emu, obj_attr);
            let basename = nt_name
                .rsplit(['\\', '/'])
                .next()
                .unwrap_or("")
                .to_lowercase();
            // ConDrv opens Input and Output relative to an existing console device handle.
            let root_dir = read_object_attributes_root_directory(emu, obj_attr);
            let components = nt_name.trim_start_matches('\\');
            let is_console = (root_dir == 0
                && components
                    .split('\\')
                    .take(2)
                    .map(str::to_ascii_lowercase)
                    .eq(["device", "condrv"]))
                || emu.handle_management.is_console_device(root_dir);
            let resolved = resolve_maps_dll(emu, &basename).is_some();
            let is_virtual_dll = !is_console
                && !resolved
                && basename.ends_with(".dll")
                && nt_name.to_ascii_lowercase().contains("system32");
            let exists = is_console || resolved || is_virtual_dll;
            if exists {
                let h = if is_console {
                    const OBJECT_ATTRIBUTES_FLAGS64: u64 = 24;
                    let Some(flags) = obj_attr
                        .checked_add(OBJECT_ATTRIBUTES_FLAGS64)
                        .and_then(|p| emu.maps.read_dword(p))
                    else {
                        return WinSyscall.set_return(emu, STATUS_ACCESS_VIOLATION);
                    };
                    if flags & !(OBJ_INHERIT | OBJ_CASE_INSENSITIVE | OBJ_OPENIF) != 0 {
                        return policy::unsupported_with_abi(
                            emu,
                            "ntdll",
                            "NtCreateFile",
                            emu.pc(),
                            WinSyscall,
                        );
                    }
                    if !emu.maps.validate_write_range(file_handle_out, 8)
                        || !emu.maps.validate_write_range(io_status_block, 16)
                    {
                        return WinSyscall.set_return(emu, STATUS_ACCESS_VIOLATION);
                    }
                    let access = emu.regs().rdx as u32;
                    emu.handle_management
                        .insert_native_object(NativeObject::ConsoleDevice {
                            access,
                            attributes: flags & OBJ_INHERIT,
                        })
                } else {
                    next_handle()
                };
                if file_handle_out != 0 && emu.maps.is_mapped(file_handle_out) {
                    let _ = emu.maps.write_qword(file_handle_out, h);
                }
                if io_status_block != 0 && emu.maps.is_mapped(io_status_block) {
                    // IO_STATUS_BLOCK: Status (4) + pad(4) + Information (8).
                    // Information = FILE_OPENED (1).
                    let _ = emu.maps.write_dword(io_status_block, STATUS_SUCCESS as u32);
                    let _ = emu.maps.write_qword(io_status_block + 0x08, 1);
                }
                if !is_console {
                    emu.file_handles.insert(h, basename.clone());
                    if is_virtual_dll {
                        emu.virtual_system32_dlls.insert(basename.clone());
                    }
                }
                log_orange!(
                    emu,
                    "syscall 0x{:x}: NtCreateFile name={:?} → handle 0x{:x} (SUCCESS)",
                    nr,
                    nt_name,
                    h,
                );
                emu.regs_mut().rax = STATUS_SUCCESS;
            } else {
                log_orange!(
                    emu,
                    "syscall 0x{:x}: NtCreateFile name={:?} → OBJECT_NAME_NOT_FOUND",
                    nr,
                    nt_name,
                );
                emu.regs_mut().rax = STATUS_OBJECT_NAME_NOT_FOUND;
            }
        }
        // `NtDeviceIoControlFile`: kernel32's console client driver issues
        // the CONDRV IOCTLs (BIND_HANDLE / SET_SERVER_INFORMATION / GET_HANDLE …)
        // against the console object handles opened above during process init
        // (`ConsoleClientConnectToServer`). Without an answer the console DLL
        // init returns FALSE → ntdll raises DLL_INIT_FAILED → NtTerminateProcess.
        // We don't model the real console protocol; for a console handle, report
        // success with an empty (zeroed) output so the client treats the console
        // as connected and continues. Non-console handles keep the previous
        // unimplemented behaviour.
        WIN64_NTDEVICEIOCONTROLFILE => {
            let file_handle = emu.regs().rcx;
            let rsp = emu.regs().rsp;
            let io_status_block = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);
            let io_control_code = emu.maps.read_qword(rsp + 0x30).unwrap_or(0) as u32;
            let output_buffer = emu.maps.read_qword(rsp + 0x48).unwrap_or(0);
            let output_len = emu.maps.read_qword(rsp + 0x50).unwrap_or(0) as u32;
            if emu.handle_management.is_console_device(file_handle) {
                if io_status_block != 0 && emu.maps.is_mapped(io_status_block) {
                    // IO_STATUS_BLOCK: Status (4) + pad(4) + Information (8).
                    // Report 0 bytes transferred — we don't synthesize a CONDRV
                    // reply payload, and writing into the (possibly stack-based)
                    // output buffer risks clobbering live locals / stack cookies.
                    let _ = emu.maps.write_dword(io_status_block, STATUS_SUCCESS as u32);
                    let _ = emu.maps.write_qword(io_status_block + 0x08, 0u64);
                }
                log_orange!(
                    emu,
                    "syscall 0x{:x}: NtDeviceIoControlFile console handle 0x{:x} ioctl 0x{:x} out_buf 0x{:x} out_len 0x{:x} (stub → SUCCESS)",
                    nr,
                    file_handle,
                    io_control_code,
                    output_buffer,
                    output_len,
                );
                emu.regs_mut().rax = STATUS_SUCCESS;
            } else {
                log_orange!(
                    emu,
                    "syscall 0x{:x}: NtDeviceIoControlFile handle 0x{:x} ioctl 0x{:x} (unimplemented)",
                    nr,
                    file_handle,
                    io_control_code,
                );
                emu.regs_mut().rax = STATUS_NOT_IMPLEMENTED;
            }
        }
        WIN64_NTCREATEWORKERFACTORY => {
            let handle_out = emu.regs().rcx;
            if handle_out == 0 || !emu.maps.is_mapped(handle_out) {
                emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            } else {
                let h = next_handle();
                let _ = emu.maps.write_qword(handle_out, h);
                log_orange!(
                    emu,
                    "syscall 0x{:x}: {} out=0x{:x} → handle 0x{:x} (stub → SUCCESS)",
                    nr,
                    what_syscall(nr),
                    handle_out,
                    h,
                );
                emu.regs_mut().rax = STATUS_SUCCESS;
            }
        }
        // Thread-pool worker-factory configuration / teardown. After creating a
        // worker factory the pool manager configures it
        // (`NtSetInformationWorkerFactory`: min/max threads, binding info) and,
        // on a probe path, tears it down (`NtShutdownWorkerFactory`). Neither
        // returns data we must synthesize; STATUS_SUCCESS lets the pool setup
        // run to completion instead of looping on STATUS_NOT_IMPLEMENTED.
        WIN64_NTSETINFORMATIONWORKERFACTORY | WIN64_NTSHUTDOWNWORKERFACTORY => {
            log_orange!(
                emu,
                "syscall 0x{:x}: {} (stub → SUCCESS)",
                nr,
                what_syscall(nr),
            );
            emu.regs_mut().rax = STATUS_SUCCESS;
        }
        WIN64_NTQUERYWNFSTATEDATA => wnf::query_state_data(emu),
        WIN64_NTSUBSCRIBEWNFSTATECHANGE => wnf::subscribe(emu),
        WIN64_NTUNSUBSCRIBEWNFSTATECHANGE => wnf::unsubscribe(emu),
        WIN64_NTTRACECONTROL => sync::trace_control(emu),
        WIN64_NTTRACEEVENT => sync::trace_event(emu),
        WIN64_NTSETINFORMATIONPROCESS => process::set_information_process(emu),
        // `NtApphelpCacheControl`: kernel32/kernelbase consult the
        // app-compat shim cache during process start (`BasepCheckBadapp`,
        // `BasepCheckAppCompat`, etc.). STATUS_NOT_SUPPORTED tells callers
        // "no shim infrastructure available" so they skip compat-fixup work
        // and continue with the normal load
        // `handle_NtApphelpCacheControl`.
        WIN64_NTAPPHELPCACHECONTROL => {
            log_orange!(
                emu,
                "syscall 0x{:x}: NtApphelpCacheControl (stub → NOT_SUPPORTED)",
                nr,
            );
            emu.regs_mut().rax = STATUS_NOT_SUPPORTED;
        }
        // `NtOpenThreadToken`: kernel32/kernelbase open a thread's primary
        // token during `BasepCheckAppCompat`, `BasepCheckWebBlockedFileType`,
        // and several user32/uxtheme init paths. Returning STATUS_NOT_IMPLEMENTED
        // makes callers treat the failure as "this thread has no token attached"
        // and retry against `NtOpenProcessToken` — but the retry doesn't always
        // happen (e.g. user32!CreateWindowExW init), and the unhandled fail
        // propagates back as a DllMain `BOOL FALSE`, which makes ntdll roll
        // back the user32 load and `LoadLibraryA` returns 0. STATUS_NO_TOKEN
        // is the documented "no impersonation token on this thread" answer
        // that callers handle gracefully (skip the token-based check and
        // continue init).
        WIN64_NTOPENTHREADTOKEN => {
            let thread_handle = emu.regs().rcx;
            let access = emu.regs().rdx;
            let open_as_self = emu.regs().r8 as u8;
            let handle_out = emu.regs().r9;
            log_orange!(
                emu,
                "syscall 0x{:x}: NtOpenThreadToken thread=0x{:x} access=0x{:x} self={} out=0x{:x} (stub → NO_TOKEN)",
                real_nr,
                thread_handle,
                access,
                open_as_self,
                handle_out,
            );
            // Zero the out-handle so callers don't accidentally re-use a
            // stale pointer value.
            if handle_out != 0 && emu.maps.is_mapped(handle_out) {
                let _ = emu.maps.write_qword(handle_out, 0);
            }
            emu.regs_mut().rax = STATUS_NO_TOKEN;
        }
        _ => {
            policy::unsupported_with_abi(emu, "ntdll", "", emu.pc(), WinSyscall);
        }
    }
}

/// Maps x64 NT syscall number to the `Nt…` export name (PascalCase).
pub fn what_syscall(sys: u64) -> String {
    match sys {
        WIN64_NTACCESSCHECK => "NtAccessCheck",
        WIN64_NTWORKERFACTORYWORKERREADY => "NtWorkerFactoryWorkerReady",
        WIN64_NTACCEPTCONNECTPORT => "NtAcceptConnectPort",
        WIN64_NTMAPUSERPHYSICALPAGESSCATTER => "NtMapUserPhysicalPagesScatter",
        WIN64_NTWAITFORSINGLEOBJECT => "NtWaitForSingleObject",
        WIN64_NTCALLBACKRETURN => "NtCallbackReturn",
        WIN64_NTREADFILE => "NtReadFile",
        WIN64_NTDEVICEIOCONTROLFILE => "NtDeviceIoControlFile",
        WIN64_NTWRITEFILE => "NtWriteFile",
        WIN64_NTREMOVEIOCOMPLETION => "NtRemoveIoCompletion",
        WIN64_NTRELEASESEMAPHORE => "NtReleaseSemaphore",
        WIN64_NTREPLYWAITRECEIVEPORT => "NtReplyWaitReceivePort",
        WIN64_NTREPLYPORT => "NtReplyPort",
        WIN64_NTSETINFORMATIONTHREAD => "NtSetInformationThread",
        WIN64_NTSETEVENT => "NtSetEvent",
        WIN64_NTCLOSE => "NtClose",
        WIN64_NTQUERYOBJECT => "NtQueryObject",
        WIN64_NTQUERYINFORMATIONFILE => "NtQueryInformationFile",
        WIN64_NTOPENKEY => "NtOpenKey",
        WIN64_NTENUMERATEVALUEKEY => "NtEnumerateValueKey",
        WIN64_NTFINDATOM => "NtFindAtom",
        WIN64_NTQUERYDEFAULTLOCALE => "NtQueryDefaultLocale",
        WIN64_NTQUERYKEY => "NtQueryKey",
        WIN64_NTQUERYVALUEKEY => "NtQueryValueKey",
        WIN64_NTALLOCATEVIRTUALMEMORY => "NtAllocateVirtualMemory",
        WIN64_NTQUERYINFORMATIONPROCESS => "NtQueryInformationProcess",
        WIN64_NTWAITFORMULTIPLEOBJECTS32 => "NtWaitForMultipleObjects32",
        WIN64_NTWRITEFILEGATHER => "NtWriteFileGather",
        WIN64_NTSETINFORMATIONPROCESS => "NtSetInformationProcess",
        WIN64_NTCREATEKEY => "NtCreateKey",
        WIN64_NTFREEVIRTUALMEMORY => "NtFreeVirtualMemory",
        WIN64_NTIMPERSONATECLIENTOFPORT => "NtImpersonateClientOfPort",
        WIN64_NTRELEASEMUTANT => "NtReleaseMutant",
        WIN64_NTQUERYINFORMATIONTOKEN => "NtQueryInformationToken",
        WIN64_NTREQUESTWAITREPLYPORT => "NtRequestWaitReplyPort",
        WIN64_NTQUERYVIRTUALMEMORY => "NtQueryVirtualMemory",
        WIN64_NTOPENTHREADTOKEN => "NtOpenThreadToken",
        WIN64_NTQUERYINFORMATIONTHREAD => "NtQueryInformationThread",
        WIN64_NTOPENPROCESS => "NtOpenProcess",
        WIN64_NTSETINFORMATIONFILE => "NtSetInformationFile",
        WIN64_NTMAPVIEWOFSECTION => "NtMapViewOfSection",
        WIN64_NTACCESSCHECKANDAUDITALARM => "NtAccessCheckAndAuditAlarm",
        WIN64_NTUNMAPVIEWOFSECTION => "NtUnmapViewOfSection",
        WIN64_NTREPLYWAITRECEIVEPORTEX => "NtReplyWaitReceivePortEx",
        WIN64_NTTERMINATEPROCESS => "NtTerminateProcess",
        WIN64_NTSETEVENTBOOSTPRIORITY => "NtSetEventBoostPriority",
        WIN64_NTREADFILESCATTER => "NtReadFileScatter",
        WIN64_NTOPENTHREADTOKENEX => "NtOpenThreadTokenEx",
        WIN64_NTOPENPROCESSTOKENEX => "NtOpenProcessTokenEx",
        WIN64_NTQUERYPERFORMANCECOUNTER => "NtQueryPerformanceCounter",
        WIN64_NTENUMERATEKEY => "NtEnumerateKey",
        WIN64_NTOPENFILE => "NtOpenFile",
        WIN64_NTDELAYEXECUTION => "NtDelayExecution",
        WIN64_NTQUERYDIRECTORYFILE => "NtQueryDirectoryFile",
        WIN64_NTQUERYSYSTEMINFORMATION => "NtQuerySystemInformation",
        WIN64_NTOPENSECTION => "NtOpenSection",
        WIN64_NTQUERYTIMER => "NtQueryTimer",
        WIN64_NTFSCONTROLFILE => "NtFsControlFile",
        WIN64_NTWRITEVIRTUALMEMORY => "NtWriteVirtualMemory",
        WIN64_NTCLOSEOBJECTAUDITALARM => "NtCloseObjectAuditAlarm",
        WIN64_NTDUPLICATEOBJECT => "NtDuplicateObject",
        WIN64_NTQUERYATTRIBUTESFILE => "NtQueryAttributesFile",
        WIN64_NTCLEAREVENT => "NtClearEvent",
        WIN64_NTREADVIRTUALMEMORY => "NtReadVirtualMemory",
        WIN64_NTOPENEVENT => "NtOpenEvent",
        WIN64_NTADJUSTPRIVILEGESTOKEN => "NtAdjustPrivilegesToken",
        WIN64_NTDUPLICATETOKEN => "NtDuplicateToken",
        WIN64_NTCONTINUE => "NtContinue",
        WIN64_NTQUERYDEFAULTUILANGUAGE => "NtQueryDefaultUILanguage",
        WIN64_NTQUEUEAPCTHREAD => "NtQueueApcThread",
        WIN64_NTYIELDEXECUTION => "NtYieldExecution",
        WIN64_NTADDATOM => "NtAddAtom",
        WIN64_NTCREATEEVENT => "NtCreateEvent",
        WIN64_NTQUERYVOLUMEINFORMATIONFILE => "NtQueryVolumeInformationFile",
        WIN64_NTCREATESECTION => "NtCreateSection",
        WIN64_NTFLUSHBUFFERSFILE => "NtFlushBuffersFile",
        WIN64_NTAPPHELPCACHECONTROL => "NtApphelpCacheControl",
        WIN64_NTCREATEPROCESSEX => "NtCreateProcessEx",
        WIN64_NTCREATETHREAD => "NtCreateThread",
        WIN64_NTISPROCESSINJOB => "NtIsProcessInJob",
        WIN64_NTPROTECTVIRTUALMEMORY => "NtProtectVirtualMemory",
        WIN64_NTQUERYSECTION => "NtQuerySection",
        WIN64_NTRESUMETHREAD => "NtResumeThread",
        WIN64_NTTERMINATETHREAD => "NtTerminateThread",
        WIN64_NTREADREQUESTDATA => "NtReadRequestData",
        WIN64_NTCREATEFILE => "NtCreateFile",
        WIN64_NTQUERYEVENT => "NtQueryEvent",
        WIN64_NTWRITEREQUESTDATA => "NtWriteRequestData",
        WIN64_NTOPENDIRECTORYOBJECT => "NtOpenDirectoryObject",
        WIN64_NTACCESSCHECKBYTYPEANDAUDITALARM => "NtAccessCheckByTypeAndAuditAlarm",
        WIN64_NTQUERYSYSTEMTIME => "NtQuerySystemTime",
        WIN64_NTWAITFORMULTIPLEOBJECTS => "NtWaitForMultipleObjects",
        WIN64_NTSETINFORMATIONOBJECT => "NtSetInformationObject",
        WIN64_NTCANCELIOFILE => "NtCancelIoFile",
        WIN64_NTTRACEEVENT => "NtTraceEvent",
        WIN64_NTPOWERINFORMATION => "NtPowerInformation",
        WIN64_NTSETVALUEKEY => "NtSetValueKey",
        WIN64_NTCANCELTIMER => "NtCancelTimer",
        WIN64_NTSETTIMER => "NtSetTimer",
        WIN64_NTACCESSCHECKBYTYPE => "NtAccessCheckByType",
        WIN64_NTACCESSCHECKBYTYPERESULTLIST => "NtAccessCheckByTypeResultList",
        WIN64_NTACCESSCHECKBYTYPERESULTLISTANDAUDITALARM => {
            "NtAccessCheckByTypeResultListAndAuditAlarm"
        }
        WIN64_NTACCESSCHECKBYTYPERESULTLISTANDAUDITALARMBYHANDLE => {
            "NtAccessCheckByTypeResultListAndAuditAlarmByHandle"
        }
        WIN64_NTACQUIRECROSSVMMUTANT => "NtAcquireCrossVmMutant",
        WIN64_NTACQUIREPROCESSACTIVITYREFERENCE => "NtAcquireProcessActivityReference",
        WIN64_NTADDATOMEX => "NtAddAtomEx",
        WIN64_NTADDBOOTENTRY => "NtAddBootEntry",
        WIN64_NTADDDRIVERENTRY => "NtAddDriverEntry",
        WIN64_NTADJUSTGROUPSTOKEN => "NtAdjustGroupsToken",
        WIN64_NTADJUSTTOKENCLAIMSANDDEVICEGROUPS => "NtAdjustTokenClaimsAndDeviceGroups",
        WIN64_NTALERTMULTIPLETHREADBYTHREADID => "NtAlertMultipleThreadByThreadId",
        WIN64_NTALERTRESUMETHREAD => "NtAlertResumeThread",
        WIN64_NTALERTTHREAD => "NtAlertThread",
        WIN64_NTALERTTHREADBYTHREADID => "NtAlertThreadByThreadId",
        WIN64_NTALERTTHREADBYTHREADIDEX => "NtAlertThreadByThreadIdEx",
        WIN64_NTALLOCATELOCALLYUNIQUEID => "NtAllocateLocallyUniqueId",
        WIN64_NTALLOCATERESERVEOBJECT => "NtAllocateReserveObject",
        WIN64_NTALLOCATEUSERPHYSICALPAGES => "NtAllocateUserPhysicalPages",
        WIN64_NTALLOCATEUSERPHYSICALPAGESEX => "NtAllocateUserPhysicalPagesEx",
        WIN64_NTALLOCATEUUIDS => "NtAllocateUuids",
        WIN64_NTALLOCATEVIRTUALMEMORYEX => "NtAllocateVirtualMemoryEx",
        WIN64_NTALPCACCEPTCONNECTPORT => "NtAlpcAcceptConnectPort",
        WIN64_NTALPCCANCELMESSAGE => "NtAlpcCancelMessage",
        WIN64_NTALPCCONNECTPORT => "NtAlpcConnectPort",
        WIN64_NTALPCCONNECTPORTEX => "NtAlpcConnectPortEx",
        WIN64_NTALPCCREATEPORT => "NtAlpcCreatePort",
        WIN64_NTALPCCREATEPORTSECTION => "NtAlpcCreatePortSection",
        WIN64_NTALPCCREATERESOURCERESERVE => "NtAlpcCreateResourceReserve",
        WIN64_NTALPCCREATESECTIONVIEW => "NtAlpcCreateSectionView",
        WIN64_NTALPCCREATESECURITYCONTEXT => "NtAlpcCreateSecurityContext",
        WIN64_NTALPCDELETEPORTSECTION => "NtAlpcDeletePortSection",
        WIN64_NTALPCDELETERESOURCERESERVE => "NtAlpcDeleteResourceReserve",
        WIN64_NTALPCDELETESECTIONVIEW => "NtAlpcDeleteSectionView",
        WIN64_NTALPCDELETESECURITYCONTEXT => "NtAlpcDeleteSecurityContext",
        WIN64_NTALPCDISCONNECTPORT => "NtAlpcDisconnectPort",
        WIN64_NTALPCIMPERSONATECLIENTCONTAINEROFPORT => "NtAlpcImpersonateClientContainerOfPort",
        WIN64_NTALPCIMPERSONATECLIENTOFPORT => "NtAlpcImpersonateClientOfPort",
        WIN64_NTALPCOPENSENDERPROCESS => "NtAlpcOpenSenderProcess",
        WIN64_NTALPCOPENSENDERTHREAD => "NtAlpcOpenSenderThread",
        WIN64_NTALPCQUERYINFORMATION => "NtAlpcQueryInformation",
        WIN64_NTALPCQUERYINFORMATIONMESSAGE => "NtAlpcQueryInformationMessage",
        WIN64_NTALPCREVOKESECURITYCONTEXT => "NtAlpcRevokeSecurityContext",
        WIN64_NTALPCSENDWAITRECEIVEPORT => "NtAlpcSendWaitReceivePort",
        WIN64_NTALPCSETINFORMATION => "NtAlpcSetInformation",
        WIN64_NTAREMAPPEDFILESTHESAME => "NtAreMappedFilesTheSame",
        WIN64_NTASSIGNPROCESSTOJOBOBJECT => "NtAssignProcessToJobObject",
        WIN64_NTASSOCIATEWAITCOMPLETIONPACKET => "NtAssociateWaitCompletionPacket",
        WIN64_NTCALLENCLAVE => "NtCallEnclave",
        WIN64_NTCANCELIOFILEEX => "NtCancelIoFileEx",
        WIN64_NTCANCELSYNCHRONOUSIOFILE => "NtCancelSynchronousIoFile",
        WIN64_NTCANCELTIMER2 => "NtCancelTimer2",
        WIN64_NTCANCELWAITCOMPLETIONPACKET => "NtCancelWaitCompletionPacket",
        WIN64_NTCHANGEPROCESSSTATE => "NtChangeProcessState",
        WIN64_NTCHANGETHREADSTATE => "NtChangeThreadState",
        WIN64_NTCOMMITCOMPLETE => "NtCommitComplete",
        WIN64_NTCOMMITENLISTMENT => "NtCommitEnlistment",
        WIN64_NTCOMMITREGISTRYTRANSACTION => "NtCommitRegistryTransaction",
        WIN64_NTCOMMITTRANSACTION => "NtCommitTransaction",
        WIN64_NTCOMPACTKEYS => "NtCompactKeys",
        WIN64_NTCOMPAREOBJECTS => "NtCompareObjects",
        WIN64_NTCOMPARESIGNINGLEVELS => "NtCompareSigningLevels",
        WIN64_NTCOMPARETOKENS => "NtCompareTokens",
        WIN64_NTCOMPLETECONNECTPORT => "NtCompleteConnectPort",
        WIN64_NTCOMPRESSKEY => "NtCompressKey",
        WIN64_NTCONNECTPORT => "NtConnectPort",
        WIN64_NTCONTINUEEX => "NtContinueEx",
        WIN64_NTCONVERTBETWEENAUXILIARYCOUNTERANDPERFORMANCECOUNTER => {
            "NtConvertBetweenAuxiliaryCounterAndPerformanceCounter"
        }
        WIN64_NTCOPYFILECHUNK => "NtCopyFileChunk",
        WIN64_NTCREATECPUPARTITION => "NtCreateCpuPartition",
        WIN64_NTCREATECROSSVMEVENT => "NtCreateCrossVmEvent",
        WIN64_NTCREATECROSSVMMUTANT => "NtCreateCrossVmMutant",
        WIN64_NTCREATEDEBUGOBJECT => "NtCreateDebugObject",
        WIN64_NTCREATEDIRECTORYOBJECT => "NtCreateDirectoryObject",
        WIN64_NTCREATEDIRECTORYOBJECTEX => "NtCreateDirectoryObjectEx",
        WIN64_NTCREATEENCLAVE => "NtCreateEnclave",
        WIN64_NTCREATEENLISTMENT => "NtCreateEnlistment",
        WIN64_NTCREATEEVENTPAIR => "NtCreateEventPair",
        WIN64_NTCREATEIRTIMER => "NtCreateIRTimer",
        WIN64_NTCREATEIOCOMPLETION => "NtCreateIoCompletion",
        WIN64_NTCREATEIORING => "NtCreateIoRing",
        WIN64_NTCREATEJOBOBJECT => "NtCreateJobObject",
        WIN64_NTCREATEJOBSET => "NtCreateJobSet",
        WIN64_NTCREATEKEYTRANSACTED => "NtCreateKeyTransacted",
        WIN64_NTCREATEKEYEDEVENT => "NtCreateKeyedEvent",
        WIN64_NTCREATELOWBOXTOKEN => "NtCreateLowBoxToken",
        WIN64_NTCREATEMAILSLOTFILE => "NtCreateMailslotFile",
        WIN64_NTCREATEMUTANT => "NtCreateMutant",
        WIN64_NTCREATENAMEDPIPEFILE => "NtCreateNamedPipeFile",
        WIN64_NTCREATEPAGINGFILE => "NtCreatePagingFile",
        WIN64_NTCREATEPARTITION => "NtCreatePartition",
        WIN64_NTCREATEPORT => "NtCreatePort",
        WIN64_NTCREATEPRIVATENAMESPACE => "NtCreatePrivateNamespace",
        WIN64_NTCREATEPROCESS => "NtCreateProcess",
        WIN64_NTCREATEPROCESSSTATECHANGE => "NtCreateProcessStateChange",
        WIN64_NTCREATEPROFILE => "NtCreateProfile",
        WIN64_NTCREATEPROFILEEX => "NtCreateProfileEx",
        WIN64_NTCREATEREGISTRYTRANSACTION => "NtCreateRegistryTransaction",
        WIN64_NTCREATERESOURCEMANAGER => "NtCreateResourceManager",
        WIN64_NTCREATESECTIONEX => "NtCreateSectionEx",
        WIN64_NTCREATESEMAPHORE => "NtCreateSemaphore",
        WIN64_NTCREATESYMBOLICLINKOBJECT => "NtCreateSymbolicLinkObject",
        WIN64_NTCREATETHREADEX => "NtCreateThreadEx",
        WIN64_NTCREATETHREADSTATECHANGE => "NtCreateThreadStateChange",
        WIN64_NTCREATETIMER => "NtCreateTimer",
        WIN64_NTCREATETIMER2 => "NtCreateTimer2",
        WIN64_NTCREATETOKEN => "NtCreateToken",
        WIN64_NTCREATETOKENEX => "NtCreateTokenEx",
        WIN64_NTCREATETRANSACTION => "NtCreateTransaction",
        WIN64_NTCREATETRANSACTIONMANAGER => "NtCreateTransactionManager",
        WIN64_NTCREATEUSERPROCESS => "NtCreateUserProcess",
        WIN64_NTCREATEWAITCOMPLETIONPACKET => "NtCreateWaitCompletionPacket",
        WIN64_NTCREATEWAITABLEPORT => "NtCreateWaitablePort",
        WIN64_NTCREATEWNFSTATENAME => "NtCreateWnfStateName",
        WIN64_NTCREATEWORKERFACTORY => "NtCreateWorkerFactory",
        WIN64_NTDEBUGACTIVEPROCESS => "NtDebugActiveProcess",
        WIN64_NTDEBUGCONTINUE => "NtDebugContinue",
        WIN64_NTDELETEATOM => "NtDeleteAtom",
        WIN64_NTDELETEBOOTENTRY => "NtDeleteBootEntry",
        WIN64_NTDELETEDRIVERENTRY => "NtDeleteDriverEntry",
        WIN64_NTDELETEFILE => "NtDeleteFile",
        WIN64_NTDELETEKEY => "NtDeleteKey",
        WIN64_NTDELETEOBJECTAUDITALARM => "NtDeleteObjectAuditAlarm",
        WIN64_NTDELETEPRIVATENAMESPACE => "NtDeletePrivateNamespace",
        WIN64_NTDELETEVALUEKEY => "NtDeleteValueKey",
        WIN64_NTDELETEWNFSTATEDATA => "NtDeleteWnfStateData",
        WIN64_NTDELETEWNFSTATENAME => "NtDeleteWnfStateName",
        WIN64_NTDIRECTGRAPHICSCALL => "NtDirectGraphicsCall",
        WIN64_NTDISABLELASTKNOWNGOOD => "NtDisableLastKnownGood",
        WIN64_NTDISPLAYSTRING => "NtDisplayString",
        WIN64_NTDRAWTEXT => "NtDrawText",
        WIN64_NTENABLELASTKNOWNGOOD => "NtEnableLastKnownGood",
        WIN64_NTENUMERATEBOOTENTRIES => "NtEnumerateBootEntries",
        WIN64_NTENUMERATEDRIVERENTRIES => "NtEnumerateDriverEntries",
        WIN64_NTENUMERATESYSTEMENVIRONMENTVALUESEX => "NtEnumerateSystemEnvironmentValuesEx",
        WIN64_NTENUMERATETRANSACTIONOBJECT => "NtEnumerateTransactionObject",
        WIN64_NTEXTENDSECTION => "NtExtendSection",
        WIN64_NTFILTERBOOTOPTION => "NtFilterBootOption",
        WIN64_NTFILTERTOKEN => "NtFilterToken",
        WIN64_NTFILTERTOKENEX => "NtFilterTokenEx",
        WIN64_NTFLUSHBUFFERSFILEEX => "NtFlushBuffersFileEx",
        WIN64_NTFLUSHINSTALLUILANGUAGE => "NtFlushInstallUILanguage",
        WIN64_NTFLUSHINSTRUCTIONCACHE => "NtFlushInstructionCache",
        WIN64_NTFLUSHKEY => "NtFlushKey",
        WIN64_NTFLUSHPROCESSWRITEBUFFERS => "NtFlushProcessWriteBuffers",
        WIN64_NTFLUSHVIRTUALMEMORY => "NtFlushVirtualMemory",
        WIN64_NTFLUSHWRITEBUFFER => "NtFlushWriteBuffer",
        WIN64_NTFREEUSERPHYSICALPAGES => "NtFreeUserPhysicalPages",
        WIN64_NTFREEZEREGISTRY => "NtFreezeRegistry",
        WIN64_NTFREEZETRANSACTIONS => "NtFreezeTransactions",
        WIN64_NTGETCACHEDSIGNINGLEVEL => "NtGetCachedSigningLevel",
        WIN64_NTGETCOMPLETEWNFSTATESUBSCRIPTION => "NtGetCompleteWnfStateSubscription",
        WIN64_NTGETCONTEXTTHREAD => "NtGetContextThread",
        WIN64_NTGETCURRENTPROCESSORNUMBER => "NtGetCurrentProcessorNumber",
        WIN64_NTGETCURRENTPROCESSORNUMBEREX => "NtGetCurrentProcessorNumberEx",
        WIN64_NTGETDEVICEPOWERSTATE => "NtGetDevicePowerState",
        WIN64_NTGETMUIREGISTRYINFO => "NtGetMUIRegistryInfo",
        WIN64_NTGETNEXTPROCESS => "NtGetNextProcess",
        WIN64_NTGETNEXTTHREAD => "NtGetNextThread",
        WIN64_NTGETNLSSECTIONPTR => "NtGetNlsSectionPtr",
        WIN64_NTGETNOTIFICATIONRESOURCEMANAGER => "NtGetNotificationResourceManager",
        WIN64_NTGETWRITEWATCH => "NtGetWriteWatch",
        WIN64_NTIMPERSONATEANONYMOUSTOKEN => "NtImpersonateAnonymousToken",
        WIN64_NTIMPERSONATETHREAD => "NtImpersonateThread",
        WIN64_NTINITIALIZEENCLAVE => "NtInitializeEnclave",
        WIN64_NTINITIALIZENLSFILES => "NtInitializeNlsFiles",
        WIN64_NTINITIALIZEREGISTRY => "NtInitializeRegistry",
        WIN64_NTINITIATEPOWERACTION => "NtInitiatePowerAction",
        WIN64_NTISSYSTEMRESUMEAUTOMATIC => "NtIsSystemResumeAutomatic",
        WIN64_NTISUILANGUAGECOMITTED => "NtIsUILanguageComitted",
        WIN64_NTLISTENPORT => "NtListenPort",
        WIN64_NTLOADDRIVER => "NtLoadDriver",
        WIN64_NTLOADENCLAVEDATA => "NtLoadEnclaveData",
        WIN64_NTLOADKEY => "NtLoadKey",
        WIN64_NTLOADKEY2 => "NtLoadKey2",
        WIN64_NTLOADKEY3 => "NtLoadKey3",
        WIN64_NTLOADKEYEX => "NtLoadKeyEx",
        WIN64_NTLOCKFILE => "NtLockFile",
        WIN64_NTLOCKPRODUCTACTIVATIONKEYS => "NtLockProductActivationKeys",
        WIN64_NTLOCKREGISTRYKEY => "NtLockRegistryKey",
        WIN64_NTLOCKVIRTUALMEMORY => "NtLockVirtualMemory",
        WIN64_NTMAKEPERMANENTOBJECT => "NtMakePermanentObject",
        WIN64_NTMAKETEMPORARYOBJECT => "NtMakeTemporaryObject",
        WIN64_NTMANAGEHOTPATCH => "NtManageHotPatch",
        WIN64_NTMANAGEPARTITION => "NtManagePartition",
        WIN64_NTMAPCMFMODULE => "NtMapCMFModule",
        WIN64_NTMAPUSERPHYSICALPAGES => "NtMapUserPhysicalPages",
        WIN64_NTMAPVIEWOFSECTIONEX => "NtMapViewOfSectionEx",
        WIN64_NTMODIFYBOOTENTRY => "NtModifyBootEntry",
        WIN64_NTMODIFYDRIVERENTRY => "NtModifyDriverEntry",
        WIN64_NTNOTIFYCHANGEDIRECTORYFILE => "NtNotifyChangeDirectoryFile",
        WIN64_NTNOTIFYCHANGEDIRECTORYFILEEX => "NtNotifyChangeDirectoryFileEx",
        WIN64_NTNOTIFYCHANGEKEY => "NtNotifyChangeKey",
        WIN64_NTNOTIFYCHANGEMULTIPLEKEYS => "NtNotifyChangeMultipleKeys",
        WIN64_NTNOTIFYCHANGESESSION => "NtNotifyChangeSession",
        WIN64_NTOPENCPUPARTITION => "NtOpenCpuPartition",
        WIN64_NTOPENENLISTMENT => "NtOpenEnlistment",
        WIN64_NTOPENEVENTPAIR => "NtOpenEventPair",
        WIN64_NTOPENIOCOMPLETION => "NtOpenIoCompletion",
        WIN64_NTOPENJOBOBJECT => "NtOpenJobObject",
        WIN64_NTOPENKEYEX => "NtOpenKeyEx",
        WIN64_NTOPENKEYTRANSACTED => "NtOpenKeyTransacted",
        WIN64_NTOPENKEYTRANSACTEDEX => "NtOpenKeyTransactedEx",
        WIN64_NTOPENKEYEDEVENT => "NtOpenKeyedEvent",
        WIN64_NTOPENMUTANT => "NtOpenMutant",
        WIN64_NTOPENOBJECTAUDITALARM => "NtOpenObjectAuditAlarm",
        WIN64_NTOPENPARTITION => "NtOpenPartition",
        WIN64_NTOPENPRIVATENAMESPACE => "NtOpenPrivateNamespace",
        WIN64_NTOPENPROCESSTOKEN => "NtOpenProcessToken",
        WIN64_NTOPENREGISTRYTRANSACTION => "NtOpenRegistryTransaction",
        WIN64_NTOPENRESOURCEMANAGER => "NtOpenResourceManager",
        WIN64_NTOPENSEMAPHORE => "NtOpenSemaphore",
        WIN64_NTOPENSESSION => "NtOpenSession",
        WIN64_NTOPENSYMBOLICLINKOBJECT => "NtOpenSymbolicLinkObject",
        WIN64_NTOPENTHREAD => "NtOpenThread",
        WIN64_NTOPENTIMER => "NtOpenTimer",
        WIN64_NTOPENTRANSACTION => "NtOpenTransaction",
        WIN64_NTOPENTRANSACTIONMANAGER => "NtOpenTransactionManager",
        WIN64_NTPLUGPLAYCONTROL => "NtPlugPlayControl",
        WIN64_NTPREPREPARECOMPLETE => "NtPrePrepareComplete",
        WIN64_NTPREPREPAREENLISTMENT => "NtPrePrepareEnlistment",
        WIN64_NTPREPARECOMPLETE => "NtPrepareComplete",
        WIN64_NTPREPAREENLISTMENT => "NtPrepareEnlistment",
        WIN64_NTPRIVILEGECHECK => "NtPrivilegeCheck",
        WIN64_NTPRIVILEGEOBJECTAUDITALARM => "NtPrivilegeObjectAuditAlarm",
        WIN64_NTPRIVILEGEDSERVICEAUDITALARM => "NtPrivilegedServiceAuditAlarm",
        WIN64_NTPROPAGATIONCOMPLETE => "NtPropagationComplete",
        WIN64_NTPROPAGATIONFAILED => "NtPropagationFailed",
        WIN64_NTPSSCAPTUREVASPACEBULK => "NtPssCaptureVaSpaceBulk",
        WIN64_NTPULSEEVENT => "NtPulseEvent",
        WIN64_NTQUERYAUXILIARYCOUNTERFREQUENCY => "NtQueryAuxiliaryCounterFrequency",
        WIN64_NTQUERYBOOTENTRYORDER => "NtQueryBootEntryOrder",
        WIN64_NTQUERYBOOTOPTIONS => "NtQueryBootOptions",
        WIN64_NTQUERYDEBUGFILTERSTATE => "NtQueryDebugFilterState",
        WIN64_NTQUERYDIRECTORYFILEEX => "NtQueryDirectoryFileEx",
        WIN64_NTQUERYDIRECTORYOBJECT => "NtQueryDirectoryObject",
        WIN64_NTQUERYDRIVERENTRYORDER => "NtQueryDriverEntryOrder",
        WIN64_NTQUERYEAFILE => "NtQueryEaFile",
        WIN64_NTQUERYFULLATTRIBUTESFILE => "NtQueryFullAttributesFile",
        WIN64_NTQUERYINFORMATIONATOM => "NtQueryInformationAtom",
        WIN64_NTQUERYINFORMATIONBYNAME => "NtQueryInformationByName",
        WIN64_NTQUERYINFORMATIONCPUPARTITION => "NtQueryInformationCpuPartition",
        WIN64_NTQUERYINFORMATIONENLISTMENT => "NtQueryInformationEnlistment",
        WIN64_NTQUERYINFORMATIONJOBOBJECT => "NtQueryInformationJobObject",
        WIN64_NTQUERYINFORMATIONPORT => "NtQueryInformationPort",
        WIN64_NTQUERYINFORMATIONRESOURCEMANAGER => "NtQueryInformationResourceManager",
        WIN64_NTQUERYINFORMATIONTRANSACTION => "NtQueryInformationTransaction",
        WIN64_NTQUERYINFORMATIONTRANSACTIONMANAGER => "NtQueryInformationTransactionManager",
        WIN64_NTQUERYINFORMATIONWORKERFACTORY => "NtQueryInformationWorkerFactory",
        WIN64_NTQUERYINSTALLUILANGUAGE => "NtQueryInstallUILanguage",
        WIN64_NTQUERYINTERVALPROFILE => "NtQueryIntervalProfile",
        WIN64_NTQUERYIOCOMPLETION => "NtQueryIoCompletion",
        WIN64_NTQUERYIORINGCAPABILITIES => "NtQueryIoRingCapabilities",
        WIN64_NTQUERYLICENSEVALUE => "NtQueryLicenseValue",
        WIN64_NTQUERYMULTIPLEVALUEKEY => "NtQueryMultipleValueKey",
        WIN64_NTQUERYMUTANT => "NtQueryMutant",
        WIN64_NTQUERYOPENSUBKEYS => "NtQueryOpenSubKeys",
        WIN64_NTQUERYOPENSUBKEYSEX => "NtQueryOpenSubKeysEx",
        WIN64_NTQUERYPORTINFORMATIONPROCESS => "NtQueryPortInformationProcess",
        WIN64_NTQUERYQUOTAINFORMATIONFILE => "NtQueryQuotaInformationFile",
        WIN64_NTQUERYSECURITYATTRIBUTESTOKEN => "NtQuerySecurityAttributesToken",
        WIN64_NTQUERYSECURITYOBJECT => "NtQuerySecurityObject",
        WIN64_NTQUERYSECURITYPOLICY => "NtQuerySecurityPolicy",
        WIN64_NTQUERYSEMAPHORE => "NtQuerySemaphore",
        WIN64_NTQUERYSYMBOLICLINKOBJECT => "NtQuerySymbolicLinkObject",
        WIN64_NTQUERYSYSTEMENVIRONMENTVALUE => "NtQuerySystemEnvironmentValue",
        WIN64_NTQUERYSYSTEMENVIRONMENTVALUEEX => "NtQuerySystemEnvironmentValueEx",
        WIN64_NTQUERYSYSTEMINFORMATIONEX => "NtQuerySystemInformationEx",
        WIN64_NTQUERYTIMERRESOLUTION => "NtQueryTimerResolution",
        WIN64_NTQUERYWNFSTATEDATA => "NtQueryWnfStateData",
        WIN64_NTQUERYWNFSTATENAMEINFORMATION => "NtQueryWnfStateNameInformation",
        WIN64_NTQUEUEAPCTHREADEX => "NtQueueApcThreadEx",
        WIN64_NTQUEUEAPCTHREADEX2 => "NtQueueApcThreadEx2",
        WIN64_NTRAISEEXCEPTION => "NtRaiseException",
        WIN64_NTRAISEHARDERROR => "NtRaiseHardError",
        WIN64_NTREADONLYENLISTMENT => "NtReadOnlyEnlistment",
        WIN64_NTREADVIRTUALMEMORYEX => "NtReadVirtualMemoryEx",
        WIN64_NTRECOVERENLISTMENT => "NtRecoverEnlistment",
        WIN64_NTRECOVERRESOURCEMANAGER => "NtRecoverResourceManager",
        WIN64_NTRECOVERTRANSACTIONMANAGER => "NtRecoverTransactionManager",
        WIN64_NTREGISTERPROTOCOLADDRESSINFORMATION => "NtRegisterProtocolAddressInformation",
        WIN64_NTREGISTERTHREADTERMINATEPORT => "NtRegisterThreadTerminatePort",
        WIN64_NTRELEASEKEYEDEVENT => "NtReleaseKeyedEvent",
        WIN64_NTRELEASEWORKERFACTORYWORKER => "NtReleaseWorkerFactoryWorker",
        WIN64_NTREMOVEIOCOMPLETIONEX => "NtRemoveIoCompletionEx",
        WIN64_NTREMOVEPROCESSDEBUG => "NtRemoveProcessDebug",
        WIN64_NTRENAMEKEY => "NtRenameKey",
        WIN64_NTRENAMETRANSACTIONMANAGER => "NtRenameTransactionManager",
        WIN64_NTREPLACEKEY => "NtReplaceKey",
        WIN64_NTREPLACEPARTITIONUNIT => "NtReplacePartitionUnit",
        WIN64_NTREPLYWAITREPLYPORT => "NtReplyWaitReplyPort",
        WIN64_NTREQUESTPORT => "NtRequestPort",
        WIN64_NTRESETEVENT => "NtResetEvent",
        WIN64_NTRESETWRITEWATCH => "NtResetWriteWatch",
        WIN64_NTRESTOREKEY => "NtRestoreKey",
        WIN64_NTRESUMEPROCESS => "NtResumeProcess",
        WIN64_NTREVERTCONTAINERIMPERSONATION => "NtRevertContainerImpersonation",
        WIN64_NTROLLBACKCOMPLETE => "NtRollbackComplete",
        WIN64_NTROLLBACKENLISTMENT => "NtRollbackEnlistment",
        WIN64_NTROLLBACKREGISTRYTRANSACTION => "NtRollbackRegistryTransaction",
        WIN64_NTROLLBACKTRANSACTION => "NtRollbackTransaction",
        WIN64_NTROLLFORWARDTRANSACTIONMANAGER => "NtRollforwardTransactionManager",
        WIN64_NTSAVEKEY => "NtSaveKey",
        WIN64_NTSAVEKEYEX => "NtSaveKeyEx",
        WIN64_NTSAVEMERGEDKEYS => "NtSaveMergedKeys",
        WIN64_NTSECURECONNECTPORT => "NtSecureConnectPort",
        WIN64_NTSERIALIZEBOOT => "NtSerializeBoot",
        WIN64_NTSETBOOTENTRYORDER => "NtSetBootEntryOrder",
        WIN64_NTSETBOOTOPTIONS => "NtSetBootOptions",
        WIN64_NTSETCACHEDSIGNINGLEVEL => "NtSetCachedSigningLevel",
        WIN64_NTSETCACHEDSIGNINGLEVEL2 => "NtSetCachedSigningLevel2",
        WIN64_NTSETCONTEXTTHREAD => "NtSetContextThread",
        WIN64_NTSETDEBUGFILTERSTATE => "NtSetDebugFilterState",
        WIN64_NTSETDEFAULTHARDERRORPORT => "NtSetDefaultHardErrorPort",
        WIN64_NTSETDEFAULTLOCALE => "NtSetDefaultLocale",
        WIN64_NTSETDEFAULTUILANGUAGE => "NtSetDefaultUILanguage",
        WIN64_NTSETDRIVERENTRYORDER => "NtSetDriverEntryOrder",
        WIN64_NTSETEAFILE => "NtSetEaFile",
        WIN64_NTSETEVENTEX => "NtSetEventEx",
        WIN64_NTSETHIGHEVENTPAIR => "NtSetHighEventPair",
        WIN64_NTSETHIGHWAITLOWEVENTPAIR => "NtSetHighWaitLowEventPair",
        WIN64_NTSETIRTIMER => "NtSetIRTimer",
        WIN64_NTSETINFORMATIONCPUPARTITION => "NtSetInformationCpuPartition",
        WIN64_NTSETINFORMATIONDEBUGOBJECT => "NtSetInformationDebugObject",
        WIN64_NTSETINFORMATIONENLISTMENT => "NtSetInformationEnlistment",
        WIN64_NTSETINFORMATIONIORING => "NtSetInformationIoRing",
        WIN64_NTSETINFORMATIONJOBOBJECT => "NtSetInformationJobObject",
        WIN64_NTSETINFORMATIONKEY => "NtSetInformationKey",
        WIN64_NTSETINFORMATIONRESOURCEMANAGER => "NtSetInformationResourceManager",
        WIN64_NTSETINFORMATIONSYMBOLICLINK => "NtSetInformationSymbolicLink",
        WIN64_NTSETINFORMATIONTOKEN => "NtSetInformationToken",
        WIN64_NTSETINFORMATIONTRANSACTION => "NtSetInformationTransaction",
        WIN64_NTSETINFORMATIONTRANSACTIONMANAGER => "NtSetInformationTransactionManager",
        WIN64_NTSETINFORMATIONVIRTUALMEMORY => "NtSetInformationVirtualMemory",
        WIN64_NTSETINFORMATIONWORKERFACTORY => "NtSetInformationWorkerFactory",
        WIN64_NTSETINTERVALPROFILE => "NtSetIntervalProfile",
        WIN64_NTSETIOCOMPLETION => "NtSetIoCompletion",
        WIN64_NTSETIOCOMPLETIONEX => "NtSetIoCompletionEx",
        WIN64_NTSETLDTENTRIES => "NtSetLdtEntries",
        WIN64_NTSETLOWEVENTPAIR => "NtSetLowEventPair",
        WIN64_NTSETLOWWAITHIGHEVENTPAIR => "NtSetLowWaitHighEventPair",
        WIN64_NTSETQUOTAINFORMATIONFILE => "NtSetQuotaInformationFile",
        WIN64_NTSETSECURITYOBJECT => "NtSetSecurityObject",
        WIN64_NTSETSYSTEMENVIRONMENTVALUE => "NtSetSystemEnvironmentValue",
        WIN64_NTSETSYSTEMENVIRONMENTVALUEEX => "NtSetSystemEnvironmentValueEx",
        WIN64_NTSETSYSTEMINFORMATION => "NtSetSystemInformation",
        WIN64_NTSETSYSTEMPOWERSTATE => "NtSetSystemPowerState",
        WIN64_NTSETSYSTEMTIME => "NtSetSystemTime",
        WIN64_NTSETTHREADEXECUTIONSTATE => "NtSetThreadExecutionState",
        WIN64_NTSETTIMER2 => "NtSetTimer2",
        WIN64_NTSETTIMEREX => "NtSetTimerEx",
        WIN64_NTSETTIMERRESOLUTION => "NtSetTimerResolution",
        WIN64_NTSETUUIDSEED => "NtSetUuidSeed",
        WIN64_NTSETVOLUMEINFORMATIONFILE => "NtSetVolumeInformationFile",
        WIN64_NTSETWNFPROCESSNOTIFICATIONEVENT => "NtSetWnfProcessNotificationEvent",
        WIN64_NTSHUTDOWNSYSTEM => "NtShutdownSystem",
        WIN64_NTSHUTDOWNWORKERFACTORY => "NtShutdownWorkerFactory",
        WIN64_NTSIGNALANDWAITFORSINGLEOBJECT => "NtSignalAndWaitForSingleObject",
        WIN64_NTSINGLEPHASEREJECT => "NtSinglePhaseReject",
        WIN64_NTSTARTPROFILE => "NtStartProfile",
        WIN64_NTSTOPPROFILE => "NtStopProfile",
        WIN64_NTSUBMITIORING => "NtSubmitIoRing",
        WIN64_NTSUBSCRIBEWNFSTATECHANGE => "NtSubscribeWnfStateChange",
        WIN64_NTSUSPENDPROCESS => "NtSuspendProcess",
        WIN64_NTSUSPENDTHREAD => "NtSuspendThread",
        WIN64_NTSYSTEMDEBUGCONTROL => "NtSystemDebugControl",
        WIN64_NTTERMINATEENCLAVE => "NtTerminateEnclave",
        WIN64_NTTERMINATEJOBOBJECT => "NtTerminateJobObject",
        WIN64_NTTESTALERT => "NtTestAlert",
        WIN64_NTTHAWREGISTRY => "NtThawRegistry",
        WIN64_NTTHAWTRANSACTIONS => "NtThawTransactions",
        WIN64_NTTRACECONTROL => "NtTraceControl",
        WIN64_NTTRANSLATEFILEPATH => "NtTranslateFilePath",
        WIN64_NTUMSTHREADYIELD => "NtUmsThreadYield",
        WIN64_NTUNLOADDRIVER => "NtUnloadDriver",
        WIN64_NTUNLOADKEY => "NtUnloadKey",
        WIN64_NTUNLOADKEY2 => "NtUnloadKey2",
        WIN64_NTUNLOADKEYEX => "NtUnloadKeyEx",
        WIN64_NTUNLOCKFILE => "NtUnlockFile",
        WIN64_NTUNLOCKVIRTUALMEMORY => "NtUnlockVirtualMemory",
        WIN64_NTUNMAPVIEWOFSECTIONEX => "NtUnmapViewOfSectionEx",
        WIN64_NTUNSUBSCRIBEWNFSTATECHANGE => "NtUnsubscribeWnfStateChange",
        WIN64_NTUPDATEWNFSTATEDATA => "NtUpdateWnfStateData",
        WIN64_NTVDMCONTROL => "NtVdmControl",
        WIN64_NTWAITFORALERTBYTHREADID => "NtWaitForAlertByThreadId",
        WIN64_NTWAITFORDEBUGEVENT => "NtWaitForDebugEvent",
        WIN64_NTWAITFORKEYEDEVENT => "NtWaitForKeyedEvent",
        WIN64_NTWAITFORWORKVIAWORKERFACTORY => "NtWaitForWorkViaWorkerFactory",
        WIN64_NTWAITHIGHEVENTPAIR => "NtWaitHighEventPair",
        WIN64_NTWAITLOWEVENTPAIR => "NtWaitLowEventPair",
        _ => return format!("NtSyscall_unknown(0x{sys:x})"),
    }
    .to_owned()
}
