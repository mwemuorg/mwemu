use std::{fs::File, io};

use super::{
    BYTES_PER_SECTOR, SECTORS_PER_CLUSTER, attribute_bits, attribute_data, file, file_names,
    io_error, read_array,
};
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::emu::object_handle::FileHandle;
use crate::winapi::common::memory::{pointer_size, write_pointer};
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;
use crate::windows::constants::files::*;
use crate::windows::structures::FileStandardInformation;

const BY_HANDLE_INFORMATION_SIZE: usize = 13 * size_of::<u32>();
const ATTRIBUTE_TAG_SIZE: usize = 2 * size_of::<u32>();

struct FileIdentity {
    volume: u32,
    links: u32,
    index: u64,
}

#[cfg(windows)]
fn identity(file: &File, _: &std::fs::Metadata) -> io::Result<FileIdentity> {
    let info = winapi_util::file::information(file)?;
    Ok(FileIdentity {
        volume: info.volume_serial_number() as u32,
        links: info.number_of_links() as u32,
        index: info.file_index(),
    })
}

#[cfg(unix)]
fn identity(_: &File, metadata: &std::fs::Metadata) -> io::Result<FileIdentity> {
    use std::os::unix::fs::MetadataExt;
    Ok(FileIdentity {
        volume: metadata.dev() as u32,
        links: u32::try_from(metadata.nlink()).map_err(|_| io::ErrorKind::InvalidData)?,
        index: metadata.ino(),
    })
}

#[cfg(not(any(windows, unix)))]
fn identity(_: &File, _: &std::fs::Metadata) -> io::Result<FileIdentity> {
    Err(io::ErrorKind::Unsupported.into())
}

fn standard_information(guest: &FileHandle) -> io::Result<FileStandardInformation> {
    let links = match guest.open_file() {
        Ok(file) => identity(file, &file.metadata()?)?.links,
        // A file supplied by the guest has one directory entry and no host metadata.
        Err(error) if error.kind() == io::ErrorKind::Unsupported => 1,
        Err(error) => return Err(error),
    };
    let length = if guest.is_dir() { 0 } else { guest.length()? };
    let cluster = u64::from(SECTORS_PER_CLUSTER) * u64::from(BYTES_PER_SECTOR);
    let allocation = length
        .checked_next_multiple_of(cluster)
        .ok_or(io::ErrorKind::InvalidData)?;
    Ok(FileStandardInformation {
        allocation_size: allocation,
        end_of_file: length,
        number_of_links: links,
        delete_pending: u8::from(links == 0),
        directory: u8::from(guest.is_dir()),
    })
}

pub(crate) fn native_query_file(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [handle, io, output, capacity, class] = api_try!(emu, call.args(emu));
    let class = class as u32;
    log_red!(
        emu,
        "ntdll!NtQueryInformationFile handle={handle:#x} class={class} capacity={}",
        capacity as u32
    );
    if class != FILE_STANDARD_INFORMATION {
        policy::unsupported(emu, "ntdll", "NtQueryInformationFile", emu.pc());
        return;
    }
    let size = FileStandardInformation::size();
    let width = pointer_size(emu);
    let status = if (capacity as u32) < size {
        STATUS_INFO_LENGTH_MISMATCH
    } else if io == 0
        || !emu.maps.validate_write_range(io, width * 2)
        || output == 0
        || !emu.maps.validate_write_range(output, u64::from(size))
    {
        STATUS_ACCESS_VIOLATION
    } else if let Ok(guest) = file(emu, handle) {
        match standard_information(guest) {
            Ok(info) => {
                emu.maps.write_bytes(output, &info.to_bytes());
                write_pointer(emu, io, STATUS_SUCCESS);
                write_pointer(emu, io + width, u64::from(size));
                STATUS_SUCCESS
            }
            Err(_) => STATUS_UNSUCCESSFUL,
        }
    } else {
        STATUS_INVALID_HANDLE
    };
    call.set_return(emu, status);
    call.cleanup_args(emu, 5);
}

fn information(file: &File, extended: bool) -> io::Result<Option<Vec<u8>>> {
    let metadata = file.metadata()?;
    let attributes = attribute_bits(&metadata);
    if extended {
        if attributes & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return Ok(None);
        }
        let mut output = attributes.to_le_bytes().to_vec();
        output.extend_from_slice(&0u32.to_le_bytes());
        return Ok(Some(output));
    }
    let id = identity(file, &metadata)?;
    let mut output = attribute_data(&metadata);
    // The classic handle layout inserts the volume serial before the two size DWORDs.
    let size_offset = size_of::<u32>() + 3 * size_of::<u64>();
    output.splice(size_offset..size_offset, id.volume.to_le_bytes());
    for word in [id.links, (id.index >> u32::BITS) as u32, id.index as u32] {
        output.extend_from_slice(&word.to_le_bytes());
    }
    Ok(Some(output))
}

pub(super) fn dispatch(library: &str, api: &str, emu: &mut Emu) -> bool {
    let extended = match api {
        "GetFinalPathNameByHandleA" | "GetFinalPathNameByHandleW" => {
            final_path(emu, library, api);
            return true;
        }
        "SetFileInformationByHandle" => {
            set_information(emu, library, api);
            return true;
        }
        "GetFileInformationByHandle" => false,
        "GetFileInformationByHandleEx" => true,
        _ => return false,
    };
    query(emu, library, api, extended);
    true
}

fn query(emu: &mut Emu, library: &str, api: &str, extended: bool) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    if extended {
        let class = api_try!(emu, call.arg_u32(emu, 1));
        if class != FILE_ATTRIBUTE_TAG_INFO_CLASS {
            log_red!(emu, "{library}!{api} unsupported information class {class}");
            policy::unsupported(emu, library, api, emu.pc());
            return;
        }
    }
    let pointer = api_try!(emu, call.arg(emu, if extended { 2 } else { 1 }));
    let size = if extended {
        ATTRIBUTE_TAG_SIZE
    } else {
        BY_HANDLE_INFORMATION_SIZE
    };
    let capacity = if extended {
        api_try!(emu, call.arg_u32(emu, 3)) as usize
    } else {
        size
    };
    let result = if capacity < size {
        Err(ERROR_BAD_LENGTH)
    } else if pointer == 0 || !emu.maps.validate_write_range(pointer, size as u64) {
        Err(ERROR_NOACCESS)
    } else {
        file(emu, handle).and_then(|guest| {
            guest
                .open_file()
                .and_then(|file| information(file, extended))
                .map_err(|error| io_error(&error, ERROR_READ_FAULT))
        })
    };
    log_red!(emu, "{library}!{api} handle={handle:#x}");
    match result {
        Ok(None) => {
            policy::unsupported(emu, library, api, emu.pc());
            return;
        }
        Ok(Some(bytes)) if emu.maps.write_bytes(pointer, &bytes) => call.set_return(emu, 1),
        result => {
            emu.set_last_error(result.err().unwrap_or(ERROR_NOACCESS));
            call.set_return(emu, 0);
        }
    }
    call.cleanup_args(emu, if extended { 4 } else { 2 });
}

fn final_path(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let capacity = api_try!(emu, call.arg_u32(emu, 2)) as usize;
    let flags = api_try!(emu, call.arg_u32(emu, 3));
    let codec = file_names::encoding(emu, api.ends_with('W'));
    let result = file(emu, handle)
        .and_then(|file| {
            let name = file.guest_name();
            let name = name.strip_prefix(r"\\?\").unwrap_or(name);
            let (drive, tail) = name.split_once(':').ok_or(ERROR_PATH_NOT_FOUND)?;
            let path = match flags & !FILE_NAME_OPENED {
                0 => format!(r"\\?\{name}"),
                VOLUME_NAME_NONE => tail.to_owned(),
                VOLUME_NAME_NT if drive.eq_ignore_ascii_case("c") => {
                    format!("{SYSTEM_VOLUME_DEVICE}{tail}")
                }
                VOLUME_NAME_GUID | VOLUME_NAME_NT => return Err(ERROR_PATH_NOT_FOUND),
                _ => return Err(ERROR_INVALID_PARAMETER),
            };
            Ok(codec.terminated(&path.encode_utf16().collect::<Vec<_>>()))
        })
        .and_then(|bytes| {
            let needed = bytes.len() / codec.width();
            if capacity < needed {
                Ok(needed as u64)
            } else if output != 0
                && emu.maps.validate_write_range(output, bytes.len() as u64)
                && emu.maps.write_bytes(output, &bytes)
            {
                Ok((needed - 1) as u64)
            } else {
                Err(ERROR_NOACCESS)
            }
        });
    log_red!(
        emu,
        "{library}!{api} handle={handle:#x} flags={flags:#x} => {result:?}"
    );
    let length = result.unwrap_or_else(|error| {
        emu.set_last_error(error);
        0
    });
    call.set_return(emu, length);
    call.cleanup_args(emu, 4);
}

fn set_information(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let class = api_try!(emu, call.arg_u32(emu, 1));
    if class != FILE_END_OF_FILE_INFO_CLASS {
        policy::unsupported(emu, library, api, emu.pc());
        return;
    }
    let buffer = api_try!(emu, call.arg(emu, 2));
    let size = api_try!(emu, call.arg_u32(emu, 3));
    let result = (|| {
        if size < size_of::<i64>() as u32 {
            return Err(ERROR_BAD_LENGTH);
        }
        let bytes = read_array(emu, buffer)
            .filter(|_| buffer != 0)
            .ok_or(ERROR_NOACCESS)?;
        let length =
            u64::try_from(i64::from_le_bytes(bytes)).map_err(|_| ERROR_INVALID_PARAMETER)?;
        file(emu, handle)?
            .set_length(length)
            .map_err(|error| io_error(&error, ERROR_WRITE_FAULT))
    })();
    log_red!(
        emu,
        "{library}!{api} handle={handle:#x} class={class} => {result:?}"
    );
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, u64::from(result.is_ok()));
    call.cleanup_args(emu, 4);
}
