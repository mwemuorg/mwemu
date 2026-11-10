use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::environment_store;
use crate::windows::constants::errors::*;
use crate::windows::constants::system::*;

// Environment APIs over environment_store. Sizes are in characters.

fn read_text(emu: &emu::Emu, ptr: u64, wide: bool) -> String {
    if wide {
        emu.maps.read_wide_string(ptr)
    } else {
        emu.maps.read_string(ptr)
    }
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
    if wide {
        emu.maps.write_wide_string(ptr, s);
    } else {
        emu.maps.write_string(ptr, s);
    }
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
    let name = read_text(emu, emu.regs().rcx, wide);
    let buffer = emu.regs().rdx;
    let size = emu.regs().r8 as u32 as usize;
    let value = environment_store::get(&emu.cfg, &name);
    log_red!(emu, "kernel32!{} `{}` = {:?}", api, name, value);
    match value {
        Some(value) => emu.regs_mut().rax = write_text(emu, buffer, size, &value, wide),
        None => {
            emu.set_last_error(ERROR_ENVVAR_NOT_FOUND);
            emu.regs_mut().rax = 0;
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
    let name = read_text(emu, emu.regs().rcx, wide);
    let value_ptr = emu.regs().rdx;
    let value = (value_ptr != 0).then(|| read_text(emu, value_ptr, wide));
    log_red!(emu, "kernel32!{} `{}` = {:?}", api, name, value);
    if name.is_empty() || name.contains('=') {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        emu.regs_mut().rax = FALSE;
        return;
    }
    environment_store::set(&emu.cfg, &name, value.as_deref());
    emu.regs_mut().rax = TRUE;
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
    let src = read_text(emu, emu.regs().rcx, wide);
    let dst = emu.regs().rdx;
    let size = emu.regs().r8 as u32 as usize;
    let expanded = environment_store::expand(&emu.cfg, &src);
    log_red!(emu, "kernel32!{} `{}` = `{}`", api, src, expanded);
    // the result counts the terminator whether or not it was copied
    let needed = char_len(&expanded, wide) + 1;
    if dst != 0 && needed <= size {
        write_text(emu, dst, size, &expanded, wide);
    }
    emu.regs_mut().rax = needed as u64;
}

/* LPCH GetEnvironmentStrings(); LPWCH GetEnvironmentStringsW(); */
pub fn GetEnvironmentStringsA(emu: &mut emu::Emu) {
    environment_block(emu, "GetEnvironmentStringsA", false);
}

pub fn GetEnvironmentStringsW(emu: &mut emu::Emu) {
    environment_block(emu, "GetEnvironmentStringsW", true);
}

fn environment_block(emu: &mut emu::Emu, api: &str, wide: bool) {
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
        emu.regs_mut().rax = 0;
        return;
    };
    let name = format!("environment_{:x}", block);
    if emu
        .maps
        .create_map(&name, block, bytes.len() as u64, Permission::READ_WRITE)
        .is_err()
    {
        emu.regs_mut().rax = 0;
        return;
    }
    emu.maps.write_bytes(block, &bytes);
    log_red!(emu, "kernel32!{} = 0x{:x}", api, block);
    emu.regs_mut().rax = block;
}

/* BOOL FreeEnvironmentStringsW(LPWCH penv); */
pub fn FreeEnvironmentStrings(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    log_red!(emu, "kernel32!FreeEnvironmentStrings 0x{:x}", ptr);
    emu.maps.dealloc(ptr);
    emu.regs_mut().rax = TRUE;
}
