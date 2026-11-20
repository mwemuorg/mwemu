use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory::{writable_pointer, write_pointer};
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn AreFileApisANSI(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "AreFileApisANSI");
    call.set_return(emu, u64::from(!emu.oem_file_names));
}

pub fn SetFileApisToANSI(emu: &mut Emu) {
    log_red!(emu, "SetFileApisToANSI");
    emu.oem_file_names = false;
}

pub fn SetFileApisToOEM(emu: &mut Emu) {
    log_red!(emu, "SetFileApisToOEM");
    emu.oem_file_names = true;
}

pub(crate) fn encoding(emu: &Emu, wide: bool) -> Encoding {
    if !wide && emu.oem_file_names {
        Encoding::Oem
    } else {
        Encoding::from_wide(wide)
    }
}

pub(crate) fn read_path(emu: &Emu, address: u64, wide: bool) -> Result<String, u64> {
    encoding(emu, wide)
        .read(emu, address)
        .and_then(|path| String::from_utf16(&path).map_err(|_| ERROR_INVALID_PARAMETER))
}

const SEP: u16 = b'\\' as u16;
const DOT: u16 = b'.' as u16;
const COLON: u16 = b':' as u16;

fn drive(path: &[u16]) -> Option<u8> {
    match path {
        [letter, COLON, ..] => u8::try_from(*letter).ok().filter(u8::is_ascii_alphabetic),
        _ => None,
    }
}

fn volume(path: &[u16]) -> &[u16] {
    if path.starts_with(&[SEP, SEP]) {
        let end = path
            .iter()
            .enumerate()
            .skip(2)
            .filter(|(_, c)| **c == SEP)
            .nth(1);
        &path[..end.map_or(path.len(), |(i, _)| i)]
    } else if drive(path).is_some() {
        &path[..2]
    } else {
        &[]
    }
}

pub(crate) fn absolute_path(cwd: &str, mut path: Vec<u16>) -> Result<Vec<u16>, u64> {
    if path.is_empty() {
        return Err(ERROR_INVALID_PARAMETER);
    }
    if path.starts_with(&[SEP, SEP, b'?' as u16, SEP]) || path.starts_with(&[SEP, SEP, DOT, SEP]) {
        return Ok(path);
    }
    for c in &mut path {
        if *c == b'/' as u16 {
            *c = SEP;
        }
    }
    let cwd: Vec<u16> = cwd.encode_utf16().collect();
    let scope = volume(&path);
    let (mut base, rest) = if path.starts_with(&[SEP, SEP]) {
        (scope.to_vec(), &path[scope.len()..])
    } else if let Some(letter) = drive(&path) {
        let tail = &path[2..];
        if !tail.starts_with(&[SEP]) && drive(&cwd).is_some_and(|c| c.eq_ignore_ascii_case(&letter))
        {
            (cwd, tail)
        } else {
            (scope.to_vec(), tail)
        }
    } else if path.starts_with(&[SEP]) {
        (volume(&cwd).to_vec(), path.as_slice())
    } else {
        (cwd, path.as_slice())
    };
    if !base.ends_with(&[SEP]) {
        base.push(SEP);
    }
    base.extend_from_slice(rest);
    let root = volume(&base);
    let mut components = Vec::new();
    for part in base[root.len()..].split(|c| *c == SEP) {
        match part {
            [] | [DOT] => {}
            [DOT, DOT] => {
                components.pop();
            }
            _ => components.push(part),
        }
    }
    let mut full = root.to_vec();
    full.push(SEP);
    for (i, part) in components.iter().enumerate() {
        if i != 0 {
            full.push(SEP);
        }
        full.extend_from_slice(part);
    }
    if path.ends_with(&[SEP]) && !full.ends_with(&[SEP]) {
        full.push(SEP);
    }
    Ok(full)
}

fn get_full_path(emu: &mut Emu, wide: bool) {
    let call = WinApi(Stdcall);
    let input = api_try!(emu, call.arg(emu, 0));
    let capacity = api_try!(emu, call.arg_u32(emu, 1)) as usize;
    let buffer = api_try!(emu, call.arg(emu, 2));
    let part = api_try!(emu, call.arg(emu, 3));
    let codec = encoding(emu, wide);
    let result = codec.read(emu, input).and_then(|path| {
        let full = absolute_path(&emu.cfg.cwd_path, path)?;
        let bytes = codec.terminated(&full);
        let needed = bytes.len() / codec.width();
        if capacity < needed {
            return Ok(needed);
        }
        if buffer == 0
            || !emu.maps.validate_write_range(buffer, bytes.len() as u64)
            || (part != 0 && !writable_pointer(emu, part))
        {
            return Err(ERROR_NOACCESS);
        }
        if !emu.maps.write_bytes(buffer, &bytes) {
            return Err(ERROR_NOACCESS);
        }
        if part != 0 {
            let start = full.iter().rposition(|c| *c == SEP).map_or(0, |i| i + 1);
            let ptr = if start == full.len() {
                0
            } else {
                buffer + codec.encode(&full[..start]).len() as u64
            };
            write_pointer(emu, part, ptr);
        }
        Ok(needed - 1)
    });
    log_red!(
        emu,
        "GetFullPathName{} => {:?}",
        if wide { "W" } else { "A" },
        result
    );
    let length = result.unwrap_or_else(|error| {
        emu.set_last_error(error);
        0
    });
    call.set_return(emu, length as u64);
    call.cleanup_args(emu, 4);
}

pub fn GetFullPathNameA(emu: &mut Emu) {
    get_full_path(emu, false);
}

pub fn GetFullPathNameW(emu: &mut Emu) {
    get_full_path(emu, true);
}
