use crate::winapi::common::trace::input_string;
use crate::winapi::policy;
mod information;
mod legacy;
mod mutation;
mod names;
mod search;
mod volume;
pub(crate) use information::native_query_file;
pub(crate) use volume::drive_number_type;

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi, WindowsAbi::WinApi};
use crate::winapi::common::guest_files::{self, GuestFileRead};
use crate::winapi::common::memory::read_pointer;
use crate::winapi::common::memory::{pointer_size, read_array, read_guest_bytes, write_pointer};
use crate::winapi::common::{file_names, heap, ipc, ntdll};
use crate::winapi::environment_store;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::files::*;
use crate::windows::constants::process::*;
use crate::windows::constants::security::GENERIC_READ;
use goblin::pe::header::COFF_MACHINE_X86;
use std::io::{self, SeekFrom};

use crate::emu::Emu;
use crate::emu::object_handle::FileHandle;
use crate::emu::object_handle::file_handle::guest_file_metadata;
use crate::maps::mem64::Permission;
use crate::winapi::helper;
use crate::windows::constants::time::{
    FILETIME_TICKS_PER_SECOND, NANOSECONDS_PER_FILETIME_TICK, UNIX_EPOCH_AS_FILETIME,
};
use std::fs::Metadata;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Preserve the public DLL entry points for direct callers and use the same
// implementation or unsupported-API policy as the library gateway.
macro_rules! file_entry_points {
    ($library:literal, $($name:ident),+ $(,)?) => {
        $(
            pub fn $name(emu: &mut $crate::emu::Emu) {
                if !$crate::winapi::common::sections::dispatch(stringify!($name), emu) && !$crate::winapi::common::files::dispatch(stringify!($name), $library, emu) {
                    $crate::winapi::policy::unsupported(emu, $library, stringify!($name), emu.pc());
                }
            }
        )+
    };
}

pub(crate) use file_entry_points;

const SECTORS_PER_CLUSTER: u32 = 8;
const BYTES_PER_SECTOR: u32 = 512;
const FREE_CLUSTERS: u32 = 100_000;
const TOTAL_CLUSTERS: u32 = 200_000;

const STD_OUTPUT_PREVIEW_LEN: usize = 4096;

pub(crate) fn io_error(error: &io::Error, fallback: u64) -> u64 {
    match error.kind() {
        io::ErrorKind::NotFound => ERROR_FILE_NOT_FOUND,
        io::ErrorKind::NotADirectory => ERROR_PATH_NOT_FOUND,
        io::ErrorKind::DirectoryNotEmpty => ERROR_DIR_NOT_EMPTY,
        io::ErrorKind::IsADirectory => ERROR_ACCESS_DENIED,
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

pub(crate) fn to_filetime(time: SystemTime) -> u64 {
    let delta = time.duration_since(UNIX_EPOCH);
    let after_epoch = delta.is_ok();
    let duration = delta.unwrap_or_else(|before| before.duration());
    let ticks = u64::try_from(duration.as_nanos() / u128::from(NANOSECONDS_PER_FILETIME_TICK))
        .unwrap_or(u64::MAX);
    if after_epoch {
        UNIX_EPOCH_AS_FILETIME.saturating_add(ticks)
    } else {
        UNIX_EPOCH_AS_FILETIME.saturating_sub(ticks)
    }
}

fn read_filetime(emu: &Emu, address: u64) -> Result<Option<SystemTime>, u64> {
    if address == 0 {
        return Ok(None);
    }
    let ticks = read_array(emu, address)
        .map(i64::from_le_bytes)
        .ok_or(ERROR_NOACCESS)?;
    // These controls do not supply a replacement timestamp.
    match ticks {
        -2..=0 => return Ok(None),
        ..-2 => return Err(ERROR_INVALID_PARAMETER),
        _ => {}
    }
    let ticks = ticks as u64;
    let delta = ticks.abs_diff(UNIX_EPOCH_AS_FILETIME);
    let elapsed = Duration::new(
        delta / FILETIME_TICKS_PER_SECOND,
        (delta % FILETIME_TICKS_PER_SECOND) as u32 * NANOSECONDS_PER_FILETIME_TICK,
    );
    let time = if ticks < UNIX_EPOCH_AS_FILETIME {
        UNIX_EPOCH.checked_sub(elapsed)
    } else {
        UNIX_EPOCH.checked_add(elapsed)
    };
    time.map(Some).ok_or(ERROR_INVALID_PARAMETER)
}

fn get_file_times(emu: &mut Emu, handle: u64, outputs: [u64; 3]) -> Result<(), u64> {
    let times = file(emu, handle)?
        .times()
        .map_err(|e| io_error(&e, ERROR_ACCESS_DENIED))?;
    if outputs
        .iter()
        .any(|&p| p != 0 && !emu.maps.validate_write_range(p, size_of::<u64>() as u64))
    {
        return Err(ERROR_NOACCESS);
    }
    for (address, time) in outputs.into_iter().zip(times) {
        if address != 0
            && !emu
                .maps
                .write_bytes(address, &to_filetime(time).to_le_bytes())
        {
            return Err(ERROR_NOACCESS);
        }
    }
    Ok(())
}

fn set_file_times(emu: &mut Emu, handle: u64, inputs: [u64; 3]) -> Result<(), u64> {
    let [creation, access, write] = inputs;
    let created = read_filetime(emu, creation)?;
    let accessed = read_filetime(emu, access)?;
    let written = read_filetime(emu, write)?;
    file(emu, handle)?
        .set_times(created, accessed, written)
        .map_err(|e| io_error(&e, ERROR_ACCESS_DENIED))
}

fn file_times(emu: &mut Emu, api: &str, set: bool) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let pointers = [
        api_try!(emu, call.arg(emu, 1)),
        api_try!(emu, call.arg(emu, 2)),
        api_try!(emu, call.arg(emu, 3)),
    ];
    let result = if set {
        set_file_times(emu, handle, pointers)
    } else {
        get_file_times(emu, handle, pointers)
    };
    log_red!(emu, "{api} handle={handle:#x} => {result:?}");
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 4);
}

fn query_metadata(emu: &Emu, pointer: u64, wide: bool) -> Result<Metadata, u64> {
    if pointer == 0 {
        return Err(ERROR_PATH_NOT_FOUND);
    }
    let path = absolute_guest_path(emu, pointer, wide)?;
    guest_file_metadata(&path).map_err(|error| io_error(&error, ERROR_ACCESS_DENIED))
}

fn absolute_guest_path(emu: &Emu, pointer: u64, wide: bool) -> Result<String, u64> {
    let path = file_names::encoding(emu, wide).read(emu, pointer)?;
    if path.is_empty() {
        return Err(ERROR_PATH_NOT_FOUND);
    }
    let path = file_names::absolute_path(&emu.cfg.cwd_path, path)?;
    String::from_utf16(&path).map_err(|_| ERROR_INVALID_PARAMETER)
}

#[cfg(windows)]
fn attribute_bits(metadata: &Metadata) -> u32 {
    use std::os::windows::fs::MetadataExt;
    metadata.file_attributes()
}

#[cfg(not(windows))]
fn attribute_bits(metadata: &Metadata) -> u32 {
    let mut bits = if metadata.is_dir() {
        FILE_ATTRIBUTE_DIRECTORY
    } else {
        FILE_ATTRIBUTE_ARCHIVE
    };
    if metadata.permissions().readonly() {
        bits |= FILE_ATTRIBUTE_READONLY;
    }
    bits as u32
}

pub(crate) enum FileQuery {
    Attributes,
    Extended,
    FromApp,
    Exists,
}

pub(crate) fn query_path(emu: &mut Emu, api: &str, wide: bool, query: FileQuery) {
    let call = WinApi(Stdcall);
    let path = api_try!(emu, call.arg(emu, 0));
    let extended = matches!(query, FileQuery::Extended | FileQuery::FromApp);
    let detail = if extended {
        Some((
            api_try!(emu, call.arg_u32(emu, 1)),
            api_try!(emu, call.arg(emu, 2)),
        ))
    } else {
        None
    };
    log_red!(
        emu,
        "{api} path={}",
        input_string(emu, path, file_names::encoding(emu, wide))
    );
    let result = match detail {
        Some((level, _)) if level != GET_FILEEX_INFO_STANDARD => Err(ERROR_INVALID_PARAMETER),
        Some((_, output)) => query_metadata(emu, path, wide).and_then(|metadata| {
            let bytes = attribute_data(&metadata);
            if output != 0
                && emu.maps.validate_write_range(output, bytes.len() as u64)
                && emu.maps.write_bytes(output, &bytes)
            {
                Ok(1)
            } else {
                Err(ERROR_NOACCESS)
            }
        }),
        None => query_metadata(emu, path, wide).map(|metadata| match query {
            FileQuery::Exists => 1,
            _ => u64::from(attribute_bits(&metadata)),
        }),
    };
    log_red!(emu, "{api} result={result:?}");
    if result.is_ok() && matches!(query, FileQuery::FromApp) {
        emu.set_last_error(ERROR_SUCCESS);
    }
    let value = result.unwrap_or_else(|error| {
        if path != 0 || !matches!(query, FileQuery::Exists) {
            emu.set_last_error(error);
        }
        if matches!(query, FileQuery::Attributes) {
            u64::from(INVALID_FILE_ATTRIBUTES)
        } else {
            0
        }
    });
    call.set_return(emu, value);
    call.cleanup_args(emu, if extended { 3 } else { 1 });
}

fn attribute_data(metadata: &Metadata) -> Vec<u8> {
    let mut bytes = attribute_bits(metadata).to_le_bytes().to_vec();
    for time in [metadata.created(), metadata.accessed(), metadata.modified()] {
        bytes.extend_from_slice(&time.map(to_filetime).unwrap_or(0).to_le_bytes());
    }
    let size = metadata.len();
    bytes.extend_from_slice(&((size >> u32::BITS) as u32).to_le_bytes());
    bytes.extend_from_slice(&(size as u32).to_le_bytes());
    bytes
}

pub(crate) fn create_file(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let name_ptr = api_try!(emu, call.arg(emu, 0));
    let access = api_try!(emu, call.arg_u32(emu, 1));
    let share = api_try!(emu, call.arg_u32(emu, 2));
    let extended = api.ends_with("CreateFile2") || api.ends_with("CreateFile2FromAppW");
    let (disposition, attributes) = if extended {
        let disposition = api_try!(emu, call.arg_u32(emu, 3));
        let params = api_try!(emu, call.arg(emu, 4));
        (disposition, extended_attributes(emu, params))
    } else {
        let disposition = api_try!(emu, call.arg_u32(emu, 4));
        let attributes = api_try!(emu, call.arg_u32(emu, 5));
        (disposition, Ok(attributes))
    };
    let name = absolute_guest_path(emu, name_ptr, wide);
    log_red!(
        emu,
        "{api} path={name:?} access={access:#x} disposition={disposition}"
    );
    let result = match (attributes, name) {
        (Ok(flags), Ok(name)) => {
            let Some(result) = open_file(emu, name, access, disposition, flags, share) else {
                return;
            };
            result
        }
        (Err(error), _) | (_, Err(error)) => Err(error),
    };
    let handle = match result {
        Ok((handle, existed)) => {
            if matches!(disposition as usize, OPEN_ALWAYS | CREATE_ALWAYS) {
                emu.set_last_error(if existed {
                    ERROR_ALREADY_EXISTS
                } else {
                    ERROR_SUCCESS
                });
            }
            handle
        }
        Err(error) => {
            emu.set_last_error(error);
            if emu.cfg.arch.is_64bits() {
                INVALID_HANDLE_VALUE_64
            } else {
                INVALID_HANDLE_VALUE_32
            }
        }
    };
    call.set_return(emu, handle);
    call.cleanup_args(emu, if extended { 5 } else { 7 });
}

fn open_file(
    emu: &mut Emu,
    name: String,
    access: u32,
    disposition: u32,
    flags: u32,
    share: u32,
) -> Option<Result<(u64, bool), u64>> {
    if let Some(result) = ipc::open(emu, &name, access, disposition, flags) {
        return Some(result.map(|handle| (handle, false)));
    }
    if disposition as usize == OPEN_EXISTING
        && matches!(access, GENERIC_READ | FILE_READ_DATA)
        && flags as usize
            & (FILE_FLAG_DELETE_ON_CLOSE | FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OVERLAPPED)
            == 0
    {
        match guest_files::read_file(emu, &name, share) {
            GuestFileRead::File(bytes) => {
                let file = FileHandle::from_guest_bytes(name, bytes, access, flags, share);
                return Some(Ok((
                    u64::from(emu.handle_management.insert_file_handle(file)),
                    true,
                )));
            }
            GuestFileRead::Failed(status) => return Some(Err(guest_files::win32_error(status))),
            GuestFileRead::Stopped => return None,
            GuestFileRead::NotIntercepted => {}
        }
    }
    Some(
        FileHandle::open(name, access, disposition, flags, share)
            .map(|(file, existed)| {
                (
                    u64::from(emu.handle_management.insert_file_handle(file)),
                    existed,
                )
            })
            .map_err(|error| {
                error
                    .downcast_ref::<io::Error>()
                    .map_or(ERROR_INVALID_PARAMETER, |e| {
                        io_error(e, ERROR_ACCESS_DENIED)
                    })
            }),
    )
}

fn extended_attributes(emu: &Emu, pointer: u64) -> Result<u32, u64> {
    if pointer == 0 {
        return Ok(FILE_ATTRIBUTE_NORMAL as u32);
    }
    // Four DWORDs followed by two guest pointers in CREATEFILE2_EXTENDED_PARAMETERS.
    let expected = 4 * size_of::<u32>() + 2 * pointer_size(emu) as usize;
    let header = read_array::<16>(emu, pointer).ok_or(ERROR_NOACCESS)?;
    let word = |offset: usize| {
        u32::from_le_bytes([
            header[offset],
            header[offset + 1],
            header[offset + 2],
            header[offset + 3],
        ])
    };
    if word(0) != expected as u32 {
        return Err(ERROR_INVALID_PARAMETER);
    }
    if !(16..expected).all(|offset| {
        pointer
            .checked_add(offset as u64)
            .and_then(|p| emu.maps.read_byte(p))
            .is_some()
    }) {
        return Err(ERROR_NOACCESS);
    }
    Ok(word(4) | word(8))
}

pub(crate) fn get_file_size(emu: &mut Emu, api: &str, extended: bool) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
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
    log_red!(emu, "{} handle=0x{:x} => {:?}", api, handle, result);
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
    call.set_return(emu, value);
    call.cleanup_args(emu, 2);
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
    let call = WinApi(Stdcall);
    let low = api_try!(emu, call.arg(emu, 1));
    let handle = api_try!(emu, call.arg(emu, 0));
    let split_distance = extended && !emu.cfg.arch.is_64bits();
    let output = api_try!(emu, call.arg(emu, if split_distance { 3 } else { 2 }));
    let method = api_try!(emu, call.arg_u32(emu, if split_distance { 4 } else { 3 }));
    let distance = if split_distance {
        Some(((api_try!(emu, call.arg(emu, 2)) << u32::BITS) | low) as i64)
    } else if extended {
        Some(low as i64)
    } else if output != 0 {
        emu.maps
            .read_dword(output)
            .map(|high| (((high as u64) << u32::BITS) | low as u32 as u64) as i64)
    } else {
        Some(low as i32 as i64)
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
        "{} handle=0x{:x} distance={:?} method={} => {:?}",
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
    call.set_return(emu, value);
    call.cleanup_args(emu, if split_distance { 5 } else { 4 });
}

fn transfer(
    emu: &mut Emu,
    handle: u64,
    buffer: u64,
    size: usize,
    write: bool,
    offset: Option<u64>,
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
    if offset.is_none()
        && let Some(result) = ipc::transfer(emu, handle, buffer, size, write)
    {
        return result;
    }
    if write {
        let bytes = read_guest_bytes(emu, buffer, size).ok_or(ERROR_NOACCESS)?;
        let standard = handle == (handle as u32 as u64) || handle == (handle as i32 as i64 as u64);
        let standard_output = matches!(
            helper::handler_get_uri(handle).as_str(),
            "stdio://stdout" | "stdio://stderr"
        );
        if emu
            .handle_management
            .console(handle)
            .is_some_and(|(selector, _)| selector != STD_INPUT_HANDLE)
            || standard_output
            || standard && matches!(handle as u32, STD_OUTPUT_HANDLE | STD_ERROR_HANDLE)
        {
            log_red!(
                emu,
                "standard handle=0x{:x}> {}",
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
            .saturating_sub(offset.unwrap_or_else(|| file.get_position()));
        let mut bytes = vec![0; (size as u64).min(remaining) as usize];
        let original_position = file.get_position();
        let seek = offset.filter(|_| !bytes.is_empty());
        if let Some(start) = seek {
            file.seek(SeekFrom::Start(start))
                .map_err(|e| io_error(&e, ERROR_READ_FAULT))?;
        }
        let read = file.read(&mut bytes).map_err(|error| {
            if seek.is_some() {
                let _ = file.seek(SeekFrom::Start(original_position));
            }
            io_error(&error, ERROR_READ_FAULT)
        })?;
        if read != 0 && !emu.maps.write_bytes(buffer, &bytes[..read]) {
            return Err(ERROR_NOACCESS);
        }
        Ok(read)
    }
}

pub(crate) fn read_write_file(emu: &mut Emu, api: &str, write: bool) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let buffer = api_try!(emu, call.arg(emu, 1));
    let size = api_try!(emu, call.arg_u32(emu, 2)) as usize;
    let count = api_try!(emu, call.arg(emu, 3));
    let overlapped = api_try!(emu, call.arg(emu, 4));
    log_red!(
        emu,
        "{} handle=0x{:x} buffer=0x{:x} size={} overlapped=0x{:x}",
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
            transfer(emu, handle, buffer, size, write, None)
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
    call.set_return(emu, success as u64);
    call.cleanup_args(emu, 5);
}

pub(crate) fn flush_file(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let result = file(emu, handle).and_then(|file| {
        file.flush()
            .map_err(|error| io_error(&error, ERROR_WRITE_FAULT))
    });
    log_red!(emu, "{} handle=0x{:x} => {:?}", api, handle, result);
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, result.is_ok() as u64);
    call.cleanup_args(emu, 1);
}

pub(crate) fn set_end_of_file(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let result = file(emu, handle).and_then(|file| {
        file.set_end_of_file()
            .map_err(|error| io_error(&error, ERROR_WRITE_FAULT))
    });
    log_red!(emu, "{} handle=0x{:x} => {:?}", api, handle, result);
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, result.is_ok() as u64);
    call.cleanup_args(emu, 1);
}

pub(crate) fn dispatch(api: &str, library: &str, emu: &mut Emu) -> bool {
    if legacy::dispatch(library, api, emu)
        || information::dispatch(library, api, emu)
        || names::dispatch(library, api, emu)
        || mutation::dispatch(library, api, emu)
        || search::dispatch(library, api, emu)
        || volume::dispatch(library, api, emu)
    {
        return true;
    }
    let handler: fn(&mut Emu, &str) = match api {
        "AreFileApisANSI" | "AreFileApiIsAnsi" => |emu, _| file_names::AreFileApisANSI(emu),
        "SetFileApisToANSI" => |emu, _| file_names::SetFileApisToANSI(emu),
        "SetFileApisToOEM" => |emu, _| file_names::SetFileApisToOEM(emu),
        "GetFullPathNameA" => |emu, _| file_names::GetFullPathNameA(emu),
        "GetFullPathNameW" => |emu, _| file_names::GetFullPathNameW(emu),
        "PathFileExistsA" | "PathFileExistsW" if library == "kernelbase" => {
            |emu, name| query_path(emu, name, name.ends_with('W'), FileQuery::Exists)
        }
        "CreateFileA" => |emu, name| create_file(emu, name, false),
        "CreateFile2" => |emu, name| create_file(emu, name, true),
        "CreateFile2FromAppW" if library == "kernelbase" => {
            |emu, name| create_file(emu, name, true)
        }
        "CreateFileW" => |emu, name| create_file(emu, name, true),
        "CreateFileFromAppW" if library == "kernelbase" => |emu, name| create_file(emu, name, true),
        "GetFileType" => get_file_type,
        "ReadFile" => |emu, name| read_write_file(emu, name, false),
        "WriteFile" => |emu, name| read_write_file(emu, name, true),
        "GetFileSize" => |emu, name| get_file_size(emu, name, false),
        "GetFileSizeEx" => |emu, name| get_file_size(emu, name, true),
        "SetFilePointer" => |emu, name| set_file_pointer(emu, name, false),
        "SetFilePointerEx" => |emu, name| set_file_pointer(emu, name, true),
        "SetEndOfFile" => set_end_of_file,
        "FlushFileBuffers" => flush_file,
        "GetFileTime" => |emu, name| file_times(emu, name, false),
        "SetFileTime" => |emu, name| file_times(emu, name, true),
        "GetFileAttributesExFromAppW" if library == "kernelbase" => {
            |emu, name| query_path(emu, name, true, FileQuery::FromApp)
        }
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
        "GetSystemWow64DirectoryA" | "GetSystemWow64DirectoryW" => {
            |emu, name| get_directory(emu, name, name.ends_with('W'), Directory::Wow64)
        }
        "GetSystemWow64Directory2A" | "GetSystemWow64Directory2W" => {
            |emu, name| get_directory(emu, name, name.ends_with('W'), Directory::Wow64Machine)
        }
        "GetTempPath2A" | "GetTempPath2W" => {
            |emu, name| get_directory(emu, name, name.ends_with('W'), Directory::Temporary2)
        }
        "GetTempPathA" => |emu, name| get_directory(emu, name, false, Directory::Temporary),
        "GetTempPathW" => |emu, name| get_directory(emu, name, true, Directory::Temporary),
        "GetCurrentDirectoryA" => |emu, name| get_directory(emu, name, false, Directory::Current),
        "GetCurrentDirectoryW" => |emu, name| get_directory(emu, name, true, Directory::Current),
        _ => return false,
    };
    handler(emu, &format!("{library}!{api}"));
    true
}

fn get_file_type(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let uri = helper::handler_get_uri(handle);
    let ty = if let Ok(guest) = file(emu, handle) {
        if guest.is_null_device() {
            FILE_TYPE_CHAR
        } else {
            FILE_TYPE_DISK
        }
    } else if emu.ipc.contains(handle)
        || uri.starts_with("pipe://")
        || uri.starts_with("named_pipe://")
    {
        FILE_TYPE_PIPE
    } else if emu.handle_management.is_console_device(handle)
        || emu.handle_management.console(handle).is_some()
        || uri.starts_with("stdio://")
        || uri.starts_with("console://")
        || (handle == u64::from(handle as u32) || handle == handle as i32 as i64 as u64)
            && matches!(
                handle as u32,
                STD_INPUT_HANDLE | STD_OUTPUT_HANDLE | STD_ERROR_HANDLE
            )
    {
        FILE_TYPE_CHAR
    } else {
        emu.set_last_error(ERROR_INVALID_HANDLE);
        FILE_TYPE_UNKNOWN
    };
    log_red!(emu, "{api} handle={handle:#x} => {ty}");
    call.set_return(emu, ty);
    call.cleanup_args(emu, 1);
}

pub(crate) fn get_disk_free_space(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let root_ptr = api_try!(emu, call.arg(emu, 0));
    let outputs = [
        (api_try!(emu, call.arg(emu, 1)), SECTORS_PER_CLUSTER),
        (api_try!(emu, call.arg(emu, 2)), BYTES_PER_SECTOR),
        (api_try!(emu, call.arg(emu, 3)), FREE_CLUSTERS),
        (api_try!(emu, call.arg(emu, 4)), TOTAL_CLUSTERS),
    ];
    let root = volume::check_disk_directory(emu, root_ptr, wide);
    log_red!(emu, "{api} root={root_ptr:#x} => {root:?}");
    if let Err(error) = root {
        emu.set_last_error(error);
        call.set_return(emu, 0);
        call.cleanup_args(emu, 5);
        return;
    }
    if outputs.iter().any(|(address, _)| {
        *address != 0
            && !emu
                .maps
                .validate_write_range(*address, size_of::<u32>() as u64)
    }) {
        emu.set_last_error(ERROR_NOACCESS);
        call.set_return(emu, 0);
        call.cleanup_args(emu, 5);
        return;
    }
    for (address, value) in outputs {
        if address != 0 {
            emu.maps.write_dword(address, value);
        }
    }
    call.set_return(emu, 1);
    call.cleanup_args(emu, 5);
}

pub(crate) fn set_current_directory(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let path =
        absolute_guest_path(emu, path_ptr, wide).and_then(|path| {
            match guest_file_metadata(&path) {
                Ok(meta) if meta.is_dir() => Ok(path),
                Ok(_) => Err(ERROR_DIRECTORY),
                Err(error) if error.kind() == io::ErrorKind::NotFound => Err(ERROR_PATH_NOT_FOUND),
                Err(error) => Err(io_error(&error, ERROR_PATH_NOT_FOUND)),
            }
        });
    log_red!(emu, "{} path={:?}", api, path);
    match path {
        Ok(path) => {
            emu.cfg.cwd_path = path;
            call.set_return(emu, 1);
        }
        Err(error) => {
            emu.set_last_error(error);
            call.set_return(emu, 0);
        }
    }
    call.cleanup_args(emu, 1);
}

#[derive(Clone, Copy)]
pub(crate) enum Directory {
    Windows,
    System,
    Temporary,
    Temporary2,
    Wow64,
    Wow64Machine,
    Current,
}

pub(crate) fn get_directory(emu: &mut Emu, api: &str, wide: bool, directory: Directory) {
    let call = WinApi(Stdcall);
    let size_first = matches!(
        directory,
        Directory::Temporary | Directory::Temporary2 | Directory::Current
    );
    let slots = if matches!(directory, Directory::Wow64Machine) {
        3
    } else {
        2
    };
    let buffer = api_try!(emu, call.arg(emu, usize::from(size_first)));
    let capacity = api_try!(emu, call.arg_u32(emu, usize::from(!size_first))) as usize;
    let mut path = match directory {
        Directory::Windows => emu.cfg.windows_directory.clone(),
        Directory::System => emu.cfg.system_directory.clone(),
        Directory::Wow64 | Directory::Wow64Machine => {
            if matches!(directory, Directory::Wow64Machine)
                && api_try!(emu, call.arg_u16(emu, 2)) != COFF_MACHINE_X86
            {
                emu.set_last_error(ERROR_BAD_ARGUMENTS);
                call.set_return(emu, 0);
                call.cleanup_args(emu, slots);
                return;
            }
            format!(
                r"{}\SysWOW64",
                emu.cfg.windows_directory.trim_end_matches('\\')
            )
        }
        Directory::Temporary2 if emu.cfg.user_name.eq_ignore_ascii_case("SYSTEM") => {
            let key: Vec<_> = "SystemTemp".encode_utf16().collect();
            environment_store::process(emu)
                .get(&key)
                .and_then(|v| String::from_utf16(v).ok())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| {
                    format!(
                        r"{}\SystemTemp",
                        emu.cfg.windows_directory.trim_end_matches('\\')
                    )
                })
        }
        Directory::Temporary2 => emu.cfg.temp_path.clone(),
        Directory::Temporary => emu.cfg.temp_path.clone(),
        Directory::Current => emu.cfg.cwd_path.clone(),
    };
    if matches!(directory, Directory::Temporary | Directory::Temporary2) && !path.ends_with('\\') {
        path.push('\\');
    }
    let codec = file_names::encoding(emu, wide);
    let bytes = codec.terminated(&path.encode_utf16().collect::<Vec<_>>());
    let length = bytes.len() / codec.width() - 1;
    log_red!(
        emu,
        "{} buffer=0x{:x} capacity={} length={}",
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
    call.set_return(emu, result as u64);
    call.cleanup_args(emu, slots);
}

pub(crate) fn device_io_control(emu: &mut Emu, library: &str) {
    let call = WinApi(Stdcall);
    let [
        device,
        code,
        input,
        input_size,
        output,
        output_size,
        returned,
        overlapped,
    ] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "{library}!DeviceIoControl device={device:#x} control_code={:#x} input={input:#x} input_size={} output={output:#x} output_size={} bytes_returned={returned:#x} overlapped={overlapped:#x}",
        code as u32,
        input_size as u32,
        output_size as u32
    );
    let valid = emu.handle_management.console(device).is_some()
        || emu.handle_management.is_console_device(device)
        || emu.file_handles.contains_key(&device)
        || u32::try_from(device)
            .ok()
            .is_some_and(|handle| emu.handle_management.get_mut_file_handle(handle).is_some());
    let error = if !valid {
        ERROR_INVALID_HANDLE
    } else if returned != 0
        && !emu
            .maps
            .validate_write_range(returned, size_of::<u32>() as u64)
    {
        ERROR_NOACCESS
    } else {
        policy::unsupported(emu, library, "DeviceIoControl", emu.pc());
        return;
    };
    emu.set_last_error(error);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 8);
}

pub(crate) fn nt_transfer_file(emu: &mut Emu, write: bool) {
    let api = if write { "NtWriteFile" } else { "NtReadFile" };
    let call = WinApi(Stdcall);
    let [handle, event, apc, context, iosb, buf, len, offset, key] = api_try!(emu, call.args(emu));
    log_red!(emu, "{api} handle=0x{:x} size={}", handle, len as u32);
    if event != 0 || apc != 0 || context != 0 || (write && offset != 0) || key != 0 {
        policy::unsupported(emu, "ntdll", api, emu.pc());
        return;
    }
    let width = pointer_size(emu);
    let status = if iosb == 0 || !emu.maps.validate_write_range(iosb, width * 2) {
        STATUS_ACCESS_VIOLATION
    } else {
        let byte_offset = if offset == 0 {
            Ok(None)
        } else {
            match emu.maps.read_qword(offset).map(|value| value as i64) {
                Some(FILE_USE_FILE_POINTER_POSITION) => Ok(None),
                Some(value) if value >= 0 => Ok(Some(value as u64)),
                Some(_) => Err(STATUS_INVALID_PARAMETER),
                None => Err(STATUS_ACCESS_VIOLATION),
            }
        };
        let offset = match byte_offset {
            Ok(offset) => offset,
            Err(status) => {
                call.set_return(emu, status);
                call.cleanup_args(emu, 9);
                return;
            }
        };
        let result = transfer(emu, handle, buf, len as u32 as usize, write, offset);
        let (status, bytes) = match result {
            Ok(0) if !write && len as u32 != 0 && file(emu, handle).is_ok() => {
                (STATUS_END_OF_FILE, 0)
            }
            Ok(transferred) => (STATUS_SUCCESS, transferred as u64),
            Err(ERROR_INVALID_HANDLE) => (STATUS_INVALID_HANDLE, 0),
            Err(ERROR_NOACCESS) => (STATUS_ACCESS_VIOLATION, 0),
            Err(ERROR_NOT_ENOUGH_MEMORY) => (STATUS_NO_MEMORY, 0),
            Err(ERROR_ACCESS_DENIED) => (STATUS_ACCESS_DENIED, 0),
            Err(_) => (STATUS_UNSUCCESSFUL, 0),
        };
        write_pointer(emu, iosb, status);
        write_pointer(emu, iosb + width, bytes);
        status
    };
    call.set_return(emu, status);
    call.cleanup_args(emu, 9);
}

pub(crate) fn query_volume_information(emu: &mut Emu, call: WindowsAbi) {
    let [handle, io, output, length, class] = api_try!(emu, call.args(emu));
    if class as u32 != FILE_FS_DEVICE_INFORMATION
        || !(emu.handle_management.is_console_device(handle)
            || emu.handle_management.console(handle).is_some())
    {
        return policy::unsupported_with_abi(
            emu,
            "ntdll",
            "NtQueryVolumeInformationFile",
            emu.pc(),
            call,
        );
    }
    let width = pointer_size(emu);
    let size = size_of::<[u32; 2]>() as u64;
    let status = if (length as u32) < size as u32 {
        STATUS_INFO_LENGTH_MISMATCH
    } else if !emu.maps.validate_write_range(output, size)
        || !emu.maps.validate_write_range(io, 2 * width)
    {
        STATUS_ACCESS_VIOLATION
    } else {
        emu.maps.write_dword(output, FILE_DEVICE_CONSOLE);
        emu.maps
            .write_dword(output + 4, FILE_DEVICE_ALLOW_APPCONTAINER_TRAVERSAL);
        write_pointer(emu, io, STATUS_SUCCESS);
        write_pointer(emu, io + width, size);
        STATUS_SUCCESS
    };
    call.set_return(emu, status);
    call.cleanup_args(emu, 5);
}

pub(crate) fn native_path(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [dos, descriptor, part, relative] = api_try!(emu, call.args(emu));
    let width = pointer_size(emu);
    let result = (|| {
        let units = file_names::encoding(emu, true).read(emu, dos).ok()?;
        log_red!(
            emu,
            "ntdll!RtlDosPathNameToNtPathName_U path={:?}",
            String::from_utf16_lossy(&units)
        );
        let (name, last) = file_names::native_path(&emu.cfg.cwd_path, units).ok()?;
        let size = u16::try_from(name.len().checked_add(1)?.checked_mul(2)?).ok()?;
        if descriptor == 0
            || !emu.maps.validate_write_range(descriptor, 2 * width)
            || (part != 0 && !emu.maps.validate_write_range(part, width))
            || (relative != 0 && !emu.maps.validate_write_range(relative, 4 * width))
        {
            return None;
        }
        let buffer = heap::heap_allocate(emu, 0, u64::from(size))?;
        let bytes: Vec<_> = name
            .into_iter()
            .chain(Some(0))
            .flat_map(u16::to_le_bytes)
            .collect();
        let ok = emu.maps.write_bytes(buffer, &bytes)
            && emu
                .maps
                .write_bytes(descriptor, &[0; 16][..(2 * width) as usize])
            && emu.maps.write_word(descriptor, size - 2)
            && emu.maps.write_word(descriptor + 2, size)
            && write_pointer(emu, descriptor + width, buffer)
            && (part == 0 || write_pointer(emu, part, last.map_or(0, |i| buffer + 2 * i as u64)))
            && (relative == 0
                || emu
                    .maps
                    .write_bytes(relative, &[0; 32][..(4 * width) as usize]));
        if !ok {
            heap::heap_free(emu, 0, buffer);
        }
        ok.then_some(())
    })();
    log_red!(emu, "ntdll!RtlDosPathNameToNtPathName_U result={result:?}");
    call.set_return(emu, u64::from(result.is_some()));
    call.cleanup_args(emu, 4);
}
pub(crate) fn native_open_file(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let open = api == "NtOpenFile";
    let [
        out,
        access,
        object,
        io,
        allocation,
        attributes,
        share,
        disposition,
        options,
        ea,
        ea_len,
    ] = if open {
        let [out, access, object, io, share, options] = api_try!(emu, call.args(emu));
        [
            out,
            access,
            object,
            io,
            0,
            0,
            share,
            u64::from(FILE_OPEN),
            options,
            0,
            0,
        ]
    } else {
        api_try!(emu, call.args(emu))
    };
    let width = pointer_size(emu);
    let result = (|| {
        if out == 0
            || io == 0
            || !emu.maps.validate_write_range(out, width)
            || !emu.maps.validate_write_range(io, 2 * width)
        {
            return Err(STATUS_ACCESS_VIOLATION);
        }
        if object == 0
            || !emu
                .maps
                .can_access_range(object, (6 * width) as usize, Permission::READ)
        {
            return Err(STATUS_ACCESS_VIOLATION);
        }
        if emu.maps.read_dword(object) != Some((6 * width) as u32) {
            return Err(STATUS_INVALID_PARAMETER);
        }
        let field = |i| read_pointer(emu, object + i * width).ok_or(STATUS_ACCESS_VIOLATION);
        if field(1)? != 0
            || field(4)? != 0
            || field(5)? != 0
            || emu
                .maps
                .read_dword(object + 3 * width)
                .ok_or(STATUS_ACCESS_VIOLATION)?
                & !(OBJ_CASE_INSENSITIVE | OBJ_OPENIF)
                != 0
            || ea != 0
            || ea_len as u32 != 0
            || allocation != 0
            || options as u32
                & !(FILE_SYNCHRONOUS_IO_ALERT
                    | FILE_SYNCHRONOUS_IO_NONALERT
                    | FILE_NON_DIRECTORY_FILE)
                != 0
        {
            return Err(STATUS_NOT_SUPPORTED);
        }
        let native = ntdll::read_unicode_string(emu, field(2)?)?;
        if native.eq_ignore_ascii_case(r"\??\NUL") || native.eq_ignore_ascii_case(r"\Device\Null") {
            if disposition as u32 != FILE_OPEN {
                return Err(STATUS_NOT_SUPPORTED);
            }
            let file =
                FileHandle::null_device(native, access as u32, attributes as u32, share as u32);
            let handle = emu.handle_management.insert_file_handle(file);
            write_pointer(emu, out, u64::from(handle));
            // The null device completes the create request with zero Information.
            return Ok(0);
        }
        if open && access as u32 & !(GENERIC_READ | FILE_GENERIC_READ) != 0 {
            return Err(STATUS_NOT_SUPPORTED);
        }
        let path = native.strip_prefix(r"\??\").ok_or(STATUS_NOT_SUPPORTED)?;
        let bytes = path.as_bytes();
        if !matches!(bytes,[drive,b':',b'\\',..] if drive.is_ascii_alphabetic()) {
            return Err(STATUS_NOT_SUPPORTED);
        }
        let disp = match disposition as u32 {
            FILE_OPEN => OPEN_EXISTING,
            FILE_CREATE => CREATE_NEW,
            FILE_OPEN_IF => OPEN_ALWAYS,
            FILE_OVERWRITE => TRUNCATE_EXISTING,
            FILE_OVERWRITE_IF => CREATE_ALWAYS,
            0 => return Err(STATUS_NOT_SUPPORTED),
            _ => return Err(STATUS_INVALID_PARAMETER),
        };
        let (file, existed) = FileHandle::open(
            path.to_owned(),
            access as u32,
            disp as u32,
            attributes as u32,
            share as u32,
        )
        .map_err(
            |e| match e.downcast_ref::<io::Error>().map(io::Error::kind) {
                Some(io::ErrorKind::NotFound) => STATUS_OBJECT_NAME_NOT_FOUND,
                Some(io::ErrorKind::NotADirectory) => STATUS_OBJECT_PATH_NOT_FOUND,
                Some(io::ErrorKind::AlreadyExists) => STATUS_OBJECT_NAME_COLLISION,
                Some(io::ErrorKind::PermissionDenied) => STATUS_ACCESS_DENIED,
                _ => STATUS_INVALID_PARAMETER,
            },
        )?;
        let info = match disp {
            OPEN_EXISTING => FILE_OPENED,
            TRUNCATE_EXISTING => FILE_OVERWRITTEN,
            CREATE_ALWAYS if existed => FILE_OVERWRITTEN,
            OPEN_ALWAYS if existed => FILE_OPENED,
            _ => FILE_CREATED,
        };
        let handle = emu.handle_management.insert_file_handle(file);
        write_pointer(emu, out, u64::from(handle));
        Ok(info)
    })();
    if result == Err(STATUS_NOT_SUPPORTED) {
        policy::unsupported(emu, "ntdll", api, emu.pc());
        return;
    }
    let status = result.err().unwrap_or(STATUS_SUCCESS);
    if let Ok(information) = result {
        write_pointer(emu, io, STATUS_SUCCESS);
        write_pointer(emu, io + width, information);
    }
    log_red!(
        emu,
        "ntdll!{api} object={object:#x} access={access:#x} status={status:#x}"
    );
    call.set_return(emu, status);
    call.cleanup_args(emu, if open { 6 } else { 11 });
}
