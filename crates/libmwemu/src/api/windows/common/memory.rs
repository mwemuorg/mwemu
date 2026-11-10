//! Shared Windows memory semantics. ABI adaptation is kept at the boundary.
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::{
    ALLOCATION_GRANULARITY, MemoryError, MemoryKind, PAGE_SIZE, align_up, page_range,
};
use crate::winapi::common::virtual_protect;
use crate::winapi::common::{nt_api, sections, trace};
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;
use crate::windows::constants::memory::*;
use crate::windows::constants::process::*;

const PAGE_MASK: u32 = 0xff;
const PAGE_MODIFIERS: u32 = 0x4000_0700;

#[derive(Clone, Copy)]
pub(crate) enum Call {
    Api,
    Syscall,
}

impl Call {
    pub(crate) fn arg(self, emu: &Emu, index: usize) -> u64 {
        if !emu.cfg.arch.is_64bits() {
            return emu
                .maps
                .read_dword(emu.regs().get_esp() + index as u64 * 4)
                .unwrap_or(0) as u64;
        }
        if matches!(self, Self::Syscall) && index >= 4 {
            emu.maps
                .read_qword(emu.regs().rsp + 0x28 + (index as u64 - 4) * 8)
                .unwrap_or(0)
        } else {
            WinApiAbi::arg(emu, index)
        }
    }

    pub(crate) fn finish(self, emu: &mut Emu, value: u64, args: usize) {
        WinApiAbi::set_ret(emu, value);
        if matches!(self, Self::Api) && !emu.cfg.arch.is_64bits() {
            for _ in 0..args {
                emu.stack_pop32(false);
            }
        }
    }
}

pub(crate) fn pointer_size(emu: &Emu) -> u64 {
    if emu.cfg.arch.is_64bits() { 8 } else { 4 }
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

pub(crate) fn current_process(handle: u64) -> bool {
    handle == NT_CURRENT_PROCESS as u32 as u64 || handle == NT_CURRENT_PROCESS
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
    if length > emu.maps.max_alloc_size {
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
    let call = Call::Api;
    let offset = remote as usize;
    let count = 4 + offset + numa as usize;
    if remote && !current_process(call.arg(emu, 0)) {
        emu.set_last_error(ERROR_INVALID_HANDLE);
        call.finish(emu, 0, count);
        return;
    }
    let args = (
        call.arg(emu, offset),
        call.arg(emu, offset + 1),
        call.arg(emu, offset + 2) as u32,
        call.arg(emu, offset + 3) as u32,
    );
    let result = allocate(emu, args.0, args.1, args.2, args.3);
    let value = match result {
        Ok((base, _)) => base,
        Err(e) => {
            emu.set_last_error(win32_error(e));
            0
        }
    };
    call.finish(emu, value, count);
}

pub(crate) fn virtual_free(emu: &mut Emu, remote: bool) {
    let call = Call::Api;
    let offset = remote as usize;
    let count = 3 + offset;
    if remote && !current_process(call.arg(emu, 0)) {
        emu.set_last_error(ERROR_INVALID_HANDLE);
        call.finish(emu, 0, count);
        return;
    }
    let result = free(
        emu,
        call.arg(emu, offset),
        call.arg(emu, offset + 1),
        call.arg(emu, offset + 2) as u32,
    );
    let value = match result {
        Ok(_) => 1,
        Err(e) => {
            emu.set_last_error(win32_error(e));
            0
        }
    };
    call.finish(emu, value, count);
}

pub(crate) fn nt_allocate(emu: &mut Emu, call: Call, extended: bool) {
    let base_ptr = call.arg(emu, 1);
    let size_ptr = call.arg(emu, if extended { 2 } else { 3 });
    let flags = call.arg(emu, if extended { 3 } else { 4 }) as u32;
    let protection = call.arg(emu, if extended { 4 } else { 5 }) as u32;
    let count = if extended { 7 } else { 6 };
    let result = (|| {
        if !current_process(call.arg(emu, 0)) {
            return Err(STATUS_INVALID_HANDLE);
        }
        if !writable_pointer(emu, base_ptr) || !writable_pointer(emu, size_ptr) {
            return Err(STATUS_ACCESS_VIOLATION);
        }
        if (!extended && call.arg(emu, 2) != 0) || (extended && call.arg(emu, 6) != 0) {
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
    call.finish(emu, result.err().unwrap_or(0), count);
}

pub(crate) fn nt_free(emu: &mut Emu, call: Call) {
    let base_ptr = call.arg(emu, 1);
    let size_ptr = call.arg(emu, 2);
    let flags = call.arg(emu, 3) as u32;
    let result = (|| {
        if !current_process(call.arg(emu, 0)) {
            return Err(STATUS_INVALID_HANDLE);
        }
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
    call.finish(emu, result.err().unwrap_or(0), 4);
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
    let call = Call::Api;
    let offset = remote as usize;
    if remote && !current_process(call.arg(emu, 0)) {
        emu.set_last_error(ERROR_INVALID_HANDLE);
        call.finish(emu, 0, 4);
        return;
    }
    let result = query(
        emu,
        call.arg(emu, offset),
        call.arg(emu, offset + 1),
        call.arg(emu, offset + 2),
    );
    let value = match result {
        Ok(size) => size,
        Err(_) => {
            emu.set_last_error(ERROR_INVALID_PARAMETER);
            0
        }
    };
    call.finish(emu, value, 3 + offset);
}

pub(crate) fn nt_query(emu: &mut Emu, call: Call) {
    let returned = call.arg(emu, 5);
    let result = (|| {
        if !current_process(call.arg(emu, 0)) {
            return Err(STATUS_INVALID_HANDLE);
        }
        if call.arg(emu, 2) != 0 {
            return Err(STATUS_NOT_IMPLEMENTED);
        }
        if returned != 0 && !writable_pointer(emu, returned) {
            return Err(STATUS_ACCESS_VIOLATION);
        }
        let size = query(emu, call.arg(emu, 1), call.arg(emu, 3), call.arg(emu, 4))?;
        if returned != 0 {
            write_pointer(emu, returned, size);
        }
        Ok(())
    })();
    call.finish(emu, result.err().unwrap_or(0), 6);
}

fn transfer_memory(emu: &mut Emu, call: Call, write: bool) -> Result<(), u64> {
    let process = call.arg(emu, 0);
    let remote = call.arg(emu, 1);
    let buffer = call.arg(emu, 2);
    let size = call.arg(emu, 3);
    let count = call.arg(emu, 4);
    if !current_process(process) {
        return Err(STATUS_INVALID_HANDLE);
    }
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
    let result = transfer_memory(emu, Call::Api, write);
    if let Err(status) = result {
        let error = match status {
            STATUS_INVALID_HANDLE => ERROR_INVALID_HANDLE,
            STATUS_ACCESS_VIOLATION => ERROR_NOACCESS,
            STATUS_INVALID_PARAMETER => ERROR_INVALID_PARAMETER,
            _ => ERROR_PARTIAL_COPY,
        };
        emu.set_last_error(error);
    }
    Call::Api.finish(emu, result.is_ok() as u64, 5);
}

pub(crate) fn nt_process_memory(emu: &mut Emu, call: Call, write: bool) {
    let result = transfer_memory(emu, call, write);
    call.finish(emu, result.err().unwrap_or(0), 5);
}

pub(crate) fn virtual_protect(emu: &mut Emu, remote: bool) {
    let call = Call::Api;
    let offset = remote as usize;
    if remote && !current_process(call.arg(emu, 0)) {
        emu.set_last_error(ERROR_INVALID_HANDLE);
        call.finish(emu, 0, 5);
        return;
    }
    let address = call.arg(emu, offset);
    let size = call.arg(emu, offset + 1);
    let protection = call.arg(emu, offset + 2) as u32;
    let old = call.arg(emu, offset + 3);
    let ok = virtual_protect::protect_winapi(emu, address, size, protection, old);
    call.finish(emu, ok as u64, 4 + offset);
}

pub(crate) fn nt_protect(emu: &mut Emu, call: Call) {
    let base_ptr = call.arg(emu, 1);
    let size_ptr = call.arg(emu, 2);
    let protection = call.arg(emu, 3) as u32;
    let old = call.arg(emu, 4);
    let result = (|| {
        if !current_process(call.arg(emu, 0)) {
            return Err(STATUS_INVALID_HANDLE);
        }
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
    call.finish(emu, result.err().unwrap_or(0), 5);
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

pub(crate) fn dispatch_nt(api: &str, emu: &mut Emu, call: Call) -> bool {
    let normalized = nt_api::normalize(api);
    let (handler, parameters): (fn(&mut Emu, Call), &str) = match normalized.as_ref() {
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
    let call = Call::Api;
    let process = call.arg(emu, 0);
    let base = call.arg(emu, 1);
    let size = call.arg(emu, 2);
    log_red!(
        emu,
        "{} process:0x{:x} base:0x{:x} size:0x{:x}",
        api,
        process,
        base,
        size
    );
    // The emulator has one address space. Flushing must not consume the process handle.
    if base == 0 {
        emu.instruction_cache().invalidate_all();
    } else {
        emu.instruction_cache()
            .invalidate_range(base, size, BLOCK_LEN as u64);
    }
    call.finish(emu, 1, 3);
}
