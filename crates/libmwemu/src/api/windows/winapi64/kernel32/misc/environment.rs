use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::strings;
use crate::winapi::environment_store;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::system::*;

// Environment APIs over environment_store. Sizes are in characters.

fn read_text(emu: &emu::Emu, ptr: u64, wide: bool) -> String {
    strings::read_text(emu, ptr, wide)
}

fn char_len(s: &str, wide: bool) -> usize {
    if wide {
        s.encode_utf16().count()
    } else {
        s.len()
    }
}

/// Copy `s` into a buffer of `size` characters. Returns the length written
/// (without the terminator), or the size needed (with it) when it does not fit.
fn write_text(emu: &mut emu::Emu, ptr: u64, size: usize, s: &str, wide: bool) -> u64 {
    let len = char_len(s, wide);
    if ptr == 0 || len + 1 > size {
        return (len + 1) as u64;
    }
    strings::write_string(emu, ptr, s, wide);
    len as u64
}

/*
DWORD GetEnvironmentVariableW(
  [in, optional]  LPCWSTR lpName,
  [out, optional] LPWSTR  lpBuffer,
  [in]            DWORD   nSize
);
*/
pub fn GetEnvironmentVariableA(emu: &mut emu::Emu) {
    get_variable(emu, "GetEnvironmentVariableA", false);
}

pub fn GetEnvironmentVariableW(emu: &mut emu::Emu) {
    get_variable(emu, "GetEnvironmentVariableW", true);
}

fn get_variable(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let name = read_text(emu, api_try!(emu, call.arg(emu, 0)), wide);
    let buffer = api_try!(emu, call.arg(emu, 1));
    let size = api_try!(emu, call.arg_u32(emu, 2)) as usize;
    let value = environment_store::get(&emu.cfg, &name);
    log_red!(emu, "kernel32!{} `{}` = {:?}", api, name, value);
    match value {
        Some(value) => {
            let return_value = write_text(emu, buffer, size, &value, wide);
            call.set_return(emu, return_value);
        }
        None => {
            emu.set_last_error(ERROR_ENVVAR_NOT_FOUND);
            call.set_return(emu, 0);
        }
    }
}

/*
BOOL SetEnvironmentVariableW(
  [in]           LPCWSTR lpName,
  [in, optional] LPCWSTR lpValue
);
*/
pub fn SetEnvironmentVariableA(emu: &mut emu::Emu) {
    set_variable(emu, "SetEnvironmentVariableA", false);
}

pub fn SetEnvironmentVariableW(emu: &mut emu::Emu) {
    set_variable(emu, "SetEnvironmentVariableW", true);
}

fn set_variable(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let name = read_text(emu, api_try!(emu, call.arg(emu, 0)), wide);
    let value_ptr = api_try!(emu, call.arg(emu, 1));
    let value = (value_ptr != 0).then(|| read_text(emu, value_ptr, wide));
    log_red!(emu, "kernel32!{} `{}` = {:?}", api, name, value);
    if name.is_empty() || name.contains('=') {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        call.set_return(emu, FALSE);
        return;
    }
    environment_store::set(&emu.cfg, &name, value.as_deref());
    call.set_return(emu, TRUE);
}

/*
DWORD ExpandEnvironmentStringsW(
  [in]            LPCWSTR lpSrc,
  [out, optional] LPWSTR  lpDst,
  [in]            DWORD   nSize
);
*/
pub fn ExpandEnvironmentStringsA(emu: &mut emu::Emu) {
    expand(emu, "ExpandEnvironmentStringsA", false);
}

pub fn ExpandEnvironmentStringsW(emu: &mut emu::Emu) {
    expand(emu, "ExpandEnvironmentStringsW", true);
}

fn expand(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let src = read_text(emu, api_try!(emu, call.arg(emu, 0)), wide);
    let dst = api_try!(emu, call.arg(emu, 1));
    let size = api_try!(emu, call.arg_u32(emu, 2)) as usize;
    let expanded = environment_store::expand(&emu.cfg, &src);
    log_red!(emu, "kernel32!{} `{}` = `{}`", api, src, expanded);
    // the result counts the terminator whether or not it was copied
    let needed = char_len(&expanded, wide) + 1;
    if dst != 0 && needed <= size {
        write_text(emu, dst, size, &expanded, wide);
    }
    call.set_return(emu, needed as u64);
}

/* LPCH GetEnvironmentStrings(); LPWCH GetEnvironmentStringsW(); */
pub fn GetEnvironmentStringsA(emu: &mut emu::Emu) {
    environment_block(emu, "GetEnvironmentStringsA", false);
}

pub fn GetEnvironmentStringsW(emu: &mut emu::Emu) {
    environment_block(emu, "GetEnvironmentStringsW", true);
}

fn environment_block(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let mut text = String::new();
    for (name, value) in environment_store::all(&emu.cfg) {
        text.push_str(&format!("{}={}\0", name, value));
    }
    text.push('\0');
    let bytes: Vec<u8> = if wide {
        text.encode_utf16().flat_map(|c| c.to_le_bytes()).collect()
    } else {
        text.into_bytes()
    };
    let Some(block) = emu.maps.alloc(bytes.len() as u64) else {
        emu.set_last_error(ERROR_NOT_ENOUGH_MEMORY);
        call.set_return(emu, 0);
        return;
    };
    let name = format!("environment_{:x}", block);
    if emu
        .maps
        .create_map(&name, block, bytes.len() as u64, Permission::READ_WRITE)
        .is_err()
    {
        call.set_return(emu, 0);
        return;
    }
    emu.maps.write_bytes(block, &bytes);
    log_red!(emu, "kernel32!{} = 0x{:x}", api, block);
    call.set_return(emu, block);
}

/* BOOL FreeEnvironmentStringsW(LPWCH penv); */
pub fn FreeEnvironmentStrings(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernel32!FreeEnvironmentStrings 0x{:x}", ptr);
    emu.maps.dealloc(ptr);
    call.set_return(emu, TRUE);
}
