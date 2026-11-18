use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::nls::*;

pub fn LCMapStringA(emu: &mut Emu) {
    invoke(emu, false, "LCMapStringA");
}

pub fn LCMapStringW(emu: &mut Emu) {
    invoke(emu, true, "LCMapStringW");
}

fn invoke(emu: &mut Emu, wide: bool, api: &str) {
    let call = WinApi(Stdcall);
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let source = api_try!(emu, call.arg(emu, 2));
    let source_length = api_try!(emu, call.arg_i32(emu, 3));
    let output = api_try!(emu, call.arg(emu, 4));
    let capacity = api_try!(emu, call.arg_i32(emu, 5));
    log_red!(
        emu,
        "kernel32!{} flags:0x{:x} source:0x{:x} length:{} capacity:{}",
        api,
        flags,
        source,
        source_length,
        capacity
    );
    let result = map_string(emu, flags, source, source_length, output, capacity, wide);
    let return_value = match result {
        Ok(count) => count,
        Err(error) => {
            emu.set_last_error(error);
            0
        }
    };
    call.set_return(emu, return_value);
}

fn map_string(
    emu: &mut Emu,
    flags: u32,
    source: u64,
    source_length: i32,
    output: u64,
    capacity: i32,
    wide: bool,
) -> Result<u64, u64> {
    if !matches!(flags, 0 | LCMAP_LOWERCASE | LCMAP_UPPERCASE) {
        return Err(ERROR_INVALID_FLAGS);
    }
    if source == 0 || source_length == 0 || source_length < -1 || capacity < 0 {
        return Err(ERROR_INVALID_PARAMETER);
    }
    let width = if wide {
        size_of::<u16>()
    } else {
        size_of::<u8>()
    };
    let mut bytes = if source_length == -1 {
        strings::encode_text(
            &format!("{}\0", strings::read_text(emu, source, wide)),
            wide,
        )
    } else {
        emu.maps
            .try_read_bytes(source, source_length as usize * width)
            .ok_or(ERROR_NOACCESS)?
            .into_owned()
    };
    if flags != 0 {
        if wide {
            #[allow(unknown_lints, clippy::chunks_exact_to_as_chunks)]
            let words: Vec<u16> = bytes
                .chunks_exact(size_of::<u16>())
                .map(|b| u16::from_le_bytes([b[0], b[1]]))
                .collect();
            let text = String::from_utf16_lossy(&words);
            let mapped = if flags == LCMAP_LOWERCASE {
                text.to_lowercase()
            } else {
                text.to_uppercase()
            };
            bytes = strings::encode_text(&mapped, true);
        } else if flags == LCMAP_LOWERCASE {
            bytes.make_ascii_lowercase();
        } else {
            bytes.make_ascii_uppercase();
        }
    }
    let count = bytes.len() / width;
    if capacity == 0 {
        return Ok(count as u64);
    }
    if output == 0 {
        return Err(ERROR_INVALID_PARAMETER);
    }
    if (capacity as usize) < count {
        return Err(ERROR_INSUFFICIENT_BUFFER);
    }
    if !emu.maps.validate_write_range(output, bytes.len() as u64)
        || !emu.maps.write_bytes(output, &bytes)
    {
        return Err(ERROR_NOACCESS);
    }
    Ok(count as u64)
}
