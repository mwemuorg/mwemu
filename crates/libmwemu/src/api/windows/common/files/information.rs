use std::{fs::File, io};

use super::{attribute_bits, attribute_data, file, file_names, io_error, read_array};
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;
use crate::windows::constants::files::*;

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
