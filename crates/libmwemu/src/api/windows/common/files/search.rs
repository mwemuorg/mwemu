use super::{absolute_guest_path, attribute_bits, file_names, io_error, to_filetime};
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::emu::object_handle::{FoundFile, file_handle::guest_directory_entries};
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::{errors::*, files::MAX_PATH, process::*};
use std::collections::VecDeque;

const FIND_EX_INFO_BASIC: u32 = 1;
const FIND_EX_SEARCH_LIMIT_TO_DIRECTORIES: u32 = 1;
const FIND_FIRST_EX_CASE_SENSITIVE: u32 = 1;
const FIND_FIRST_EX_LARGE_FETCH: u32 = 2;
const FIND_DATA_NAME_OFFSET: usize = 11 * size_of::<u32>();
const ALTERNATE_NAME_LENGTH: usize = 14;

fn matches(pattern: &str, name: &str, sensitive: bool) -> bool {
    let pattern = if pattern == "*.*" { "*" } else { pattern };
    let (pattern, name) = if sensitive {
        (pattern.to_owned(), name.to_owned())
    } else {
        (pattern.to_uppercase(), name.to_uppercase())
    };
    let pattern: Vec<_> = pattern.chars().collect();
    let name: Vec<_> = name.chars().collect();
    let (mut p, mut n, mut star) = (0, 0, None);
    while n < name.len() {
        match pattern.get(p) {
            Some('*') => {
                p += 1;
                star = Some((p, n));
            }
            Some(c) if *c == '?' || *c == name[n] => {
                p += 1;
                n += 1;
            }
            _ => match star {
                Some((after, prev)) => {
                    p = after;
                    n = prev + 1;
                    star = Some((p, n));
                }
                None => return false,
            },
        }
    }
    pattern[p..].iter().all(|c| *c == '*')
}

fn find_entries(
    path: &str,
    directories: bool,
    sensitive: bool,
) -> Result<VecDeque<FoundFile>, u64> {
    let (directory, pattern) = path.rsplit_once('\\').ok_or(ERROR_PATH_NOT_FOUND)?;
    if pattern.is_empty() {
        return Err(ERROR_FILE_NOT_FOUND);
    }
    let entries = guest_directory_entries(&format!("{directory}\\"))
        .map_err(|e| io_error(&e, ERROR_ACCESS_DENIED))?;
    Ok(entries
        .into_iter()
        .filter(|(name, metadata)| {
            (!directories || metadata.is_dir()) && matches(pattern, name, sensitive)
        })
        .map(|(name, metadata)| FoundFile {
            name,
            attributes: attribute_bits(&metadata),
            size: if metadata.is_dir() { 0 } else { metadata.len() },
            times: [metadata.created(), metadata.accessed(), metadata.modified()]
                .map(|t| t.map(to_filetime).unwrap_or(0)),
        })
        .collect())
}

fn write_entry(emu: &mut Emu, output: u64, wide: bool, entry: &FoundFile) -> Result<(), u64> {
    let codec = file_names::encoding(emu, wide);
    let name = codec.terminated(&entry.name.encode_utf16().collect::<Vec<_>>());
    if name.len() > MAX_PATH * codec.width() {
        return Err(ERROR_INSUFFICIENT_BUFFER);
    }
    let size = (FIND_DATA_NAME_OFFSET + (MAX_PATH + ALTERNATE_NAME_LENGTH) * codec.width())
        .next_multiple_of(size_of::<u32>());
    if output == 0 || !emu.maps.validate_write_range(output, size as u64) {
        return Err(ERROR_NOACCESS);
    }
    let mut bytes = entry.attributes.to_le_bytes().to_vec();
    for time in entry.times {
        bytes.extend(time.to_le_bytes());
    }
    bytes.extend(((entry.size >> u32::BITS) as u32).to_le_bytes());
    bytes.extend((entry.size as u32).to_le_bytes());
    bytes.resize(FIND_DATA_NAME_OFFSET, 0);
    bytes.extend(name);
    bytes.resize(size, 0);
    emu.maps
        .write_bytes(output, &bytes)
        .then_some(())
        .ok_or(ERROR_NOACCESS)
}

fn first(emu: &mut Emu, library: &str, api: &str, extended: bool) {
    let call = WinApi(Stdcall);
    let path = api_try!(emu, call.arg(emu, 0));
    let wide = api.ends_with('W');
    let (output, level, mode, filter, flags) = if extended {
        (
            api_try!(emu, call.arg(emu, 2)),
            api_try!(emu, call.arg_u32(emu, 1)),
            api_try!(emu, call.arg_u32(emu, 3)),
            api_try!(emu, call.arg(emu, 4)),
            api_try!(emu, call.arg_u32(emu, 5)),
        )
    } else {
        (api_try!(emu, call.arg(emu, 1)), 0, 0, 0, 0)
    };
    if mode > FIND_EX_SEARCH_LIMIT_TO_DIRECTORIES
        || flags & !(FIND_FIRST_EX_CASE_SENSITIVE | FIND_FIRST_EX_LARGE_FETCH) != 0
    {
        policy::unsupported(emu, library, api, emu.pc());
        return;
    }
    let result = (|| {
        if level > FIND_EX_INFO_BASIC || filter != 0 {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let path = absolute_guest_path(emu, path, wide)?;
        let mut entries = find_entries(
            &path,
            mode == FIND_EX_SEARCH_LIMIT_TO_DIRECTORIES,
            flags & FIND_FIRST_EX_CASE_SENSITIVE != 0,
        )?;
        write_entry(
            emu,
            output,
            wide,
            &entries.pop_front().ok_or(ERROR_FILE_NOT_FOUND)?,
        )?;
        Ok(u64::from(emu.handle_management.insert_file_search(entries)))
    })();
    log_red!(emu, "{library}!{api} => {result:?}");
    let handle = result.unwrap_or_else(|error| {
        emu.set_last_error(error);
        if emu.cfg.arch.is_64bits() {
            INVALID_HANDLE_VALUE_64
        } else {
            INVALID_HANDLE_VALUE_32
        }
    });
    call.set_return(emu, handle);
    call.cleanup_args(emu, if extended { 6 } else { 2 });
}

fn next(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let result = (|| {
        let entries = emu
            .handle_management
            .file_search(handle)
            .ok_or(ERROR_INVALID_HANDLE)?;
        let entry = entries.front().ok_or(ERROR_NO_MORE_FILES)?.clone();
        write_entry(emu, output, api.ends_with('W'), &entry)?;
        if let Some(entries) = emu.handle_management.file_search(handle) {
            entries.pop_front();
        }
        Ok::<_, u64>(())
    })();
    log_red!(emu, "{library}!{api} handle={handle:#x} => {result:?}");
    if let Err(e) = result {
        emu.set_last_error(e);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 2);
}

fn close(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let closed = emu.handle_management.close_file_search(handle);
    log_red!(emu, "{library}!{api} handle={handle:#x} => {closed}");
    if !closed {
        emu.set_last_error(ERROR_INVALID_HANDLE);
    }
    call.set_return(emu, u64::from(closed));
    call.cleanup_args(emu, 1);
}

pub(super) fn dispatch(library: &str, api: &str, emu: &mut Emu) -> bool {
    match api {
        "FindFirstFileA" | "FindFirstFileW" => first(emu, library, api, false),
        "FindFirstFileExA" | "FindFirstFileExW" => first(emu, library, api, true),
        "FindFirstFileExFromAppW" if library == "kernelbase" => first(emu, library, api, true),
        "FindNextFileA" | "FindNextFileW" => next(emu, library, api),
        "FindClose" => close(emu, library, api),
        _ => return false,
    }
    true
}
