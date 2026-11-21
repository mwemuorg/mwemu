use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::emu::object_handle::SectionHandle;
use crate::maps::virtual_memory::{
    ALLOCATION_GRANULARITY, Allocation, MemoryKind, PAGE_SIZE, PageState, align_up,
};
use crate::winapi::common::memory;
use crate::winapi::common::process::require_memory_access;
use crate::winapi::common::strings;
use crate::winapi::common::trace;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::files::*;
use crate::windows::constants::memory::*;
use crate::windows::constants::process::{
    INVALID_HANDLE_VALUE_32, INVALID_HANDLE_VALUE_64, PROCESS_VM_OPERATION,
};

fn error_to_win32(status: u64) -> u64 {
    match status {
        STATUS_INVALID_HANDLE => ERROR_INVALID_HANDLE,
        STATUS_ACCESS_DENIED => ERROR_ACCESS_DENIED,
        STATUS_OBJECT_NAME_NOT_FOUND => ERROR_FILE_NOT_FOUND,
        STATUS_NO_MEMORY => ERROR_NOT_ENOUGH_MEMORY,
        STATUS_NOT_IMPLEMENTED => ERROR_NOT_SUPPORTED,
        STATUS_CONFLICTING_ADDRESSES => ERROR_INVALID_ADDRESS,
        STATUS_ACCESS_VIOLATION => ERROR_NOACCESS,
        _ => ERROR_INVALID_PARAMETER,
    }
}

pub(crate) fn create(
    emu: &mut Emu,
    file: Option<u64>,
    maximum: u64,
    protection: u32,
    name: String,
) -> Result<(u64, bool), u64> {
    let rights = memory::permissions(protection & 0xff).ok_or(STATUS_INVALID_PARAMETER)?;
    if protection & !(SEC_COMMIT | 0xff) != 0 || matches!(protection & 0xff, 8 | 0x80) {
        return Err(STATUS_NOT_IMPLEMENTED);
    }
    if !rights.can_read() {
        return Err(STATUS_INVALID_PARAMETER);
    }
    if let Some(section) = emu.handle_management.named_section(&name).cloned() {
        return Ok((
            emu.handle_management.insert_section_handle(section) as u64,
            true,
        ));
    }
    let length = if let Some(handle) = file {
        let key = u32::try_from(handle).map_err(|_| STATUS_INVALID_HANDLE)?;
        let file = emu
            .handle_management
            .get_mut_file_handle(key)
            .ok_or(STATUS_INVALID_HANDLE)?;
        if maximum == 0 {
            file.file_size
        } else {
            maximum
        }
    } else {
        maximum
    };
    if length == 0 {
        return Err(STATUS_INVALID_PARAMETER);
    }
    let size = align_up(length, PAGE_SIZE)
        .filter(|&n| n <= emu.maps.max_alloc_size)
        .ok_or(STATUS_NO_MEMORY)?;
    let bytes = if let Some(handle) = file {
        emu.handle_management
            .get_mut_file_handle(handle as u32)
            .ok_or(STATUS_INVALID_HANDLE)?
            .mapping_bytes(size as usize)
            .map_err(|_| STATUS_ACCESS_VIOLATION)?
    } else {
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(size as usize)
            .map_err(|_| STATUS_NO_MEMORY)?;
        bytes.resize(size as usize, 0);
        bytes
    };
    let storage = emu.maps.create_shared_storage(bytes);
    let section = SectionHandle {
        name,
        max_size: length,
        protection: protection & 0xff,
        storage,
        access: None,
    };
    Ok((
        emu.handle_management.insert_section_handle(section) as u64,
        false,
    ))
}

pub(crate) fn map(
    emu: &mut Emu,
    handle: u64,
    address: u64,
    offset: u64,
    length: u64,
    protection: u32,
) -> Result<(u64, u64), u64> {
    let section = emu
        .handle_management
        .section_handle(handle)
        .cloned()
        .ok_or(STATUS_INVALID_HANDLE)?;
    let requested = memory::permissions(protection).ok_or(STATUS_INVALID_PARAMETER)?;
    let allowed = memory::permissions(section.protection).ok_or(STATUS_INVALID_PARAMETER)?;
    if matches!(protection & 0xff, 8 | 0x80) {
        return Err(STATUS_NOT_IMPLEMENTED);
    }
    if !allowed.contains(requested)
        || section
            .access
            .is_some_and(|access| !access.contains(requested))
    {
        return Err(STATUS_SECTION_PROTECTION);
    }
    if !offset.is_multiple_of(ALLOCATION_GRANULARITY)
        || !address.is_multiple_of(ALLOCATION_GRANULARITY)
        || offset >= section.max_size
    {
        return Err(STATUS_INVALID_PARAMETER);
    }
    let length = if length == 0 {
        section.max_size - offset
    } else {
        length
    };
    if offset
        .checked_add(length)
        .is_none_or(|end| end > section.max_size)
    {
        return Err(STATUS_INVALID_PARAMETER);
    }
    let size = align_up(length, PAGE_SIZE).ok_or(STATUS_INVALID_PARAMETER)?;
    let base = if address == 0 {
        let (bottom, top) = if emu.cfg.arch.is_64bits() {
            (ALLOC64_MIN, ALLOC64_MAX)
        } else {
            (ALLOC32_MIN, ALLOC32_MAX)
        };
        emu.maps
            .find_free_range(size, ALLOCATION_GRANULARITY, bottom, top)
            .ok_or(STATUS_NO_MEMORY)?
    } else {
        address
    };
    let limit = if emu.cfg.arch.is_64bits() {
        USER64_ADDRESS_LIMIT
    } else {
        USER32_ADDRESS_LIMIT
    };
    if base.checked_add(size).is_none_or(|end| end > limit) {
        return Err(STATUS_INVALID_PARAMETER);
    }
    let name = emu.maps.unique_map_name(&format!("section_view_{base:x}"));
    emu.maps
        .create_shared_view(&name, base, size, section.storage, offset, requested)
        .map_err(|_| STATUS_CONFLICTING_ADDRESSES)?;
    let mut allocation = Allocation::new(
        base,
        size,
        section.protection,
        MemoryKind::Mapped,
        PageState::committed(protection, requested),
    );
    allocation.maximum_permission = Some(requested);
    emu.maps.register_allocation(allocation);
    Ok((base, size))
}

pub(crate) fn collect_unused(emu: &mut Emu) {
    let unused: Vec<_> = emu
        .maps
        .shared_stores
        .keys()
        .copied()
        .filter(|&storage| {
            !emu.handle_management.section_uses_storage(storage)
                && !emu
                    .maps
                    .mem_slab
                    .iter()
                    .any(|(_, m)| m.shared_view().is_some_and(|v| v.storage == storage))
        })
        .collect();
    for id in unused {
        emu.maps.shared_stores.remove(&id);
        emu.handle_management.forget_section(id);
    }
}

pub(crate) fn unmap(emu: &mut Emu, address: u64) -> Result<(), u64> {
    let allocation = emu
        .maps
        .allocation_at(address)
        .filter(|a| matches!(a.kind, MemoryKind::Mapped | MemoryKind::Image))
        .cloned()
        .ok_or(STATUS_NOT_MAPPED_VIEW)?;
    emu.maps.unmap_range(allocation.base, allocation.size);
    if allocation.kind == MemoryKind::Image
        && let Some(index) = emu.export_indexes.get_by_base(allocation.base)
    {
        let name = index.normalized_name.clone();
        emu.export_indexes.remove(&name);
    }
    collect_unused(emu);
    Ok(())
}

pub(crate) fn close(emu: &mut Emu, handle: u64) -> bool {
    if emu.handle_management.section_handle(handle).is_none() {
        return false;
    }
    let success = emu.handle_management.close_handle(handle);
    collect_unused(emu);
    success
}

pub(crate) fn create_file_mapping(emu: &mut Emu, wide: bool) {
    let call = WinApi(Stdcall);
    let file = api_try!(emu, call.arg(emu, 0));
    let protection = api_try!(emu, call.arg_u32(emu, 2));
    let size = ((api_try!(emu, call.arg_u32(emu, 3)) as u64) << 32)
        | (api_try!(emu, call.arg_u32(emu, 4)) as u64);
    let ptr = api_try!(emu, call.arg(emu, 5));
    finish_create_mapping(emu, file, size, protection, ptr, wide, 6);
}

fn create_mapping_from_app(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let file = api_try!(emu, call.arg(emu, 0));
    let protect = api_try!(emu, call.arg_u32(emu, 2));
    let size = api_try!(emu, call.arg_u64(emu, 3));
    let name_index = if emu.cfg.arch.is_64bits() { 4 } else { 5 };
    let ptr = api_try!(emu, call.arg(emu, name_index));
    finish_create_mapping(emu, file, size, protect, ptr, true, name_index + 1);
}

fn finish_create_mapping(
    emu: &mut Emu,
    file: u64,
    size: u64,
    protection: u32,
    ptr: u64,
    wide: bool,
    slots: usize,
) {
    let call = WinApi(Stdcall);
    let name = if ptr == 0 {
        String::new()
    } else {
        strings::read_text(emu, ptr, wide)
    };
    let page_file = if emu.cfg.arch.is_64bits() {
        INVALID_HANDLE_VALUE_64
    } else {
        INVALID_HANDLE_VALUE_32
    };
    let file = (file != page_file).then_some(file);
    let value = match create(emu, file, size, protection, name) {
        Ok((handle, exists)) => {
            emu.set_last_error(if exists {
                ERROR_ALREADY_EXISTS
            } else {
                ERROR_SUCCESS
            });
            handle
        }
        Err(status) => {
            emu.set_last_error(error_to_win32(status));
            0
        }
    };
    call.set_return(emu, value);
    call.cleanup_args(emu, slots);
}

fn view_protection(access: u32) -> Result<u32, u64> {
    if access & FILE_MAP_COPY != 0 && access & FILE_MAP_ALL_ACCESS != FILE_MAP_ALL_ACCESS {
        return Err(STATUS_NOT_IMPLEMENTED);
    }
    if access & (FILE_MAP_READ | FILE_MAP_WRITE) == 0 {
        return Err(STATUS_INVALID_PARAMETER);
    }
    match (access & FILE_MAP_WRITE != 0, access & FILE_MAP_EXECUTE != 0) {
        (true, true) => Ok(0x40),
        (false, true) => Ok(0x20),
        (true, false) => Ok(4),
        (false, false) => Ok(2),
    }
}

pub(crate) fn open_file_mapping(emu: &mut Emu, wide: bool) {
    let call = WinApi(Stdcall);
    let access = api_try!(emu, call.arg_u32(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 2));
    let name = strings::read_text(emu, name_ptr, wide);
    let result = (|| {
        let protection = view_protection(access)?;
        let mut section = emu
            .handle_management
            .named_section(&name)
            .cloned()
            .ok_or(STATUS_OBJECT_NAME_NOT_FOUND)?;
        let allowed = memory::permissions(section.protection).ok_or(STATUS_INVALID_PARAMETER)?;
        let requested = memory::permissions(protection).ok_or(STATUS_INVALID_PARAMETER)?;
        if !allowed.contains(requested) {
            return Err(STATUS_SECTION_PROTECTION);
        }
        section.access = Some(requested);
        Ok(emu.handle_management.insert_section_handle(section) as u64)
    })();
    let value = match result {
        Ok(handle) => handle,
        Err(status) => {
            emu.set_last_error(error_to_win32(status));
            0
        }
    };
    call.set_return(emu, value);
    call.cleanup_args(emu, 3);
}

pub(crate) fn map_view(emu: &mut Emu, extended: bool) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let access = api_try!(emu, call.arg_u32(emu, 1));
    let offset = ((api_try!(emu, call.arg_u32(emu, 2)) as u64) << 32)
        | (api_try!(emu, call.arg_u32(emu, 3)) as u64);
    let size = api_try!(emu, call.arg(emu, 4));
    let address = if extended {
        api_try!(emu, call.arg(emu, 5))
    } else {
        0
    };
    finish_map_view(
        emu,
        handle,
        access,
        offset,
        size,
        address,
        if extended { 6 } else { 5 },
    );
}

fn map_view_from_app(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let access = api_try!(emu, call.arg_u32(emu, 1));
    let offset = api_try!(emu, call.arg_u64(emu, 2));
    let size_index = if emu.cfg.arch.is_64bits() { 3 } else { 4 };
    let size = api_try!(emu, call.arg(emu, size_index));
    finish_map_view(emu, handle, access, offset, size, 0, size_index + 1);
}

fn finish_map_view(
    emu: &mut Emu,
    handle: u64,
    access: u32,
    offset: u64,
    size: u64,
    address: u64,
    slots: usize,
) {
    let call = WinApi(Stdcall);
    let result = view_protection(access)
        .and_then(|protection| map(emu, handle, address, offset, size, protection));
    let value = match result {
        Ok((base, _)) => base,
        Err(status) => {
            emu.set_last_error(error_to_win32(status));
            0
        }
    };
    call.set_return(emu, value);
    call.cleanup_args(emu, slots);
}

pub(crate) fn unmap_view(emu: &mut Emu, extended: bool) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    let result = if extended && api_try!(emu, call.arg(emu, 1)) != 0 {
        Err(STATUS_NOT_IMPLEMENTED)
    } else {
        unmap(emu, address)
    };
    let value = match result {
        Ok(()) => 1,
        Err(status) => {
            emu.set_last_error(error_to_win32(status));
            0
        }
    };
    call.set_return(emu, value);
    call.cleanup_args(emu, if extended { 2 } else { 1 });
}

fn unmap_view_in_process(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let process = api_try!(emu, call.arg(emu, 0));
    let base = api_try!(emu, call.arg(emu, 1));
    let flags = api_try!(emu, call.arg_u32(emu, 2));
    let result = if let Err(error) = require_memory_access(emu, process, PROCESS_VM_OPERATION) {
        Err(error)
    } else if flags != 0 {
        Err(STATUS_NOT_IMPLEMENTED)
    } else {
        unmap(emu, base)
    };
    if let Err(status) = result {
        emu.set_last_error(error_to_win32(status));
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 3);
}

pub(crate) fn nt_create(emu: &mut Emu, call: WindowsAbi) {
    let output = api_try!(emu, call.arg(emu, 0));
    let maximum = api_try!(emu, call.arg(emu, 3));
    let protection = api_try!(emu, call.arg_u32(emu, 4));
    let attributes = api_try!(emu, call.arg_u32(emu, 5));
    let file = api_try!(emu, call.arg(emu, 6));
    let result = (|| {
        if !memory::writable_pointer(emu, output) {
            return Err(STATUS_ACCESS_VIOLATION);
        }
        let size = if maximum == 0 {
            0
        } else {
            emu.maps
                .read_qword(maximum)
                .ok_or(STATUS_ACCESS_VIOLATION)?
        };
        if size > i64::MAX as u64 {
            return Err(STATUS_INVALID_PARAMETER);
        }
        let (handle, _) = create(
            emu,
            if file == 0 { None } else { Some(file) },
            size,
            protection | attributes,
            String::new(),
        )?;
        memory::write_pointer(emu, output, handle);
        Ok(())
    })();
    call.set_return(emu, result.err().unwrap_or(0));
    call.cleanup_args(emu, 7);
}

pub(crate) fn nt_map(emu: &mut Emu, call: WindowsAbi) {
    let process = api_try!(emu, call.arg(emu, 1));
    let zero_bits = api_try!(emu, call.arg(emu, 3));
    let commit_size = api_try!(emu, call.arg(emu, 4));
    let flags = api_try!(emu, call.arg(emu, 8));
    let handle = api_try!(emu, call.arg(emu, 0));
    let base_ptr = api_try!(emu, call.arg(emu, 2));
    let offset_ptr = api_try!(emu, call.arg(emu, 5));
    let size_ptr = api_try!(emu, call.arg(emu, 6));
    let protection = api_try!(emu, call.arg_u32(emu, 9));
    let result = (|| {
        require_memory_access(emu, process, PROCESS_VM_OPERATION)?;
        if zero_bits != 0 || commit_size != 0 || flags != 0 {
            return Err(STATUS_NOT_IMPLEMENTED);
        }
        if !memory::writable_pointer(emu, base_ptr) || !memory::writable_pointer(emu, size_ptr) {
            return Err(STATUS_ACCESS_VIOLATION);
        }
        if offset_ptr != 0 && !emu.maps.validate_write_range(offset_ptr, 8) {
            return Err(STATUS_ACCESS_VIOLATION);
        }
        let base = memory::read_pointer(emu, base_ptr).ok_or(STATUS_ACCESS_VIOLATION)?;
        let size = memory::read_pointer(emu, size_ptr).ok_or(STATUS_ACCESS_VIOLATION)?;
        let offset = if offset_ptr == 0 {
            0
        } else {
            emu.maps
                .read_qword(offset_ptr)
                .ok_or(STATUS_ACCESS_VIOLATION)?
        };
        let (address, length) = map(emu, handle, base, offset, size, protection)?;
        memory::write_pointer(emu, base_ptr, address);
        memory::write_pointer(emu, size_ptr, length);
        Ok(())
    })();
    call.set_return(emu, result.err().unwrap_or(0));
    call.cleanup_args(emu, 10);
}

pub(crate) fn nt_unmap(emu: &mut Emu, call: WindowsAbi) {
    let process = api_try!(emu, call.arg(emu, 0));
    let address = api_try!(emu, call.arg(emu, 1));
    let result = require_memory_access(emu, process, PROCESS_VM_OPERATION)
        .and_then(|()| unmap(emu, address));
    call.set_return(emu, result.err().unwrap_or(0));
    call.cleanup_args(emu, 2);
}

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    let (handler, parameters, returns_value): (fn(&mut Emu), &str, bool) = match api {
        "OpenFileMappingA" => (
            |emu| open_file_mapping(emu, false),
            "access inherit name",
            true,
        ),
        "OpenFileMappingW" | "OpenFileMappingFromApp" => (
            |emu| open_file_mapping(emu, true),
            "access inherit name",
            true,
        ),
        "CreateFileMappingA" => (
            |emu| create_file_mapping(emu, false),
            "file security protection size_high size_low name",
            true,
        ),
        "CreateFileMappingW" => (
            |emu| create_file_mapping(emu, true),
            "file security protection size_high size_low name",
            true,
        ),
        "CreateFileMappingFromApp" => (
            create_mapping_from_app,
            "file security protection size name",
            true,
        ),
        "MapViewOfFileFromApp" => (map_view_from_app, "mapping access offset size", true),
        "UnmapViewOfFile2" => (unmap_view_in_process, "process address flags", true),
        "MapViewOfFile" => (
            |emu| map_view(emu, false),
            "mapping access offset_high offset_low size",
            true,
        ),
        "MapViewOfFileEx" => (
            |emu| map_view(emu, true),
            "mapping access offset_high offset_low size address",
            true,
        ),
        "UnmapViewOfFile" => (|emu| unmap_view(emu, false), "address", true),
        "UnmapViewOfFileEx" => (|emu| unmap_view(emu, true), "address flags", true),
        _ => return false,
    };
    trace::api(emu, api, parameters, returns_value, handler);
    true
}
