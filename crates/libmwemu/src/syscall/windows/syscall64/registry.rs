use crate::api::abi::windows::WindowsAbi::WinSyscall;
use crate::api::windows::common::ntdll::read_unicode_string;
use crate::emu::Emu;
use crate::emu::object_handle::NativeObject;
use crate::maps::mem64::Permission;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::security::{
    GENERIC_ALL, GENERIC_EXECUTE, GENERIC_READ, MAXIMUM_ALLOWED,
};

const DIRECTORY_TRAVERSE: u32 = 0x0002;
const SYMBOLIC_LINK_QUERY: u32 = 0x0001;
const OBJ_CASE_INSENSITIVE: u32 = 0x0040;
const OBJECT_ATTRIBUTES_SIZE: usize = 48;

fn allows(access: u32, right: u32, generic: u32) -> bool {
    access & (right | generic | MAXIMUM_ALLOWED | GENERIC_ALL) != 0
}

pub(super) fn known_dll_directory(emu: &Emu, handle: u64) -> Result<bool, u64> {
    match emu.handle_management.native_object(handle) {
        Some(NativeObject::KnownDllDirectory { access, wow64 }) => {
            if allows(access, DIRECTORY_TRAVERSE, GENERIC_EXECUTE) {
                Ok(wow64)
            } else {
                Err(STATUS_ACCESS_DENIED)
            }
        }
        None if emu.known_dll_dir_handles.contains(&handle) => Ok(false),
        _ => Err(STATUS_INVALID_HANDLE),
    }
}

pub(super) fn object_attributes(emu: &Emu, address: u64) -> Result<(u64, String, bool), u64> {
    if address == 0
        || !emu
            .maps
            .can_access_range(address, OBJECT_ATTRIBUTES_SIZE, Permission::READ)
    {
        return Err(STATUS_ACCESS_VIOLATION);
    }
    if emu.maps.read_dword(address) != Some(OBJECT_ATTRIBUTES_SIZE as u32) {
        return Err(STATUS_INVALID_PARAMETER);
    }
    let root = emu
        .maps
        .read_qword(address + 8)
        .ok_or(STATUS_ACCESS_VIOLATION)?;
    let name = emu
        .maps
        .read_qword(address + 16)
        .ok_or(STATUS_ACCESS_VIOLATION)?;
    let flags = emu
        .maps
        .read_dword(address + 24)
        .ok_or(STATUS_ACCESS_VIOLATION)?;
    Ok((
        root,
        read_unicode_string(emu, name)?,
        flags & OBJ_CASE_INSENSITIVE != 0,
    ))
}

fn known_path(emu: &Emu, object: u64, link: bool) -> Result<bool, u64> {
    let (root, name, ignore_case) = object_attributes(emu, object)?;
    let equals = |expected: &str| {
        if ignore_case {
            name.eq_ignore_ascii_case(expected)
        } else {
            name == expected
        }
    };
    if root != 0 {
        let wow64 = known_dll_directory(emu, root)?;
        return (link && equals("KnownDllPath"))
            .then_some(wow64)
            .ok_or(STATUS_OBJECT_NAME_NOT_FOUND);
    }
    let (native, wow64) = if link {
        (r"\KnownDlls\KnownDllPath", r"\KnownDlls32\KnownDllPath")
    } else {
        (r"\KnownDlls", r"\KnownDlls32")
    };
    if equals(native) {
        Ok(false)
    } else if equals(wow64) {
        Ok(true)
    } else {
        Err(STATUS_OBJECT_NAME_NOT_FOUND)
    }
}

fn open_known_object(emu: &mut Emu, link: bool) {
    let call = WinSyscall;
    let [out, access, attributes] = api_try!(emu, call.args(emu));
    let access = access as u32;
    if !emu.maps.validate_write_range(out, 8) {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    }
    let wow64 = match known_path(emu, attributes, link) {
        Ok(wow64) => wow64,
        Err(status) => return call.set_return(emu, status),
    };
    let object = if link {
        NativeObject::KnownDllPath { wow64, access }
    } else {
        NativeObject::KnownDllDirectory { wow64, access }
    };
    let handle = emu.handle_management.insert_native_object(object);
    emu.maps.write_qword(out, handle);
    log_orange!(emu, "KnownDll object={object:?} handle={handle:#x}");
    call.set_return(emu, STATUS_SUCCESS);
}

pub(super) fn nt_open_directory_object(emu: &mut Emu) {
    open_known_object(emu, false);
}

pub(super) fn nt_open_symbolic_link_object(emu: &mut Emu) {
    open_known_object(emu, true);
}

pub(super) fn nt_query_symbolic_link_object(emu: &mut Emu) {
    let call = WinSyscall;
    let [handle, descriptor, returned] = api_try!(emu, call.args(emu));
    let target = match emu.handle_management.native_object(handle) {
        Some(NativeObject::KnownDllPath { wow64, access }) => {
            if !allows(access, SYMBOLIC_LINK_QUERY, GENERIC_READ) {
                return call.set_return(emu, STATUS_ACCESS_DENIED);
            }
            if wow64 {
                r"C:\Windows\SysWOW64"
            } else {
                r"C:\Windows\System32"
            }
        }
        None => match emu.symbolic_link_targets.get(&handle) {
            Some(target) if !target.is_empty() => target.as_str(),
            _ => return call.set_return(emu, STATUS_INVALID_HANDLE),
        },
        _ => return call.set_return(emu, STATUS_INVALID_HANDLE),
    };
    let bytes: Vec<u8> = target.encode_utf16().flat_map(u16::to_le_bytes).collect();
    if !emu.maps.can_access_range(descriptor, 16, Permission::READ)
        || !emu.maps.validate_write_range(descriptor, 2)
        || (returned != 0 && !emu.maps.validate_write_range(returned, 4))
    {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    }
    let Some(maximum) = emu.maps.read_word(descriptor + 2) else {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    };
    if bytes.len() > usize::from(maximum) {
        if returned != 0 {
            emu.maps.write_dword(returned, bytes.len() as u32);
        }
        return call.set_return(emu, STATUS_BUFFER_TOO_SMALL);
    }
    let Some(buffer) = emu.maps.read_qword(descriptor + 8) else {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    };
    let terminator_fits = bytes.len() + 2 <= usize::from(maximum);
    let written = bytes.len() + if terminator_fits { 2 } else { 0 };
    if !emu.maps.validate_write_range(buffer, written as u64) {
        return call.set_return(emu, STATUS_ACCESS_VIOLATION);
    }
    emu.maps.write_bytes(buffer, &bytes);
    if terminator_fits {
        emu.maps.write_word(buffer + bytes.len() as u64, 0);
    }
    emu.maps.write_word(descriptor, bytes.len() as u16);
    if returned != 0 {
        emu.maps.write_dword(returned, written as u32);
    }
    call.set_return(emu, STATUS_SUCCESS);
}
