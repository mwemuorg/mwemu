use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::{
    heap,
    memory::{writable_pointer, write_pointer},
};
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::{errors::*, nls::EN_US_LOCALE, system::*};

const MAX_MESSAGE_BYTES: u64 = 128 * 1024;

pub(crate) fn format_message(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let flags = api_try!(emu, call.arg_u32(emu, 0));
    let source = api_try!(emu, call.arg(emu, 1));
    let id = api_try!(emu, call.arg_u32(emu, 2));
    let language = api_try!(emu, call.arg_u32(emu, 3));
    let output = api_try!(emu, call.arg(emu, 4));
    let capacity = api_try!(emu, call.arg_u32(emu, 5));
    let arguments = api_try!(emu, call.arg(emu, 6));
    let codec = Encoding::from_wide(api.ends_with('W'));
    log_red!(
        emu,
        "{library}!{api} flags={flags:#x} source={source:#x} message_id={id} language={language:#x} buffer={output:#x} size={capacity} arguments={arguments:#x}"
    );
    let from = flags
        & (FORMAT_MESSAGE_FROM_STRING | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_FROM_HMODULE);
    let message = if output == 0
        || from == 0
        || (from & FORMAT_MESSAGE_FROM_STRING != 0 && from != FORMAT_MESSAGE_FROM_STRING)
    {
        Err(ERROR_INVALID_PARAMETER)
    } else if from & FORMAT_MESSAGE_FROM_HMODULE != 0 || flags & FORMAT_MESSAGE_MAX_WIDTH_MASK != 0
    {
        policy::unsupported(emu, library, api, emu.pc());
        return;
    } else if from == FORMAT_MESSAGE_FROM_STRING {
        codec.read(emu, source)
    } else if language != 0 && language != EN_US_LOCALE {
        Err(ERROR_RESOURCE_LANG_NOT_FOUND)
    } else if let Some(message) = system_message(u64::from(id)) {
        Ok(message.encode_utf16().collect())
    } else {
        policy::unsupported(emu, library, api, emu.pc());
        return;
    };
    let result = match message {
        Err(error) => Err(error),
        Ok(message) => {
            let Some(text) = literal_message(&message, flags & FORMAT_MESSAGE_IGNORE_INSERTS != 0)
            else {
                policy::unsupported(emu, library, api, emu.pc());
                return;
            };
            copy_message(
                emu,
                codec,
                output,
                capacity,
                flags & FORMAT_MESSAGE_ALLOCATE_BUFFER != 0,
                &text,
            )
        }
    };
    let length = result.unwrap_or_else(|error| {
        emu.set_last_error(error);
        0
    });
    call.set_return(emu, length);
    call.cleanup_args(emu, 7);
}

fn literal_message(message: &[u16], ignore_inserts: bool) -> Option<Vec<u16>> {
    let mut output = Vec::new();
    let mut units = message.iter().copied();
    while let Some(unit) = units.next() {
        if unit != u16::from(b'%') {
            output.push(unit);
            continue;
        }
        let escape = units.next()?;
        match u8::try_from(escape) {
            Ok(b'0') => break,
            Ok(b'1'..=b'9') if !ignore_inserts => return None,
            Ok(b'1'..=b'9') => output.extend([u16::from(b'%'), escape]),
            Ok(b'n') => output.extend([u16::from(b'\r'), u16::from(b'\n')]),
            Ok(b'r') => output.push(u16::from(b'\r')),
            Ok(b't') => output.push(u16::from(b'\t')),
            _ => output.push(escape),
        }
    }
    Some(output)
}

fn copy_message(
    emu: &mut Emu,
    codec: Encoding,
    output: u64,
    capacity: u32,
    allocate: bool,
    message: &[u16],
) -> Result<u64, u64> {
    let bytes = codec.terminated(message);
    let size = bytes.len() as u64;
    let width = codec.width() as u64;
    if size > MAX_MESSAGE_BYTES {
        return Err(ERROR_MORE_DATA);
    }
    let requested = u64::from(capacity) * width;
    let destination = if allocate {
        if !writable_pointer(emu, output) {
            return Err(ERROR_NOACCESS);
        }
        if requested > MAX_MESSAGE_BYTES {
            return Err(ERROR_NOT_ENOUGH_MEMORY);
        }
        heap::allocate(emu, 0, size.max(requested), 0).ok_or(ERROR_NOT_ENOUGH_MEMORY)?
    } else {
        if requested < size {
            return Err(ERROR_INSUFFICIENT_BUFFER);
        }
        output
    };
    if !emu.maps.validate_write_range(destination, size)
        || !emu.maps.write_bytes(destination, &bytes)
        || (allocate && !write_pointer(emu, output, destination))
    {
        if allocate {
            heap::heap_free(emu, 0, destination);
        }
        return Err(ERROR_NOACCESS);
    }
    Ok(size / width - 1)
}

fn system_message(code: u64) -> Option<&'static str> {
    Some(match code {
        ERROR_SUCCESS => "The operation completed successfully.\r\n",
        ERROR_INVALID_FUNCTION => "Incorrect function.\r\n",
        ERROR_FILE_NOT_FOUND => "The system cannot find the file specified.\r\n",
        ERROR_PATH_NOT_FOUND => "The system cannot find the path specified.\r\n",
        ERROR_TOO_MANY_OPEN_FILES => "The system cannot open the file.\r\n",
        ERROR_ACCESS_DENIED => "Access is denied.\r\n",
        ERROR_INVALID_HANDLE => "The handle is invalid.\r\n",
        ERROR_NOT_ENOUGH_MEMORY => {
            "Not enough memory resources are available to process this command.\r\n"
        }
        ERROR_OUTOFMEMORY => {
            "Not enough memory resources are available to complete this operation.\r\n"
        }
        ERROR_INVALID_DRIVE => "The system cannot find the drive specified.\r\n",
        ERROR_NO_MORE_FILES => "There are no more files.\r\n",
        ERROR_SHARING_VIOLATION => {
            "The process cannot access the file because it is being used by another process.\r\n"
        }
        ERROR_INVALID_PARAMETER => "The parameter is incorrect.\r\n",
        ERROR_BROKEN_PIPE => "The pipe has been ended.\r\n",
        ERROR_BUFFER_OVERFLOW => "The file name is too long.\r\n",
        ERROR_CALL_NOT_IMPLEMENTED => "This function is not supported on this system.\r\n",
        ERROR_INSUFFICIENT_BUFFER => "The data area passed to a system call is too small.\r\n",
        ERROR_MOD_NOT_FOUND => "The specified module could not be found.\r\n",
        ERROR_PROC_NOT_FOUND => "The specified procedure could not be found.\r\n",
        ERROR_ALREADY_EXISTS => "Cannot create a file when that file already exists.\r\n",
        ERROR_ENVVAR_NOT_FOUND => {
            "The system could not find the environment option that was entered.\r\n"
        }
        ERROR_NO_MORE_ITEMS => "No more data is available.\r\n",
        ERROR_DIRECTORY => "The directory name is invalid.\r\n",
        ERROR_ARITHMETIC_OVERFLOW => "Arithmetic result exceeded 32 bits.\r\n",
        ERROR_OPERATION_ABORTED => {
            "The I/O operation has been aborted because of either a thread exit or an application request.\r\n"
        }
        ERROR_IO_PENDING => "Overlapped I/O operation is in progress.\r\n",
        ERROR_NOACCESS => "Invalid access to memory location.\r\n",
        ERROR_NO_TOKEN => "An attempt was made to reference a token that does not exist.\r\n",
        ERROR_OLD_WIN_VERSION => "The specified program requires a newer version of Windows.\r\n",
        ERROR_NOT_FOUND => "Element not found.\r\n",
        _ => return None,
    })
}
