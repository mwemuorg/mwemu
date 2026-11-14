use crate::emu::Emu;
use crate::maps::buffers::BufferKind;
use crate::maps::mem64::Permission;
use crate::winapi::common::memory::Call;
use crate::winapi::common::trace;
use crate::winapi::common::{buffers, memory};

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

pub(crate) fn alloc_api(emu: &mut Emu, explicit_length: bool, byte_length: bool) {
    let source = Call::Api.arg(emu, 0);
    let bytes = if explicit_length {
        let count = Call::Api.arg(emu, 1) as u32 as u64;
        count.checked_mul(if byte_length { 1 } else { 2 })
    } else {
        string_bytes(emu, source)
    };
    let value = if !explicit_length && source == 0 {
        0
    } else {
        bytes.and_then(|n| allocate(emu, source, n)).unwrap_or(0)
    };
    Call::Api.finish(emu, value, if explicit_length { 2 } else { 1 });
}

pub(crate) fn free_api(emu: &mut Emu) {
    let address = Call::Api.arg(emu, 0);
    buffers::free(emu, address, BufferKind::Bstr);
    Call::Api.finish(emu, 0, 1);
}

pub(crate) fn length_api(emu: &mut Emu, bytes: bool) {
    let address = Call::Api.arg(emu, 0);
    let size = address
        .checked_sub(PREFIX)
        .and_then(|p| emu.maps.read_dword(p))
        .unwrap_or(0) as u64;
    Call::Api.finish(emu, if bytes { size } else { size / 2 }, 1);
}

pub(crate) fn realloc_api(emu: &mut Emu, explicit_length: bool) {
    let output = Call::Api.arg(emu, 0);
    let source = Call::Api.arg(emu, 1);
    let bytes = if explicit_length {
        (Call::Api.arg(emu, 2) as u32 as u64).checked_mul(2)
    } else {
        string_bytes(emu, source)
    };
    let value = (|| {
        if !memory::writable_pointer(emu, output) {
            return None;
        }
        let old = memory::read_pointer(emu, output)?;
        if old != 0 && buffers::size(emu, old, BufferKind::Bstr).is_none() {
            return None;
        }
        let new = allocate(emu, source, bytes?)?;
        memory::write_pointer(emu, output, new);
        buffers::free(emu, old, BufferKind::Bstr);
        Some(1)
    })()
    .unwrap_or(0);
    Call::Api.finish(emu, value, if explicit_length { 3 } else { 2 });
}

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    let (handler, parameters, returns_value): (fn(&mut Emu), &str, bool) = match api {
        "SysAllocString" => (|emu| alloc_api(emu, false, false), "source", true),
        "SysAllocStringLen" => (|emu| alloc_api(emu, true, false), "source length", true),
        "SysAllocStringByteLen" => (|emu| alloc_api(emu, true, true), "source bytes", true),
        "SysFreeString" => (free_api, "string", false),
        "SysStringLen" => (|emu| length_api(emu, false), "string", true),
        "SysStringByteLen" => (|emu| length_api(emu, true), "string", true),
        "SysReAllocString" => (|emu| realloc_api(emu, false), "output source", true),
        "SysReAllocStringLen" => (|emu| realloc_api(emu, true), "output source length", true),
        _ => return false,
    };
    trace::api(emu, api, parameters, returns_value, handler);
    true
}
