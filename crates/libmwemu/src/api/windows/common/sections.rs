use crate::emu::Emu;
use crate::emu::object_handle::SectionHandle;
use crate::maps::virtual_memory::{
    ALLOCATION_GRANULARITY, Allocation, MemoryKind, PAGE_SIZE, PageState, align_up,
};
use crate::winapi::common::memory::{self, Call};
use crate::winapi::common::trace;
use crate::windows::constants;

const STATUS_OBJECT_NAME_NOT_FOUND: u64 = 0xc0000034;
const STATUS_INVALID_HANDLE: u64 = 0xc0000008;
const STATUS_INVALID_PARAMETER: u64 = 0xc000000d;
const STATUS_ACCESS_VIOLATION: u64 = 0xc0000005;
const STATUS_NOT_IMPLEMENTED: u64 = 0xc0000002;
const STATUS_NO_MEMORY: u64 = 0xc0000017;
const STATUS_CONFLICTING_ADDRESSES: u64 = 0xc0000018;
const STATUS_SECTION_PROTECTION: u64 = 0xc000004e;
const STATUS_NOT_MAPPED_VIEW: u64 = 0xc0000019;
const SEC_COMMIT: u32 = 0x08000000;
const FILE_MAP_COPY: u32 = 1;
const FILE_MAP_WRITE: u32 = 2;
const FILE_MAP_READ: u32 = 4;
const FILE_MAP_EXECUTE: u32 = 0x20;
const FILE_MAP_ALL_ACCESS: u32 = 0xf001f;

fn error_to_win32(status: u64) -> u64 {
    match status {
        STATUS_INVALID_HANDLE => 6,
        STATUS_OBJECT_NAME_NOT_FOUND => 2,
        STATUS_NO_MEMORY => 8,
        STATUS_NOT_IMPLEMENTED => 50,
        STATUS_CONFLICTING_ADDRESSES => 487,
        STATUS_ACCESS_VIOLATION => 998,
        _ => 87,
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
            (constants::ALLOC64_MIN, constants::ALLOC64_MAX)
        } else {
            (constants::ALLOC32_MIN, constants::ALLOC32_MAX)
        };
        emu.maps
            .find_free_range(size, ALLOCATION_GRANULARITY, bottom, top)
            .ok_or(STATUS_NO_MEMORY)?
    } else {
        address
    };
    let limit = if emu.cfg.arch.is_64bits() {
        constants::USER64_ADDRESS_LIMIT
    } else {
        constants::USER32_ADDRESS_LIMIT
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
    let call = Call::Api;
    let file = call.arg(emu, 0);
    let protection = call.arg(emu, 2) as u32;
    let size = ((call.arg(emu, 3) as u32 as u64) << 32) | (call.arg(emu, 4) as u32 as u64);
    let ptr = call.arg(emu, 5);
    let name = if ptr == 0 {
        String::new()
    } else if wide {
        emu.maps.read_wide_string(ptr)
    } else {
        emu.maps.read_string(ptr)
    };
    let file = if file == u64::MAX || file == u32::MAX as u64 {
        None
    } else {
        Some(file)
    };
    let value = match create(emu, file, size, protection, name) {
        Ok((handle, exists)) => {
            emu.set_last_error(if exists { 183 } else { 0 });
            handle
        }
        Err(status) => {
            emu.set_last_error(error_to_win32(status));
            0
        }
    };
    call.finish(emu, value, 6);
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
    let call = Call::Api;
    let access = call.arg(emu, 0) as u32;
    let name_ptr = call.arg(emu, 2);
    let name = if wide {
        emu.maps.read_wide_string(name_ptr)
    } else {
        emu.maps.read_string(name_ptr)
    };
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
    call.finish(emu, value, 3);
}

pub(crate) fn map_view(emu: &mut Emu, extended: bool) {
    let call = Call::Api;
    let handle = call.arg(emu, 0);
    let access = call.arg(emu, 1) as u32;
    let offset = ((call.arg(emu, 2) as u32 as u64) << 32) | (call.arg(emu, 3) as u32 as u64);
    let size = call.arg(emu, 4);
    let address = if extended { call.arg(emu, 5) } else { 0 };
    let result = view_protection(access)
        .and_then(|protection| map(emu, handle, address, offset, size, protection));
    let value = match result {
        Ok((base, _)) => base,
        Err(status) => {
            emu.set_last_error(error_to_win32(status));
            0
        }
    };
    call.finish(emu, value, if extended { 6 } else { 5 });
}

pub(crate) fn unmap_view(emu: &mut Emu, extended: bool) {
    let call = Call::Api;
    let address = call.arg(emu, 0);
    let result = if extended && call.arg(emu, 1) != 0 {
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
    call.finish(emu, value, if extended { 2 } else { 1 });
}

pub(crate) fn nt_create(emu: &mut Emu, call: Call) {
    let output = call.arg(emu, 0);
    let maximum = call.arg(emu, 3);
    let protection = call.arg(emu, 4) as u32;
    let attributes = call.arg(emu, 5) as u32;
    let file = call.arg(emu, 6);
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
    call.finish(emu, result.err().unwrap_or(0), 7);
}

pub(crate) fn nt_map(emu: &mut Emu, call: Call) {
    let handle = call.arg(emu, 0);
    let base_ptr = call.arg(emu, 2);
    let offset_ptr = call.arg(emu, 5);
    let size_ptr = call.arg(emu, 6);
    let protection = call.arg(emu, 9) as u32;
    let result = (|| {
        if !memory::current_process(call.arg(emu, 1)) {
            return Err(STATUS_INVALID_HANDLE);
        }
        if call.arg(emu, 3) != 0 || call.arg(emu, 4) != 0 || call.arg(emu, 8) != 0 {
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
    call.finish(emu, result.err().unwrap_or(0), 10);
}

pub(crate) fn nt_unmap(emu: &mut Emu, call: Call) {
    let result = if memory::current_process(call.arg(emu, 0)) {
        unmap(emu, call.arg(emu, 1))
    } else {
        Err(STATUS_INVALID_HANDLE)
    };
    call.finish(emu, result.err().unwrap_or(0), 2);
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
