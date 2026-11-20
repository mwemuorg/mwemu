//! Shared Windows memory semantics. ABI adaptation is kept at the boundary.
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::{
    ALLOCATION_GRANULARITY, MemoryError, MemoryKind, PAGE_SIZE, align_up, page_range,
};
use crate::winapi::common::process::require_memory_access;
use crate::winapi::common::virtual_protect;
use crate::winapi::common::{nt_api, sections, trace};
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::memory::*;
use crate::windows::constants::process::*;

const PAGE_MASK: u32 = 0xff;
const PAGE_MODIFIERS: u32 = 0x4000_0700;

pub(crate) fn pointer_size(emu: &Emu) -> u64 {
    if emu.cfg.arch.is_64bits() { 8 } else { 4 }
}

/// Copy a guest buffer that may straddle adjacent readable mappings.
pub(crate) fn read_guest_bytes(emu: &Emu, address: u64, length: usize) -> Option<Vec<u8>> {
    if !emu.maps.can_access_range(address, length, Permission::READ) {
        return None;
    }
    let mut bytes = Vec::with_capacity(length);
    while bytes.len() < length {
        let cursor = address.checked_add(bytes.len() as u64)?;
        let map = emu.maps.get_mem_by_addr(cursor)?;
        let count = (map.get_bottom() - cursor).min((length - bytes.len()) as u64) as usize;
        let chunk = map.read_bytes(cursor, count);
        if chunk.len() != count || count == 0 {
            return None;
        }
        bytes.extend_from_slice(&chunk);
    }
    Some(bytes)
}

/// Read a small fixed-size value across adjacent guest mappings.
pub(crate) fn read_array<const N: usize>(emu: &Emu, address: u64) -> Option<[u8; N]> {
    let mut bytes = [0; N];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = emu.maps.read_byte(address.checked_add(index as u64)?)?;
    }
    Some(bytes)
}

pub(crate) fn read_pointer(emu: &Emu, addr: u64) -> Option<u64> {
    if pointer_size(emu) == 8 {
        emu.maps.read_qword(addr)
    } else {
        emu.maps.read_dword(addr).map(u64::from)
    }
}

pub(crate) fn write_pointer(emu: &mut Emu, addr: u64, value: u64) -> bool {
    if pointer_size(emu) == 8 {
        emu.maps.write_qword(addr, value)
    } else {
        emu.maps.write_dword(addr, value as u32)
    }
}

pub(crate) fn writable_pointer(emu: &Emu, addr: u64) -> bool {
    emu.maps.validate_write_range(addr, pointer_size(emu))
}

pub(crate) fn permissions(protection: u32) -> Option<Permission> {
    let base = protection & PAGE_MASK;
    if !base.is_power_of_two() || protection & !(PAGE_MASK | PAGE_MODIFIERS) != 0 {
        return None;
    }
    let execute = base >= PAGE_EXECUTE;
    let read = base != PAGE_NOACCESS && base != PAGE_EXECUTE;
    let write = matches!(
        base,
        PAGE_READWRITE | PAGE_WRITECOPY | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY
    );
    Some(Permission::from_flags(read, write, execute))
}

fn status(error: MemoryError) -> u64 {
    match error {
        MemoryError::InvalidParameter => STATUS_INVALID_PARAMETER,
        MemoryError::Conflict => STATUS_CONFLICTING_ADDRESSES,
        MemoryError::Unmapped => STATUS_INVALID_ADDRESS,
        MemoryError::NotCommitted => STATUS_NOT_COMMITTED,
        MemoryError::NoMemory => STATUS_NO_MEMORY,
    }
}

fn win32_error(error: MemoryError) -> u64 {
    match error {
        MemoryError::InvalidParameter => ERROR_INVALID_PARAMETER,
        MemoryError::NoMemory => ERROR_NOT_ENOUGH_MEMORY,
        _ => ERROR_INVALID_ADDRESS,
    }
}

fn address_limit(emu: &Emu) -> u64 {
    if emu.cfg.arch.is_64bits() {
        USER64_ADDRESS_LIMIT
    } else {
        USER32_ADDRESS_LIMIT
    }
}

pub(crate) fn allocate(
    emu: &mut Emu,
    address: u64,
    size: u64,
    flags: u32,
    protection: u32,
) -> Result<(u64, u64), MemoryError> {
    let permission = permissions(protection).ok_or(MemoryError::InvalidParameter)?;
    if size == 0
        || flags & !(MEM_RESERVE | MEM_COMMIT | MEM_TOP_DOWN) != 0
        || flags & (MEM_RESERVE | MEM_COMMIT) == 0
    {
        return Err(MemoryError::InvalidParameter);
    }
    let reserve = flags & MEM_RESERVE != 0 || address == 0;
    let start = if reserve {
        address & !(ALLOCATION_GRANULARITY - 1)
    } else {
        address & !(PAGE_SIZE - 1)
    };
    let end = address
        .checked_add(size)
        .and_then(|n| align_up(n, PAGE_SIZE))
        .ok_or(MemoryError::InvalidParameter)?;
    let length = end
        .checked_sub(start)
        .ok_or(MemoryError::InvalidParameter)?;
    if end > address_limit(emu) {
        return Err(MemoryError::InvalidParameter);
    }
    if !reserve {
        emu.maps
            .commit_memory(start, length, protection, permission)
            .map_err(|e| match e {
                MemoryError::Unmapped => MemoryError::InvalidParameter,
                other => other,
            })?;
        return Ok((start, length));
    }
    if flags & MEM_COMMIT != 0 && length > emu.maps.max_alloc_size {
        return Err(MemoryError::NoMemory);
    }
    let base = if address == 0 {
        let bottom = if emu.cfg.arch.is_64bits() {
            ALLOC64_MIN
        } else {
            ALLOC32_MIN
        };
        let top = if emu.cfg.arch.is_64bits() {
            ALLOC64_MAX
        } else {
            ALLOC32_MAX
        };
        let address = if flags & MEM_TOP_DOWN != 0 {
            emu.maps
                .find_free_range_top_down(length, ALLOCATION_GRANULARITY, bottom, top)
        } else {
            emu.maps
                .find_free_range(length, ALLOCATION_GRANULARITY, bottom, top)
        };
        address.ok_or(MemoryError::NoMemory)?
    } else {
        start
    };
    emu.maps.reserve_private(
        base,
        length,
        protection,
        permission,
        flags & MEM_COMMIT != 0,
    )?;
    Ok((base, length))
}

pub(crate) fn free(
    emu: &mut Emu,
    address: u64,
    size: u64,
    flags: u32,
) -> Result<(u64, u64), MemoryError> {
    match flags {
        MEM_RELEASE if size == 0 => {
            emu.maps.release_memory(address)?;
            Ok((address, 0))
        }
        MEM_DECOMMIT => {
            let (base, length) = if size == 0 {
                let info = emu.maps.query_memory(address);
                if info.allocation_base != address || info.state.is_none() {
                    return Err(MemoryError::InvalidParameter);
                }
                let size = emu
                    .maps
                    .allocation_at(address)
                    .map_or(info.size, |a| a.size);
                (address, size)
            } else {
                page_range(address, size)?
            };
            emu.maps.decommit_memory(base, length)?;
            Ok((base, length))
        }
        _ => Err(MemoryError::InvalidParameter),
    }
}

pub(crate) fn virtual_alloc(emu: &mut Emu, remote: bool, numa: bool) {
    let call = WinApi(Stdcall);
    let offset = remote as usize;
    let count = 4 + offset + numa as usize;
    if remote
        && let Err(status) =
            require_memory_access(emu, api_try!(emu, call.arg(emu, 0)), PROCESS_VM_OPERATION)
    {
        emu.set_last_error(if status == STATUS_ACCESS_DENIED {
            ERROR_ACCESS_DENIED
        } else {
            ERROR_INVALID_HANDLE
        });
        call.set_return(emu, 0);
        call.cleanup_args(emu, count);
        return;
    }
    let args = (
        api_try!(emu, call.arg(emu, offset)),
        api_try!(emu, call.arg(emu, offset + 1)),
        api_try!(emu, call.arg_u32(emu, offset + 2)),
        api_try!(emu, call.arg_u32(emu, offset + 3)),
    );
    let result = allocate(emu, args.0, args.1, args.2, args.3);
    let value = match result {
        Ok((base, _)) => base,
        Err(e) => {
            emu.set_last_error(win32_error(e));
            0
        }
    };
    call.set_return(emu, value);
    call.cleanup_args(emu, count);
}

pub(crate) fn virtual_free(emu: &mut Emu, remote: bool) {
    let call = WinApi(Stdcall);
    let offset = remote as usize;
    let count = 3 + offset;
    if remote
        && let Err(status) =
            require_memory_access(emu, api_try!(emu, call.arg(emu, 0)), PROCESS_VM_OPERATION)
    {
        emu.set_last_error(if status == STATUS_ACCESS_DENIED {
            ERROR_ACCESS_DENIED
        } else {
            ERROR_INVALID_HANDLE
        });
        call.set_return(emu, 0);
        call.cleanup_args(emu, count);
        return;
    }
    let result = free(
        emu,
        api_try!(emu, call.arg(emu, offset)),
        api_try!(emu, call.arg(emu, offset + 1)),
        api_try!(emu, call.arg_u32(emu, offset + 2)),
    );
    let value = match result {
        Ok(_) => 1,
        Err(e) => {
            emu.set_last_error(win32_error(e));
            0
        }
    };
    call.set_return(emu, value);
    call.cleanup_args(emu, count);
}

pub(crate) fn nt_allocate(emu: &mut Emu, call: WindowsAbi, extended: bool) {
    let process = api_try!(emu, call.arg(emu, 0));
    let extra_count = if extended {
        api_try!(emu, call.arg_u32(emu, 6))
    } else {
        0
    };
    let zero_bits = if extended {
        0
    } else {
        api_try!(emu, call.arg(emu, 2))
    };
    let base_ptr = api_try!(emu, call.arg(emu, 1));
    let size_ptr = api_try!(emu, call.arg(emu, if extended { 2 } else { 3 }));
    let flags = api_try!(emu, call.arg_u32(emu, if extended { 3 } else { 4 }));
    let protection = api_try!(emu, call.arg_u32(emu, if extended { 4 } else { 5 }));
    let count = if extended { 7 } else { 6 };
    let result = (|| {
        require_memory_access(emu, process, PROCESS_VM_OPERATION)?;
        if !writable_pointer(emu, base_ptr) || !writable_pointer(emu, size_ptr) {
            return Err(STATUS_ACCESS_VIOLATION);
        }
        if extra_count != 0 || zero_bits != 0 {
            return Err(STATUS_NOT_IMPLEMENTED);
        }
        if permissions(protection).is_none() {
            return Err(STATUS_INVALID_PAGE_PROTECTION);
        }
        let address = read_pointer(emu, base_ptr).ok_or(STATUS_ACCESS_VIOLATION)?;
        let size = read_pointer(emu, size_ptr).ok_or(STATUS_ACCESS_VIOLATION)?;
        let (base, length) = allocate(emu, address, size, flags, protection).map_err(status)?;
        write_pointer(emu, base_ptr, base);
        write_pointer(emu, size_ptr, length);
        Ok(())
    })();
    call.set_return(emu, result.err().unwrap_or(0));
    call.cleanup_args(emu, count);
}

pub(crate) fn nt_free(emu: &mut Emu, call: WindowsAbi) {
    let process = api_try!(emu, call.arg(emu, 0));
    let base_ptr = api_try!(emu, call.arg(emu, 1));
    let size_ptr = api_try!(emu, call.arg(emu, 2));
    let flags = api_try!(emu, call.arg_u32(emu, 3));
    let result = (|| {
        require_memory_access(emu, process, PROCESS_VM_OPERATION)?;
        if !writable_pointer(emu, base_ptr) || !writable_pointer(emu, size_ptr) {
            return Err(STATUS_ACCESS_VIOLATION);
        }
        let address = read_pointer(emu, base_ptr).ok_or(STATUS_ACCESS_VIOLATION)?;
        let size = read_pointer(emu, size_ptr).ok_or(STATUS_ACCESS_VIOLATION)?;
        let (base, length) = free(emu, address, size, flags).map_err(status)?;
        write_pointer(emu, base_ptr, base);
        write_pointer(emu, size_ptr, length);
        Ok(())
    })();
    call.set_return(emu, result.err().unwrap_or(0));
    call.cleanup_args(emu, 4);
}

pub(crate) fn query_size(emu: &Emu) -> u64 {
    if pointer_size(emu) == 8 { 48 } else { 28 }
}

pub(crate) fn query(emu: &mut Emu, address: u64, output: u64, length: u64) -> Result<u64, u64> {
    let size = query_size(emu);
    if length < size {
        return Err(STATUS_INFO_LENGTH_MISMATCH);
    }
    if address >= address_limit(emu) {
        return Err(STATUS_INVALID_PARAMETER);
    }
    if !emu.maps.validate_write_range(output, size) {
        return Err(STATUS_ACCESS_VIOLATION);
    }
    let info = emu.maps.query_memory(address);
    let (state, protection) = info.state.map_or((MEM_FREE, 0), |s| {
        if s.committed {
            (MEM_COMMIT, s.protection)
        } else {
            (MEM_RESERVE, 0)
        }
    });
    let kind = if info.state.is_none() {
        0
    } else {
        match info.kind {
            MemoryKind::Private => MEM_PRIVATE,
            MemoryKind::Mapped => MEM_MAPPED,
            MemoryKind::Image => MEM_IMAGE,
        }
    };
    let mut bytes = vec![0u8; size as usize];
    if pointer_size(emu) == 8 {
        bytes[0..8].copy_from_slice(&info.base.to_le_bytes());
        bytes[8..16].copy_from_slice(&info.allocation_base.to_le_bytes());
        bytes[16..20].copy_from_slice(&info.allocation_protection.to_le_bytes());
        bytes[24..32].copy_from_slice(&info.size.to_le_bytes());
        bytes[32..36].copy_from_slice(&state.to_le_bytes());
        bytes[36..40].copy_from_slice(&protection.to_le_bytes());
        bytes[40..44].copy_from_slice(&kind.to_le_bytes());
    } else {
        for (offset, value) in [
            info.base as u32,
            info.allocation_base as u32,
            info.allocation_protection,
            info.size as u32,
            state,
            protection,
            kind,
        ]
        .into_iter()
        .enumerate()
        {
            bytes[offset * 4..offset * 4 + 4].copy_from_slice(&value.to_le_bytes());
        }
    }
    emu.maps.write_bytes(output, &bytes);
    Ok(size)
}

pub(crate) fn virtual_query(emu: &mut Emu, remote: bool) {
    let call = WinApi(Stdcall);
    let offset = remote as usize;
    if remote
        && let Err(status) = require_memory_access(
            emu,
            api_try!(emu, call.arg(emu, 0)),
            PROCESS_QUERY_INFORMATION,
        )
    {
        emu.set_last_error(if status == STATUS_ACCESS_DENIED {
            ERROR_ACCESS_DENIED
        } else {
            ERROR_INVALID_HANDLE
        });
        call.set_return(emu, 0);
        call.cleanup_args(emu, 4);
        return;
    }
    let result = query(
        emu,
        api_try!(emu, call.arg(emu, offset)),
        api_try!(emu, call.arg(emu, offset + 1)),
        api_try!(emu, call.arg(emu, offset + 2)),
    );
    let value = match result {
        Ok(size) => size,
        Err(_) => {
            emu.set_last_error(ERROR_INVALID_PARAMETER);
            0
        }
    };
    call.set_return(emu, value);
    call.cleanup_args(emu, 3 + offset);
}

pub(crate) fn nt_query(emu: &mut Emu, call: WindowsAbi) {
    let process = api_try!(emu, call.arg(emu, 0));
    let address = api_try!(emu, call.arg(emu, 1));
    let information_class = api_try!(emu, call.arg(emu, 2));
    let output = api_try!(emu, call.arg(emu, 3));
    let length = api_try!(emu, call.arg(emu, 4));
    let returned = api_try!(emu, call.arg(emu, 5));
    let result = (|| {
        require_memory_access(emu, process, PROCESS_QUERY_INFORMATION)?;
        if information_class != 0 {
            return Err(STATUS_NOT_IMPLEMENTED);
        }
        if returned != 0 && !writable_pointer(emu, returned) {
            return Err(STATUS_ACCESS_VIOLATION);
        }
        let size = query(emu, address, output, length)?;
        if returned != 0 {
            write_pointer(emu, returned, size);
        }
        Ok(())
    })();
    call.set_return(emu, result.err().unwrap_or(0));
    call.cleanup_args(emu, 6);
}

fn transfer_memory(
    emu: &mut Emu,
    write: bool,
    [process, remote, buffer, size, count]: [u64; 5],
) -> Result<(), u64> {
    require_memory_access(
        emu,
        process,
        if write {
            PROCESS_VM_WRITE
        } else {
            PROCESS_VM_READ
        },
    )?;
    if count != 0 && !writable_pointer(emu, count) {
        return Err(STATUS_ACCESS_VIOLATION);
    }
    let (to, from) = if write {
        (remote, buffer)
    } else {
        (buffer, remote)
    };
    let size = usize::try_from(size).map_err(|_| STATUS_INVALID_PARAMETER)?;
    let copied = emu.maps.copy_range(to, from, size);
    if count != 0 {
        write_pointer(emu, count, if copied { size as u64 } else { 0 });
    }
    if copied {
        Ok(())
    } else {
        Err(STATUS_PARTIAL_COPY)
    }
}

pub(crate) fn process_memory(emu: &mut Emu, write: bool) {
    let call = WinApi(Stdcall);
    let arguments = api_try!(emu, call.args(emu));
    let result = if write {
        require_memory_access(emu, arguments[0], PROCESS_VM_OPERATION)
    } else {
        Ok(())
    }
    .and_then(|()| transfer_memory(emu, write, arguments));
    if let Err(status) = result {
        let error = match status {
            STATUS_ACCESS_DENIED => ERROR_ACCESS_DENIED,
            STATUS_INVALID_HANDLE => ERROR_INVALID_HANDLE,
            STATUS_ACCESS_VIOLATION => ERROR_NOACCESS,
            STATUS_INVALID_PARAMETER => ERROR_INVALID_PARAMETER,
            _ => ERROR_PARTIAL_COPY,
        };
        emu.set_last_error(error);
    }
    call.set_return(emu, result.is_ok() as u64);
    call.cleanup_args(emu, 5);
}

pub(crate) fn nt_process_memory(emu: &mut Emu, call: WindowsAbi, write: bool) {
    let arguments = api_try!(emu, call.args(emu));
    let result = transfer_memory(emu, write, arguments);
    call.set_return(emu, result.err().unwrap_or(0));
    call.cleanup_args(emu, 5);
}

pub(crate) fn virtual_protect(emu: &mut Emu, remote: bool) {
    let call = WinApi(Stdcall);
    let offset = remote as usize;
    if remote
        && let Err(status) =
            require_memory_access(emu, api_try!(emu, call.arg(emu, 0)), PROCESS_VM_OPERATION)
    {
        emu.set_last_error(if status == STATUS_ACCESS_DENIED {
            ERROR_ACCESS_DENIED
        } else {
            ERROR_INVALID_HANDLE
        });
        call.set_return(emu, 0);
        call.cleanup_args(emu, 5);
        return;
    }
    let address = api_try!(emu, call.arg(emu, offset));
    let size = api_try!(emu, call.arg(emu, offset + 1));
    let protection = api_try!(emu, call.arg_u32(emu, offset + 2));
    let old = api_try!(emu, call.arg(emu, offset + 3));
    let ok = virtual_protect::protect_winapi(emu, address, size, protection, old);
    call.set_return(emu, ok as u64);
    call.cleanup_args(emu, 4 + offset);
}

pub(crate) fn nt_protect(emu: &mut Emu, call: WindowsAbi) {
    let process = api_try!(emu, call.arg(emu, 0));
    let base_ptr = api_try!(emu, call.arg(emu, 1));
    let size_ptr = api_try!(emu, call.arg(emu, 2));
    let protection = api_try!(emu, call.arg_u32(emu, 3));
    let old = api_try!(emu, call.arg(emu, 4));
    let result = (|| {
        require_memory_access(emu, process, PROCESS_VM_OPERATION)?;
        if !writable_pointer(emu, base_ptr)
            || !writable_pointer(emu, size_ptr)
            || !emu.maps.validate_write_range(old, 4)
        {
            return Err(STATUS_ACCESS_VIOLATION);
        }
        let address = read_pointer(emu, base_ptr).ok_or(STATUS_ACCESS_VIOLATION)?;
        let size = read_pointer(emu, size_ptr).ok_or(STATUS_ACCESS_VIOLATION)?;
        let previous = virtual_protect::validate(&emu.maps, address, size, protection)
            .map_err(|e| e.nt_status())?;
        let (base, length) = page_range(address, size).map_err(status)?;
        write_pointer(emu, base_ptr, base);
        write_pointer(emu, size_ptr, length);
        emu.maps.write_dword(old, previous);
        virtual_protect::protect(&mut emu.maps, base, length, protection)
            .expect("validated protection range");
        Ok(())
    })();
    call.set_return(emu, result.err().unwrap_or(0));
    call.cleanup_args(emu, 5);
}

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    let (handler, parameters, returns_value): (fn(&mut Emu), &str, bool) = match api {
        "VirtualAlloc" | "VirtualAllocFromApp" => (
            |emu| virtual_alloc(emu, false, false),
            "address size flags protection",
            true,
        ),
        "VirtualAllocEx" => (
            |emu| virtual_alloc(emu, true, false),
            "process address size flags protection",
            true,
        ),
        "VirtualAllocExNuma" => (
            |emu| virtual_alloc(emu, true, true),
            "process address size flags protection node",
            true,
        ),
        "VirtualFree" => (|emu| virtual_free(emu, false), "address size flags", true),
        "VirtualFreeEx" => (
            |emu| virtual_free(emu, true),
            "process address size flags",
            true,
        ),
        "VirtualProtect" | "VirtualProtectFromApp" => (
            |emu| virtual_protect(emu, false),
            "address size protection old_protection",
            true,
        ),
        "VirtualProtectEx" => (
            |emu| virtual_protect(emu, true),
            "process address size protection old_protection",
            true,
        ),
        "VirtualQuery" => (
            |emu| virtual_query(emu, false),
            "address output length",
            true,
        ),
        "VirtualQueryEx" => (
            |emu| virtual_query(emu, true),
            "process address output length",
            true,
        ),
        "ReadProcessMemory" => (
            |emu| process_memory(emu, false),
            "process address buffer size bytes_read",
            true,
        ),
        "WriteProcessMemory" => (
            |emu| process_memory(emu, true),
            "process address buffer size bytes_written",
            true,
        ),
        _ => return false,
    };
    trace::api(emu, api, parameters, returns_value, handler);
    true
}

pub(crate) fn dispatch_nt(api: &str, emu: &mut Emu, call: WindowsAbi) -> bool {
    let normalized = nt_api::normalize(api);
    let (handler, parameters): (fn(&mut Emu, WindowsAbi), &str) = match normalized.as_ref() {
        "NtAllocateVirtualMemory" => (
            |emu, call| nt_allocate(emu, call, false),
            "process base_ptr zero_bits size_ptr flags protection",
        ),
        "NtAllocateVirtualMemoryEx" => (
            |emu, call| nt_allocate(emu, call, true),
            "process base_ptr size_ptr flags protection parameters parameter_count",
        ),
        "NtFreeVirtualMemory" => (nt_free, "process base_ptr size_ptr flags"),
        "NtProtectVirtualMemory" => (
            nt_protect,
            "process base_ptr size_ptr protection old_protection",
        ),
        "NtQueryVirtualMemory" => (
            nt_query,
            "process address class output length returned_length",
        ),
        "NtReadVirtualMemory" => (
            |emu, call| nt_process_memory(emu, call, false),
            "process address buffer size bytes_read",
        ),
        "NtWriteVirtualMemory" => (
            |emu, call| nt_process_memory(emu, call, true),
            "process address buffer size bytes_written",
        ),
        "NtCreateSection" => (
            sections::nt_create,
            "handle_ptr access attributes maximum_size protection flags file",
        ),
        "NtMapViewOfSection" => (
            sections::nt_map,
            "section process base_ptr zero_bits commit_size offset_ptr view_size_ptr inherit flags protection",
        ),
        "NtUnmapViewOfSection" => (sections::nt_unmap, "process address"),
        _ => return false,
    };
    trace::invoke(emu, call, api, parameters, true, |emu| handler(emu, call));
    true
}

pub(crate) fn flush_instruction_cache(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let process = api_try!(emu, call.arg(emu, 0));
    let base = api_try!(emu, call.arg(emu, 1));
    let size = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "{} process=0x{:x} base=0x{:x} size=0x{:x}",
        api,
        process,
        base,
        size
    );
    if let Err(error) = flush_cache(emu, process, base, size) {
        emu.set_last_error(if error == STATUS_ACCESS_DENIED {
            ERROR_ACCESS_DENIED
        } else {
            ERROR_INVALID_HANDLE
        });
        call.set_return(emu, 0);
        call.cleanup_args(emu, 3);
        return;
    }
    call.set_return(emu, 1);
    call.cleanup_args(emu, 3);
}

fn flush_cache(emu: &mut Emu, handle: u64, base: u64, size: u64) -> Result<(), u64> {
    require_memory_access(emu, handle, 0)?;
    if base == 0 {
        emu.instruction_cache().invalidate_all();
    } else {
        emu.instruction_cache()
            .invalidate_range(base, size, BLOCK_LEN as u64);
    }
    Ok(())
}

pub(crate) fn nt_flush_cache(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [handle, base, size] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "NtFlushInstructionCache process=0x{:x} base=0x{:x} size={}",
        handle,
        base,
        size
    );
    let status = flush_cache(emu, handle, base, size)
        .err()
        .unwrap_or(STATUS_SUCCESS);
    call.set_return(emu, status);
    call.cleanup_args(emu, 3);
}

const GUEST_PHYSICAL_BYTES: u64 = 4 << 30;

pub(crate) fn memory_status(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 0));
    let result = if api == "GetPhysicallyInstalledSystemMemory" {
        out != 0
            && emu.maps.validate_write_range(out, 8)
            && emu.maps.write_qword(out, GUEST_PHYSICAL_BYTES / 1024)
    } else if out != 0
        && emu.maps.read_dword(out) == Some(64)
        && emu.maps.validate_write_range(out, 64)
    {
        let space = if emu.cfg.arch.is_64bits() {
            1u64 << 47
        } else {
            1u64 << 31
        };
        let mut bytes = Vec::with_capacity(64);
        bytes.extend(64u32.to_le_bytes());
        bytes.extend(50u32.to_le_bytes());
        for value in [
            GUEST_PHYSICAL_BYTES,
            GUEST_PHYSICAL_BYTES / 2,
            2 * GUEST_PHYSICAL_BYTES,
            GUEST_PHYSICAL_BYTES,
            space,
            space / 2,
            0,
        ] {
            bytes.extend(value.to_le_bytes());
        }
        emu.maps.write_bytes(out, &bytes)
    } else {
        false
    };
    log_red!(emu, "{api} output={out:#x}");
    if !result {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
    }
    call.set_return(emu, u64::from(result));
    call.cleanup_args(emu, 1);
}
