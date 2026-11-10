use crate::windows::constants::errors::*;
use crate::windows::constants::files::*;
use crate::windows::constants::process::*;
use std::io::{self, SeekFrom};

use crate::emu::Emu;
use crate::emu::object_handle::FileHandle;
use crate::maps::mem64::Permission;
use crate::winapi::common::memory::Call;
use crate::winapi::helper;

const SECTORS_PER_CLUSTER: u32 = 8;
const BYTES_PER_SECTOR: u32 = 512;
const FREE_CLUSTERS: u32 = 100_000;
const TOTAL_CLUSTERS: u32 = 200_000;

const STD_OUTPUT_PREVIEW_LEN: usize = 4096;

fn io_error(error: &io::Error, fallback: u64) -> u64 {
    match error.kind() {
        io::ErrorKind::NotFound => ERROR_FILE_NOT_FOUND,
        io::ErrorKind::PermissionDenied => ERROR_ACCESS_DENIED,
        io::ErrorKind::AlreadyExists => ERROR_FILE_EXISTS,
        io::ErrorKind::InvalidInput => ERROR_INVALID_PARAMETER,
        _ => fallback,
    }
}

fn file(emu: &mut Emu, handle: u64) -> Result<&mut FileHandle, u64> {
    let key = u32::try_from(handle).map_err(|_| ERROR_INVALID_HANDLE)?;
    emu.handle_management
        .get_mut_file_handle(key)
        .ok_or(ERROR_INVALID_HANDLE)
}

pub(crate) fn create_file(emu: &mut Emu, api: &str, wide: bool) {
    let call = Call::Api;
    let name_ptr = call.arg(emu, 0);
    let access = call.arg(emu, 1) as u32;
    let share = call.arg(emu, 2) as u32;
    let disposition = call.arg(emu, 4) as u32;
    let attributes = call.arg(emu, 5) as u32;
    let name = if wide {
        emu.maps.read_wide_string(name_ptr)
    } else {
        emu.maps.read_string(name_ptr)
    };
    log_red!(
        emu,
        "{} '{}' access:0x{:x} share:0x{:x} disposition:{} attributes:0x{:x}",
        api,
        name,
        access,
        share,
        disposition,
        attributes
    );
    let result = if name_ptr == 0 || name.is_empty() {
        Err(ERROR_INVALID_PARAMETER)
    } else {
        FileHandle::new(name, access, disposition, attributes, share)
            .map(|handle| emu.handle_management.insert_file_handle(handle) as u64)
            .map_err(|error| {
                error
                    .downcast_ref::<io::Error>()
                    .map_or(ERROR_INVALID_PARAMETER, |error| {
                        io_error(error, ERROR_ACCESS_DENIED)
                    })
            })
    };
    let handle = match result {
        Ok(handle) => handle,
        Err(error) => {
            emu.set_last_error(error);
            if emu.cfg.arch.is_64bits() {
                INVALID_HANDLE_VALUE_64
            } else {
                INVALID_HANDLE_VALUE_32
            }
        }
    };
    call.finish(emu, handle, 7);
}

pub(crate) fn get_file_size(emu: &mut Emu, api: &str, extended: bool) {
    let call = Call::Api;
    let handle = call.arg(emu, 0);
    let output = call.arg(emu, 1);
    let result = if (extended && output == 0)
        || (output != 0
            && !emu.maps.validate_write_range(
                output,
                if extended {
                    size_of::<u64>()
                } else {
                    size_of::<u32>()
                } as u64,
            )) {
        Err(ERROR_NOACCESS)
    } else {
        file(emu, handle).and_then(|file| file.length().map_err(|e| io_error(&e, ERROR_READ_FAULT)))
    };
    log_red!(emu, "{} handle:0x{:x} => {:?}", api, handle, result);
    let value = match result {
        Ok(size) => {
            if extended {
                emu.maps.write_qword(output, size);
            } else if output != 0 {
                emu.maps.write_dword(output, (size >> u32::BITS) as u32);
            }
            // INVALID_FILE_SIZE is also a valid low DWORD.
            emu.set_last_error(0);
            if extended { 1 } else { size as u32 as u64 }
        }
        Err(error) => {
            emu.set_last_error(error);
            if extended { 0 } else { INVALID_FILE_SIZE }
        }
    };
    call.finish(emu, value, 2);
}

fn seek(
    emu: &mut Emu,
    handle: u64,
    distance: i64,
    method: u32,
    low_only: bool,
) -> Result<u64, u64> {
    let file = file(emu, handle)?;
    let start = match method {
        FILE_BEGIN => 0,
        FILE_CURRENT => file.get_position(),
        FILE_END => file.length().map_err(|e| io_error(&e, ERROR_READ_FAULT))?,
        _ => return Err(ERROR_INVALID_PARAMETER),
    };
    let position = start
        .checked_add_signed(distance)
        .ok_or(ERROR_NEGATIVE_SEEK)?;
    if low_only && position > u32::MAX as u64 {
        return Err(ERROR_INVALID_PARAMETER);
    }
    file.seek(SeekFrom::Start(position))
        .map_err(|e| io_error(&e, ERROR_NEGATIVE_SEEK))
}

pub(crate) fn set_file_pointer(emu: &mut Emu, api: &str, extended: bool) {
    let call = Call::Api;
    let handle = call.arg(emu, 0);
    let split_distance = extended && !emu.cfg.arch.is_64bits();
    let output = call.arg(emu, if split_distance { 3 } else { 2 });
    let method = call.arg(emu, if split_distance { 4 } else { 3 }) as u32;
    let distance = if split_distance {
        Some(((call.arg(emu, 2) << u32::BITS) | call.arg(emu, 1)) as i64)
    } else if extended {
        Some(call.arg(emu, 1) as i64)
    } else if output != 0 {
        emu.maps
            .read_dword(output)
            .map(|high| (((high as u64) << u32::BITS) | call.arg(emu, 1) as u32 as u64) as i64)
    } else {
        Some(call.arg(emu, 1) as i32 as i64)
    };
    let result = if output != 0
        && !emu.maps.validate_write_range(
            output,
            if extended {
                size_of::<u64>()
            } else {
                size_of::<u32>()
            } as u64,
        ) {
        Err(ERROR_NOACCESS)
    } else {
        distance
            .ok_or(ERROR_NOACCESS)
            .and_then(|distance| seek(emu, handle, distance, method, !extended && output == 0))
    };
    log_red!(
        emu,
        "{} handle:0x{:x} distance:{:?} method:{} => {:?}",
        api,
        handle,
        distance,
        method,
        result
    );
    let value = match result {
        Ok(position) => {
            if output != 0 {
                if extended {
                    emu.maps.write_qword(output, position);
                } else {
                    emu.maps.write_dword(output, (position >> u32::BITS) as u32);
                }
            }
            emu.set_last_error(0);
            if extended { 1 } else { position as u32 as u64 }
        }
        Err(error) => {
            emu.set_last_error(error);
            if extended {
                0
            } else {
                INVALID_SET_FILE_POINTER
            }
        }
    };
    call.finish(emu, value, if split_distance { 5 } else { 4 });
}

fn transfer(
    emu: &mut Emu,
    handle: u64,
    buffer: u64,
    size: usize,
    write: bool,
) -> Result<usize, u64> {
    if size as u64 > emu.maps.max_alloc_size {
        return Err(ERROR_NOT_ENOUGH_MEMORY);
    }
    let accessible = if write {
        emu.maps.can_access_range(buffer, size, Permission::READ)
    } else {
        emu.maps.validate_write_range(buffer, size as u64)
    };
    if size != 0 && !accessible {
        return Err(ERROR_NOACCESS);
    }
    if write {
        let bytes = emu
            .maps
            .try_read_bytes(buffer, size)
            .ok_or(ERROR_NOACCESS)?
            .into_owned();
        let standard = handle == (handle as u32 as u64) || handle == (handle as i32 as i64 as u64);
        let standard_output = matches!(
            helper::handler_get_uri(handle).as_str(),
            "stdio://stdout" | "stdio://stderr"
        );
        if standard_output
            || standard && matches!(handle as u32, STD_OUTPUT_HANDLE | STD_ERROR_HANDLE)
        {
            log_red!(
                emu,
                "standard handle:0x{:x}> {}",
                handle,
                String::from_utf8_lossy(&bytes[..bytes.len().min(STD_OUTPUT_PREVIEW_LEN)])
            );
            return Ok(size);
        }
        file(emu, handle)?
            .write(&bytes)
            .map_err(|e| io_error(&e, ERROR_WRITE_FAULT))
    } else {
        let file = file(emu, handle)?;
        let remaining = file
            .length()
            .map_err(|e| io_error(&e, ERROR_READ_FAULT))?
            .saturating_sub(file.get_position());
        let mut bytes = vec![0; (size as u64).min(remaining) as usize];
        let read = file
            .read(&mut bytes)
            .map_err(|e| io_error(&e, ERROR_READ_FAULT))?;
        if read != 0 && !emu.maps.write_bytes(buffer, &bytes[..read]) {
            return Err(ERROR_NOACCESS);
        }
        Ok(read)
    }
}

pub(crate) fn read_write_file(emu: &mut Emu, api: &str, write: bool) {
    let call = Call::Api;
    let handle = call.arg(emu, 0);
    let buffer = call.arg(emu, 1);
    let size = call.arg(emu, 2) as u32 as usize;
    let count = call.arg(emu, 3);
    let overlapped = call.arg(emu, 4);
    log_red!(
        emu,
        "{} handle:0x{:x} buffer:0x{:x} size:{} overlapped:0x{:x}",
        api,
        handle,
        buffer,
        size,
        overlapped
    );
    let result = if count == 0
        || !emu
            .maps
            .validate_write_range(count, size_of::<u32>() as u64)
    {
        Err(ERROR_NOACCESS)
    } else {
        emu.maps.write_dword(count, 0);
        if overlapped != 0 {
            Err(ERROR_NOT_SUPPORTED)
        } else {
            transfer(emu, handle, buffer, size, write)
        }
    };
    let success = match result {
        Ok(bytes) => {
            emu.maps.write_dword(count, bytes as u32);
            true
        }
        Err(error) => {
            emu.set_last_error(error);
            false
        }
    };
    if write {
        emu.count_write += 1;
    } else {
        emu.count_read += 1;
    }
    call.finish(emu, success as u64, 5);
}

pub(crate) fn flush_file(emu: &mut Emu, api: &str) {
    let call = Call::Api;
    let handle = call.arg(emu, 0);
    let result = file(emu, handle).and_then(|file| {
        file.flush()
            .map_err(|error| io_error(&error, ERROR_WRITE_FAULT))
    });
    log_red!(emu, "{} handle:0x{:x} => {:?}", api, handle, result);
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.finish(emu, result.is_ok() as u64, 1);
}

pub(crate) fn set_end_of_file(emu: &mut Emu, api: &str) {
    let call = Call::Api;
    let handle = call.arg(emu, 0);
    let result = file(emu, handle).and_then(|file| {
        file.set_end_of_file()
            .map_err(|error| io_error(&error, ERROR_WRITE_FAULT))
    });
    log_red!(emu, "{} handle:0x{:x} => {:?}", api, handle, result);
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.finish(emu, result.is_ok() as u64, 1);
}

pub(crate) fn dispatch(api: &str, library: &str, emu: &mut Emu) -> bool {
    let handler: fn(&mut Emu, &str) = match api {
        "CreateFileA" => |emu, name| create_file(emu, name, false),
        "CreateFileW" => |emu, name| create_file(emu, name, true),
        "ReadFile" => |emu, name| read_write_file(emu, name, false),
        "WriteFile" => |emu, name| read_write_file(emu, name, true),
        "GetFileSize" => |emu, name| get_file_size(emu, name, false),
        "GetFileSizeEx" => |emu, name| get_file_size(emu, name, true),
        "SetFilePointer" => |emu, name| set_file_pointer(emu, name, false),
        "SetFilePointerEx" => |emu, name| set_file_pointer(emu, name, true),
        "SetEndOfFile" => set_end_of_file,
        "FlushFileBuffers" => flush_file,
        "GetDiskFreeSpaceA" => |emu, name| get_disk_free_space(emu, name, false),
        "GetDiskFreeSpaceW" => |emu, name| get_disk_free_space(emu, name, true),
        "SetCurrentDirectoryA" => |emu, name| set_current_directory(emu, name, false),
        "SetCurrentDirectoryW" => |emu, name| set_current_directory(emu, name, true),
        "GetWindowsDirectoryA" => |emu, name| get_directory(emu, name, false, Directory::Windows),
        "GetWindowsDirectoryW" => |emu, name| get_directory(emu, name, true, Directory::Windows),
        "GetSystemWindowsDirectoryA" => {
            |emu, name| get_directory(emu, name, false, Directory::Windows)
        }
        "GetSystemWindowsDirectoryW" => {
            |emu, name| get_directory(emu, name, true, Directory::Windows)
        }
        "GetSystemDirectoryA" => |emu, name| get_directory(emu, name, false, Directory::System),
        "GetSystemDirectoryW" => |emu, name| get_directory(emu, name, true, Directory::System),
        "GetTempPathA" => |emu, name| get_directory(emu, name, false, Directory::Temporary),
        "GetTempPathW" => |emu, name| get_directory(emu, name, true, Directory::Temporary),
        "GetCurrentDirectoryA" => |emu, name| get_directory(emu, name, false, Directory::Current),
        "GetCurrentDirectoryW" => |emu, name| get_directory(emu, name, true, Directory::Current),
        _ => return false,
    };
    handler(emu, &format!("{library}!{api}"));
    true
}

pub(crate) fn get_disk_free_space(emu: &mut Emu, api: &str, wide: bool) {
    let call = Call::Api;
    let root_ptr = call.arg(emu, 0);
    let outputs = [
        (call.arg(emu, 1), SECTORS_PER_CLUSTER),
        (call.arg(emu, 2), BYTES_PER_SECTOR),
        (call.arg(emu, 3), FREE_CLUSTERS),
        (call.arg(emu, 4), TOTAL_CLUSTERS),
    ];
    let root = if root_ptr == 0 {
        emu.cfg.cwd_path.clone()
    } else if wide {
        emu.maps.read_wide_string(root_ptr)
    } else {
        emu.maps.read_string(root_ptr)
    };
    log_red!(emu, "{} root:'{}'", api, root);
    if outputs.iter().any(|(address, _)| {
        *address != 0
            && !emu
                .maps
                .validate_write_range(*address, size_of::<u32>() as u64)
    }) {
        emu.set_last_error(ERROR_NOACCESS);
        call.finish(emu, 0, 5);
        return;
    }
    for (address, value) in outputs {
        if address != 0 {
            emu.maps.write_dword(address, value);
        }
    }
    call.finish(emu, 1, 5);
}

pub(crate) fn set_current_directory(emu: &mut Emu, api: &str, wide: bool) {
    let call = Call::Api;
    let path_ptr = call.arg(emu, 0);
    let path = if wide {
        emu.maps.read_wide_string(path_ptr)
    } else {
        emu.maps.read_string(path_ptr)
    };
    log_red!(emu, "{} path:'{}'", api, path);
    if path.is_empty() {
        emu.set_last_error(ERROR_PATH_NOT_FOUND);
        call.finish(emu, 0, 1);
        return;
    }
    emu.cfg.cwd_path = path;
    call.finish(emu, 1, 1);
}

#[derive(Clone, Copy)]
pub(crate) enum Directory {
    Windows,
    System,
    Temporary,
    Current,
}

pub(crate) fn get_directory(emu: &mut Emu, api: &str, wide: bool, directory: Directory) {
    let call = Call::Api;
    let size_first = matches!(directory, Directory::Temporary | Directory::Current);
    let buffer = call.arg(emu, usize::from(size_first));
    let capacity = call.arg(emu, usize::from(!size_first)) as u32 as usize;
    let mut path = match directory {
        Directory::Windows => emu.cfg.windows_directory.clone(),
        Directory::System => emu.cfg.system_directory.clone(),
        Directory::Temporary => emu.cfg.temp_path.clone(),
        Directory::Current => emu.cfg.cwd_path.clone(),
    };
    if matches!(directory, Directory::Temporary) && !path.ends_with('\\') {
        path.push('\\');
    }
    let (bytes, length) = if wide {
        let units: Vec<u16> = path.encode_utf16().collect();
        let length = units.len();
        let bytes = units
            .into_iter()
            .chain([0])
            .flat_map(u16::to_le_bytes)
            .collect::<Vec<_>>();
        (bytes, length)
    } else {
        let mut bytes = path.into_bytes();
        let length = bytes.len();
        bytes.push(0);
        (bytes, length)
    };
    log_red!(
        emu,
        "{} buffer:0x{:x} capacity:{} length:{}",
        api,
        buffer,
        capacity,
        length
    );
    let result = if capacity <= length {
        length + 1
    } else if buffer == 0
        || !emu.maps.validate_write_range(buffer, bytes.len() as u64)
        || !emu.maps.write_bytes(buffer, &bytes)
    {
        emu.set_last_error(ERROR_NOACCESS);
        0
    } else {
        length
    };
    call.finish(emu, result as u64, 2);
}
