use std::{fs, io};

use super::{
    absolute_guest_path, attribute_bits, file_names, guest_file_metadata, io_error, query_metadata,
};
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::{Emu, object_handle::file_handle::guest_operation_path};
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::{errors::*, files::*};

fn temp_file(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let directory = api_try!(emu, call.arg(emu, 0));
    let prefix = api_try!(emu, call.arg(emu, 1));
    let unique = api_try!(emu, call.arg_u32(emu, 2)) as u16;
    let out = api_try!(emu, call.arg(emu, 3));
    let wide = api.ends_with('W');
    let codec = file_names::encoding(emu, wide);
    let result = (|| {
        let dir = absolute_guest_path(emu, directory, wide)?;
        if !guest_file_metadata(&dir).is_ok_and(|m| m.is_dir()) {
            return Err(ERROR_DIRECTORY);
        }
        let prefix = if prefix == 0 {
            String::new()
        } else {
            file_names::read_path(emu, prefix, wide)?
                .chars()
                .take(3)
                .collect()
        };
        if prefix.contains(['\\', '/', ':']) {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let start = if unique == 0 {
            (emu.tick as u16).max(1)
        } else {
            unique
        };
        let mut id = start;
        loop {
            let path = format!(r"{}\{prefix}{id:X}.tmp", dir.trim_end_matches('\\'));
            let bytes = codec.terminated(&path.encode_utf16().collect::<Vec<_>>());
            if bytes.len() / codec.width() > MAX_PATH {
                return Err(ERROR_FILENAME_EXCED_RANGE);
            }
            if out == 0 || !emu.maps.validate_write_range(out, bytes.len() as u64) {
                return Err(ERROR_NOACCESS);
            }
            if unique == 0 {
                let host = guest_operation_path(&path, true)
                    .map_err(|e| io_error(&e, ERROR_ACCESS_DENIED))?;
                match fs::File::create_new(host) {
                    Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                        id = id.wrapping_add(1).max(1);
                        if id == start {
                            return Err(ERROR_FILE_EXISTS);
                        }
                        continue;
                    }
                    Err(e) => return Err(io_error(&e, ERROR_ACCESS_DENIED)),
                    Ok(_) => {}
                }
            }
            emu.maps.write_bytes(out, &bytes);
            return Ok(u64::from(id));
        }
    })();
    log_red!(emu, "{library}!{api} => {result:?}");
    let value = result.unwrap_or_else(|e| {
        emu.set_last_error(e);
        0
    });
    call.set_return(emu, value);
    call.cleanup_args(emu, 4);
}

fn attributes(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    let attrs = api_try!(emu, call.arg_u32(emu, 1)) as usize;
    if !matches!(attrs, FILE_ATTRIBUTE_READONLY | FILE_ATTRIBUTE_NORMAL) {
        policy::unsupported(emu, library, api, emu.pc());
        return;
    }
    let result = absolute_guest_path(emu, address, api.ends_with('W')).and_then(|path| {
        let host =
            guest_operation_path(&path, true).map_err(|e| io_error(&e, ERROR_ACCESS_DENIED))?;
        let meta = fs::symlink_metadata(&host).map_err(|e| io_error(&e, ERROR_ACCESS_DENIED))?;
        if !meta.is_file() {
            return Err(ERROR_ACCESS_DENIED);
        }
        let readonly = attrs == FILE_ATTRIBUTE_READONLY;
        let mut permissions = meta.permissions();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            const WRITE_BITS: u32 = 0o222;
            const OWNER_WRITE: u32 = 0o200;
            let mode = permissions.mode();
            permissions.set_mode(if readonly {
                mode & !WRITE_BITS
            } else {
                mode | OWNER_WRITE
            });
        }
        #[cfg(not(unix))]
        permissions.set_readonly(readonly);
        fs::set_permissions(host, permissions).map_err(|e| io_error(&e, ERROR_ACCESS_DENIED))
    });
    log_red!(emu, "{library}!{api} => {result:?}");
    if let Err(e) = result {
        emu.set_last_error(e);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 2);
}

fn existing_path(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let input = api_try!(emu, call.arg(emu, 0));
    let out = api_try!(emu, call.arg(emu, 1));
    let capacity = api_try!(emu, call.arg_u32(emu, 2)) as usize;
    let codec = file_names::encoding(emu, api.ends_with('W'));
    let result = (|| {
        // The guest file system does not generate alternate 8.3 names.
        let spelling = codec.read(emu, input)?;
        let full = file_names::absolute_path(&emu.cfg.cwd_path, spelling.clone())?;
        let name = String::from_utf16(&full).map_err(|_| ERROR_INVALID_PARAMETER)?;
        guest_file_metadata(&name).map_err(|e| io_error(&e, ERROR_ACCESS_DENIED))?;
        let bytes = codec.terminated(&spelling);
        let needed = bytes.len() / codec.width();
        if capacity < needed {
            return Ok(needed);
        }
        if out == 0
            || !emu.maps.validate_write_range(out, bytes.len() as u64)
            || !emu.maps.write_bytes(out, &bytes)
        {
            return Err(ERROR_NOACCESS);
        }
        Ok(needed - 1)
    })();
    log_red!(emu, "{library}!{api} => {result:?}");
    let length = result.unwrap_or_else(|e| {
        emu.set_last_error(e);
        0
    });
    call.set_return(emu, length as u64);
    call.cleanup_args(emu, 3);
}

fn compressed_size(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let input = api_try!(emu, call.arg(emu, 0));
    let high = api_try!(emu, call.arg(emu, 1));
    let result = query_metadata(emu, input, api.ends_with('W')).and_then(|meta| {
        let flags = attribute_bits(&meta);
        if flags
            & (FILE_ATTRIBUTE_COMPRESSED as u32
                | FILE_ATTRIBUTE_SPARSE_FILE
                | FILE_ATTRIBUTE_REPARSE_POINT)
            != 0
        {
            return Ok(None);
        }
        if meta.is_dir() {
            return Err(ERROR_ACCESS_DENIED);
        }
        let size = meta.len();
        if high != 0
            && (!emu.maps.validate_write_range(high, size_of::<u32>() as u64)
                || !emu.maps.write_dword(high, (size >> u32::BITS) as u32))
        {
            return Err(ERROR_NOACCESS);
        }
        Ok(Some(u64::from(size as u32)))
    });
    log_red!(emu, "{library}!{api} => {result:?}");
    let value = match result {
        Ok(Some(value)) => {
            if value == INVALID_FILE_SIZE {
                emu.set_last_error(ERROR_SUCCESS);
            }
            value
        }
        Ok(None) => {
            policy::unsupported(emu, library, api, emu.pc());
            return;
        }
        Err(error) => {
            emu.set_last_error(error);
            INVALID_FILE_SIZE
        }
    };
    call.set_return(emu, value);
    call.cleanup_args(emu, 2);
}

fn executable_kind(name: &str) -> Result<Option<u32>, u64> {
    use goblin::pe::{
        characteristic::IMAGE_FILE_DLL,
        header::{DOS_MAGIC, PE_MAGIC, PE_POINTER_OFFSET, SIZEOF_COFF_HEADER},
        optional_header::{MAGIC_32, MAGIC_64},
    };
    use std::io::{Read, Seek, SeekFrom};
    const SCS_32BIT_BINARY: u32 = 0;
    const SCS_64BIT_BINARY: u32 = 6;
    let host = guest_operation_path(name, false).map_err(|e| io_error(&e, ERROR_ACCESS_DENIED))?;
    let mut image = fs::File::open(host).map_err(|e| io_error(&e, ERROR_ACCESS_DENIED))?;
    let mut word = [0; 2];
    if image.read_exact(&mut word).is_err() || u16::from_le_bytes(word) != DOS_MAGIC {
        return Err(ERROR_BAD_EXE_FORMAT);
    }
    let mut dword = [0; 4];
    image
        .seek(SeekFrom::Start(u64::from(PE_POINTER_OFFSET)))
        .and_then(|_| image.read_exact(&mut dword))
        .map_err(|_| ERROR_BAD_EXE_FORMAT)?;
    let pe = u64::from(u32::from_le_bytes(dword));
    image
        .seek(SeekFrom::Start(pe))
        .and_then(|_| image.read_exact(&mut dword))
        .map_err(|_| ERROR_BAD_EXE_FORMAT)?;
    if u32::from_le_bytes(dword) != PE_MAGIC {
        return Ok(None);
    }
    let mut coff = [0; SIZEOF_COFF_HEADER];
    image
        .read_exact(&mut coff)
        .map_err(|_| ERROR_BAD_EXE_FORMAT)?;
    let offset = SIZEOF_COFF_HEADER - size_of::<u16>();
    if u16::from_le_bytes([coff[offset], coff[offset + 1]]) & IMAGE_FILE_DLL != 0 {
        return Err(ERROR_BAD_EXE_FORMAT);
    }
    image
        .read_exact(&mut word)
        .map_err(|_| ERROR_BAD_EXE_FORMAT)?;
    match u16::from_le_bytes(word) {
        MAGIC_32 => Ok(Some(SCS_32BIT_BINARY)),
        MAGIC_64 => Ok(Some(SCS_64BIT_BINARY)),
        _ => Err(ERROR_BAD_EXE_FORMAT),
    }
}

fn binary_type(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let input = api_try!(emu, call.arg(emu, 0));
    let out = api_try!(emu, call.arg(emu, 1));
    let result =
        absolute_guest_path(emu, input, api.ends_with('W')).and_then(|name| executable_kind(&name));
    log_red!(emu, "{library}!{api} => {result:?}");
    match result {
        Ok(None) => {
            policy::unsupported(emu, library, api, emu.pc());
            return;
        }
        Ok(Some(kind))
            if out != 0
                && emu.maps.validate_write_range(out, size_of::<u32>() as u64)
                && emu.maps.write_dword(out, kind) =>
        {
            call.set_return(emu, 1)
        }
        other => {
            emu.set_last_error(other.err().unwrap_or(ERROR_NOACCESS));
            call.set_return(emu, 0);
        }
    }
    call.cleanup_args(emu, 2);
}

fn needs_current_directory(emu: &mut Emu, library: &str, api: &str) {
    use crate::winapi::environment_store;
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    let name = file_names::read_path(emu, address, api.ends_with('W'));
    let key: Vec<_> = "NoDefaultCurrentDirectoryInExePath"
        .encode_utf16()
        .collect();
    let value = name.is_ok_and(|name| {
        name.contains('\\') || environment_store::process(emu).get(&key).is_none()
    });
    log_red!(emu, "{library}!{api} => {value}");
    call.set_return(emu, u64::from(value));
    call.cleanup_args(emu, 1);
}

fn search_path(emu: &mut Emu, library: &str, api: &str) {
    use crate::winapi::common::memory::{writable_pointer, write_pointer};
    let call = WinApi(Stdcall);
    let dirs = api_try!(emu, call.arg(emu, 0));
    let input = api_try!(emu, call.arg(emu, 1));
    let extension = api_try!(emu, call.arg(emu, 2));
    let size = api_try!(emu, call.arg_u32(emu, 3)) as usize;
    let out = api_try!(emu, call.arg(emu, 4));
    let part = api_try!(emu, call.arg(emu, 5));
    let wide = api.ends_with('W');
    let codec = file_names::encoding(emu, wide);
    let result = (|| {
        let mut name = file_names::read_path(emu, input, wide)?;
        if name.is_empty() {
            return Err(ERROR_FILE_NOT_FOUND);
        }
        if extension != 0
            && !name
                .rsplit(['\\', '/'])
                .next()
                .unwrap_or_default()
                .contains('.')
        {
            name.push_str(&file_names::read_path(emu, extension, wide)?);
        }
        let absolute = name.starts_with(['\\', '/']) || name.as_bytes().get(1) == Some(&b':');
        // A null list requests the process search order, which depends on SetSearchPathMode.
        if !absolute && dirs == 0 {
            return Ok(None);
        }
        let list = if absolute {
            String::new()
        } else {
            file_names::read_path(emu, dirs, wide)?
        };
        for dir in list.split(';') {
            let path = if dir.is_empty() {
                name.clone()
            } else {
                format!(r"{}\{name}", dir.trim_end_matches(['\\', '/']))
            };
            let full = file_names::absolute_path(&emu.cfg.cwd_path, path.encode_utf16().collect())?;
            let path = String::from_utf16(&full).map_err(|_| ERROR_INVALID_PARAMETER)?;
            if !guest_file_metadata(&path).is_ok_and(|m| m.is_file()) {
                continue;
            }
            let bytes = codec.terminated(&full);
            let needed = bytes.len() / codec.width();
            if size < needed {
                return Ok(Some(needed));
            }
            if out == 0
                || !emu.maps.validate_write_range(out, bytes.len() as u64)
                || part != 0 && !writable_pointer(emu, part)
            {
                return Err(ERROR_NOACCESS);
            }
            emu.maps.write_bytes(out, &bytes);
            if part != 0 {
                let index = full
                    .iter()
                    .rposition(|c| *c == u16::from(b'\\'))
                    .map_or(0, |i| i + 1);
                write_pointer(emu, part, out + codec.encode(&full[..index]).len() as u64);
            }
            return Ok(Some(needed - 1));
        }
        Err(ERROR_FILE_NOT_FOUND)
    })();
    log_red!(emu, "{library}!{api} => {result:?}");
    match result {
        Ok(Some(length)) => call.set_return(emu, length as u64),
        Ok(None) => {
            policy::unsupported(emu, library, api, emu.pc());
            return;
        }
        Err(error) => {
            emu.set_last_error(error);
            call.set_return(emu, 0);
        }
    }
    call.cleanup_args(emu, 6);
}

pub(super) fn dispatch(library: &str, api: &str, emu: &mut Emu) -> bool {
    match api {
        "GetShortPathNameA" | "GetShortPathNameW" | "GetLongPathNameA" | "GetLongPathNameW" => {
            existing_path(emu, library, api)
        }
        "GetCompressedFileSizeA" | "GetCompressedFileSizeW" => compressed_size(emu, library, api),
        "NeedCurrentDirectoryForExePathA" | "NeedCurrentDirectoryForExePathW" => {
            needs_current_directory(emu, library, api)
        }
        "SearchPathA" | "SearchPathW" => search_path(emu, library, api),
        "GetBinaryType" | "GetBinaryTypeA" | "GetBinaryTypeW" => binary_type(emu, library, api),
        "GetTempFileNameA" | "GetTempFileNameW" => temp_file(emu, library, api),
        "SetFileAttributesA" | "SetFileAttributesW" => attributes(emu, library, api),
        "SetFileAttributesFromAppW" if library == "kernelbase" => attributes(emu, library, api),
        _ => return false,
    }
    true
}
