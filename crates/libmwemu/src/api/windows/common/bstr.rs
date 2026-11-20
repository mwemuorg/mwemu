use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::maps::buffers::BufferKind;
use crate::maps::mem64::Permission;
use crate::winapi::common::trace::input_string;
use crate::winapi::common::{buffers, memory};
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::api_try;

const PREFIX: u64 = 4;
const TERMINATOR: u64 = 2;

fn string_bytes(emu: &Emu, source: u64) -> Option<u64> {
    if source == 0 {
        return Some(0);
    }
    let mut count = 0;
    while count < emu.maps.max_alloc_size {
        if emu.maps.read_word(source.checked_add(count)?)? == 0 {
            return Some(count);
        }
        count += 2;
    }
    None
}

pub(crate) fn allocate(emu: &mut Emu, source: u64, bytes: u64) -> Option<u64> {
    let length = u32::try_from(bytes).ok()?;
    if source != 0
        && !emu
            .maps
            .can_access_range(source, bytes as usize, Permission::READ)
    {
        return None;
    }
    let address = buffers::allocate(
        emu,
        bytes.checked_add(TERMINATOR)?,
        8,
        PREFIX,
        BufferKind::Bstr,
    )?;
    emu.maps.write_dword(address - PREFIX, length);
    if source != 0 {
        emu.maps.copy_range(address, source, bytes as usize);
    } else {
        emu.maps.fill_range(address, 0, bytes as usize);
    }
    emu.maps.write_word(address + bytes, 0);
    Some(address)
}

fn alloc_api(emu: &mut Emu, api: &str, explicit_length: bool, byte_length: bool) {
    let call = WinApi(Stdcall);
    let source = api_try!(emu, call.arg(emu, 0));
    let bytes = if explicit_length {
        let count = api_try!(emu, call.arg_u32(emu, 1)) as u64;
        count.checked_mul(if byte_length { 1 } else { 2 })
    } else {
        string_bytes(emu, source)
    };
    if explicit_length {
        log_red!(emu, "oleaut32!{api} source={source:#x} bytes={bytes:?}");
    } else {
        log_red!(
            emu,
            "oleaut32!{api} source={}",
            input_string(emu, source, Encoding::Wide)
        );
    }
    let value = if !explicit_length && source == 0 {
        0
    } else {
        bytes.and_then(|n| allocate(emu, source, n)).unwrap_or(0)
    };
    call.set_return(emu, value);
    call.cleanup_args(emu, if explicit_length { 2 } else { 1 });
}

pub fn SysFreeString(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "oleaut32!SysFreeString bstr={address:#x}");
    buffers::free(emu, address, BufferKind::Bstr);
    call.cleanup_args(emu, 1);
}

fn length_api(emu: &mut Emu, api: &str, bytes: bool) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    let size = address
        .checked_sub(PREFIX)
        .and_then(|p| emu.maps.read_dword(p))
        .unwrap_or(0) as u64;
    log_red!(emu, "oleaut32!{api} bstr={address:#x} byte_length={size}");
    call.set_return(emu, if bytes { size } else { size / 2 });
    call.cleanup_args(emu, 1);
}

fn realloc_api(emu: &mut Emu, api: &str, explicit_length: bool) {
    let call = WinApi(Stdcall);
    let output = api_try!(emu, call.arg(emu, 0));
    let source = api_try!(emu, call.arg(emu, 1));
    let bytes = if explicit_length {
        (api_try!(emu, call.arg_u32(emu, 2)) as u64).checked_mul(2)
    } else {
        string_bytes(emu, source)
    };
    log_red!(
        emu,
        "oleaut32!{api} output={output:#x} source={source:#x} bytes={bytes:?}"
    );
    let value = (|| {
        if !memory::writable_pointer(emu, output) {
            return None;
        }
        let old = memory::read_pointer(emu, output)?;
        if old != 0 && buffers::size(emu, old, BufferKind::Bstr).is_none() {
            return None;
        }
        let new = if !explicit_length && source == 0 {
            0
        } else {
            allocate(emu, source, bytes?)?
        };
        if !memory::write_pointer(emu, output, new) {
            buffers::free(emu, new, BufferKind::Bstr);
            return None;
        }
        buffers::free(emu, old, BufferKind::Bstr);
        Some(1)
    })()
    .unwrap_or(0);
    call.set_return(emu, value);
    call.cleanup_args(emu, if explicit_length { 3 } else { 2 });
}

pub fn SysAllocString(emu: &mut Emu) {
    alloc_api(emu, "SysAllocString", false, false);
}

pub fn SysAllocStringByteLen(emu: &mut Emu) {
    alloc_api(emu, "SysAllocStringByteLen", true, true);
}

pub fn SysAllocStringLen(emu: &mut Emu) {
    alloc_api(emu, "SysAllocStringLen", true, false);
}

pub fn SysReAllocString(emu: &mut Emu) {
    realloc_api(emu, "SysReAllocString", false);
}

pub fn SysReAllocStringLen(emu: &mut Emu) {
    realloc_api(emu, "SysReAllocStringLen", true);
}

pub fn SysStringByteLen(emu: &mut Emu) {
    length_api(emu, "SysStringByteLen", true);
}

pub fn SysStringLen(emu: &mut Emu) {
    length_api(emu, "SysStringLen", false);
}
