use crate::api::abi::windows::{ArgumentError, CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::ExceptionType;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::{descriptors, set_errno, with_stream};
use crate::winapi::common::memory::{pointer_size, read_pointer};
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::crt::*;

enum FormatError {
    Address,
    Argument(ArgumentError),
    Unsupported,
    Limit,
}

const MAX_FORMAT_BYTES: usize = 1 << 20;

fn string(emu: &Emu, start: u64, limit: usize) -> Result<Vec<u8>, FormatError> {
    let mut data = Vec::new();
    for i in 0..=limit {
        let address = start.checked_add(i as u64).ok_or(FormatError::Address)?;
        match emu.maps.read_byte(address).ok_or(FormatError::Address)? {
            0 => return Ok(data),
            _ if i == limit => return Err(FormatError::Limit),
            byte => data.push(byte),
        }
    }
    Err(FormatError::Limit)
}

enum Arguments {
    Stack(usize),
    List(u64),
}

impl Arguments {
    fn next(&mut self, emu: &Emu) -> Result<u64, FormatError> {
        match self {
            Self::Stack(index) => {
                let value = WinApi(Cdecl)
                    .arg(emu, *index)
                    .map_err(FormatError::Argument)?;
                *index += 1;
                Ok(value)
            }
            Self::List(cursor) => {
                let value = read_pointer(emu, *cursor).ok_or(FormatError::Address)?;
                *cursor = cursor
                    .checked_add(pointer_size(emu))
                    .ok_or(FormatError::Address)?;
                Ok(value)
            }
        }
    }
}

fn render(emu: &Emu, format: u64, mut args: Arguments) -> Result<Vec<u8>, FormatError> {
    let bytes = string(emu, format, MAX_FORMAT_BYTES)?;
    let mut out = Vec::new();
    let mut chars = bytes.into_iter();
    while let Some(byte) = chars.next() {
        if out.len() >= MAX_FORMAT_BYTES {
            return Err(FormatError::Limit);
        }
        if byte != b'%' {
            out.push(byte);
            continue;
        }
        let spec = chars.next().ok_or(FormatError::Unsupported)?;
        if spec == b'%' {
            out.push(b'%');
            continue;
        }
        if !matches!(spec, b'd' | b'i' | b'u' | b'x' | b'X' | b'o' | b'c' | b's') {
            return Err(FormatError::Unsupported);
        }
        let value = args.next(emu)?;
        let number = value as u32;
        match spec {
            b'd' | b'i' => out.extend((number as i32).to_string().bytes()),
            b'u' => out.extend(number.to_string().bytes()),
            b'x' => out.extend(format!("{number:x}").bytes()),
            b'X' => out.extend(format!("{number:X}").bytes()),
            b'o' => out.extend(format!("{number:o}").bytes()),
            b'c' => out.push(value as u8),
            b's' => out.extend(string(emu, value, MAX_FORMAT_BYTES - out.len())?),
            _ => return Err(FormatError::Unsupported),
        }
        if out.len() > MAX_FORMAT_BYTES {
            return Err(FormatError::Limit);
        }
    }
    Ok(out)
}

pub(crate) fn print(emu: &mut Emu, runtime: Runtime, api: &str) {
    let call = WinApi(Cdecl);
    let common = api == "__stdio_common_vfprintf";
    let explicit = api.ends_with("fprintf");
    let start = if common {
        if emu.cfg.arch.is_64bits() { 1 } else { 2 }
    } else {
        0
    };
    let stream = if explicit {
        Some(api_try!(emu, call.arg(emu, start)))
    } else {
        None
    };
    let fmt_index = start + usize::from(explicit);
    let fmt = api_try!(emu, call.arg(emu, fmt_index));
    if common {
        let locale = api_try!(emu, call.arg(emu, fmt_index + 1));
        let options = api_try!(emu, call.arg_u64(emu, 0));
        if options != 0 || locale != 0 {
            policy::unsupported_with_abi(emu, runtime.library(), api, emu.pc(), call);
            return;
        }
    }
    let first = fmt_index + 1 + usize::from(common);
    let args = if api == "fprintf" || api == "printf" {
        Arguments::Stack(first)
    } else {
        Arguments::List(api_try!(emu, call.arg(emu, first)))
    };
    let fd = match stream {
        Some(s) => with_stream(emu, runtime, s, |v| v.fd),
        None => Some(1),
    };
    log_red!(emu, "{}!{api} format={fmt:#x}", runtime.library());
    let result = if let Some(fd) = fd {
        match render(emu, fmt, args) {
            Ok(bytes) => descriptors::write_slice(emu, runtime, fd, &bytes)
                .and_then(|n| if n == bytes.len() { Ok(n) } else { Err(EIO) }),
            Err(FormatError::Argument(e)) => {
                policy::argument_fault(emu, e);
                return;
            }
            Err(FormatError::Address) => {
                emu.exception(ExceptionType::BadAddressDereferencing);
                return;
            }
            Err(FormatError::Limit | FormatError::Unsupported) => {
                policy::unsupported_with_abi(emu, runtime.library(), api, emu.pc(), call);
                return;
            }
        }
    } else {
        Err(EBADF)
    };
    match result {
        Ok(n) => call.set_return(emu, n as u64),
        Err(error) => {
            if let Some(s) = stream {
                with_stream(emu, runtime, s, |v| v.error = true);
            }
            set_errno(emu, runtime, error);
            call.set_return(emu, EOF);
        }
    }
}

pub(crate) fn buffer(emu: &mut Emu, runtime: Runtime, api: &str) {
    let call = WinApi(Cdecl);
    let destination = api_try!(emu, call.arg(emu, 0));
    let bounded = api.contains("snprintf");
    let legacy = api.starts_with('_');
    let capacity = if bounded {
        Some(api_try!(emu, call.arg(emu, 1)))
    } else {
        None
    };
    let format_index = 1 + usize::from(bounded);
    let format = api_try!(emu, call.arg(emu, format_index));
    let first = format_index + 1;
    let arguments = if api.contains('v') {
        Arguments::List(api_try!(emu, call.arg(emu, first)))
    } else {
        Arguments::Stack(first)
    };
    log_red!(
        emu,
        "{}!{api} destination={destination:#x} capacity={capacity:?} format={format:#x}",
        runtime.library()
    );
    let mut output = match render(emu, format, arguments) {
        Ok(data) => data,
        Err(FormatError::Argument(error)) => {
            policy::argument_fault(emu, error);
            return;
        }
        Err(FormatError::Address) => {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        }
        Err(FormatError::Limit | FormatError::Unsupported) => {
            policy::unsupported_with_abi(emu, runtime.library(), api, emu.pc(), call);
            return;
        }
    };
    let length = output.len();
    let capacity = capacity.map_or(usize::MAX, |c| usize::try_from(c).unwrap_or(usize::MAX));
    if capacity != 0 {
        if legacy && length >= capacity {
            output.truncate(capacity);
        } else {
            output.truncate(capacity - 1);
            output.push(0);
        }
        if destination == 0
            || !emu
                .maps
                .validate_write_range(destination, output.len() as u64)
            || !emu.maps.write_bytes(destination, &output)
        {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return;
        }
    }
    let result = if legacy && length > capacity {
        EOF
    } else {
        length as u64
    };
    call.set_return(emu, result);
}
