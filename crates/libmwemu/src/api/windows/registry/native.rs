use crate::api::abi::windows::WindowsAbi;
use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::memory::{pointer_size, read_pointer, write_pointer};
use crate::winapi::common::ntdll::read_unicode_string;
use crate::winapi::policy::{self, api_try};
use crate::winapi::registry_store::{
    self as store, REG_NOTIFY_CHANGE_NAME, REG_NOTIFY_THREAD_AGNOSTIC, Value,
};
use crate::windows::constants::errors::*;

const KEY_HANDLE_TAGS_INFORMATION: u32 = 7;
const KEY_SET_HANDLE_TAGS_INFORMATION: u32 = 5;

const KEY_VALUE_BASIC_INFORMATION: u32 = 0;
const KEY_VALUE_FULL_INFORMATION: u32 = 1;
const KEY_VALUE_PARTIAL_INFORMATION: u32 = 2;
const KEY_VALUE_FULL_INFORMATION_ALIGN64: u32 = 3;
const KEY_VALUE_PARTIAL_INFORMATION_ALIGN64: u32 = 4;

pub(crate) fn dispatch(name: &str, emu: &mut Emu, call: WindowsAbi) -> bool {
    dispatch_api!(name, {
        "NtNotifyChangeKey" => notify_key(emu, call),
        "NtOpenKey" => open_key(emu, call, false),
        "NtOpenKeyEx" => open_key(emu, call, true),
        "NtQueryKey" => query_key(emu, call),
        "NtSetInformationKey" => set_key(emu, call),
        "NtQueryValueKey" => query_value(emu, call),
    }, handled(()) => true, fallback => false)
}

pub(crate) fn notify_key(emu: &mut Emu, call: WindowsAbi) {
    let [
        key,
        event,
        apc,
        context,
        _iosb,
        filter,
        subtree,
        buffer,
        size,
        asynchronous,
    ] = api_try!(emu, call.args(emu));
    // Thread-agnostic event delivery neither retains nor writes the caller's IO_STATUS_BLOCK.
    if event == 0
        || apc != 0
        || context != 0
        || buffer != 0
        || size as u32 != 0
        || asynchronous as u8 == 0
        || filter as u32 != (REG_NOTIFY_CHANGE_NAME | REG_NOTIFY_THREAD_AGNOSTIC)
    {
        return policy::unsupported_with_abi(emu, "ntdll", "NtNotifyChangeKey", emu.pc(), call);
    }
    let status = emu
        .handle_management
        .watch_registry_names(key, event, subtree as u8 != 0);
    log_red!(
        emu,
        "NtNotifyChangeKey key={key:#x} event={event:#x} subtree={} => {status:#x}",
        subtree as u8 != 0
    );
    call.set_return(emu, status);
    call.cleanup_args(emu, 10);
}

fn key_name(emu: &Emu, attributes: u64) -> Result<String, u64> {
    let width = pointer_size(emu);
    let size = width * 6;
    if attributes == 0
        || !emu
            .maps
            .can_access_range(attributes, size as usize, Permission::READ)
    {
        return Err(STATUS_ACCESS_VIOLATION);
    }
    if emu.maps.read_dword(attributes) != Some(size as u32) {
        return Err(STATUS_INVALID_PARAMETER);
    }
    let root = read_pointer(emu, attributes + width).ok_or(STATUS_ACCESS_VIOLATION)?;
    let name_ptr = read_pointer(emu, attributes + width * 2).ok_or(STATUS_ACCESS_VIOLATION)?;
    let name = read_unicode_string(emu, name_ptr)?;
    if root != 0 {
        if name.starts_with('\\') {
            return Err(STATUS_OBJECT_PATH_SYNTAX_BAD);
        }
        let parent = emu
            .handle_management
            .registry_key(root)
            .ok_or(STATUS_INVALID_HANDLE)?;
        if parent.is_empty() {
            return Err(STATUS_KEY_DELETED);
        }
        return Ok(store::join(parent, &name));
    }
    let mut parts = name
        .strip_prefix('\\')
        .ok_or(STATUS_OBJECT_PATH_SYNTAX_BAD)?
        .splitn(3, '\\');
    if !parts
        .next()
        .is_some_and(|s| s.eq_ignore_ascii_case("Registry"))
    {
        return Err(STATUS_OBJECT_PATH_NOT_FOUND);
    }
    let hive = match parts.next() {
        Some(s) if s.eq_ignore_ascii_case("Machine") => "HKEY_LOCAL_MACHINE",
        Some(s) if s.eq_ignore_ascii_case("User") => "HKEY_USERS",
        _ => return Err(STATUS_OBJECT_PATH_NOT_FOUND),
    };
    Ok(store::join(hive, parts.next().unwrap_or_default()))
}

pub(crate) fn open_key(emu: &mut Emu, call: WindowsAbi, extended: bool) {
    let [out, access, attr] = api_try!(emu, call.args(emu));
    let options = if extended {
        api_try!(emu, call.arg_u32(emu, 3))
    } else {
        0
    };
    log_red!(
        emu,
        "NtOpenKey{} access=0x{:x} attributes=0x{:x}",
        if extended { "Ex" } else { "" },
        access,
        attr
    );
    let result = if out == 0 || !write_pointer(emu, out, 0) {
        Err(STATUS_ACCESS_VIOLATION)
    } else if options != 0 {
        Err(STATUS_NOT_SUPPORTED)
    } else {
        key_name(emu, attr).and_then(|path| {
            if !emu.registry.key_exists(&path) {
                return Err(STATUS_OBJECT_NAME_NOT_FOUND);
            }
            let handle = emu
                .handle_management
                .open_registry_key_access(path, access as u32) as u64;
            if !write_pointer(emu, out, handle) {
                emu.handle_management.close_registry_key(handle);
                return Err(STATUS_ACCESS_VIOLATION);
            }
            Ok(())
        })
    };
    call.set_return(emu, result.err().unwrap_or(STATUS_SUCCESS));
    call.cleanup_args(emu, 3 + usize::from(extended));
}

pub(crate) fn query_key(emu: &mut Emu, call: WindowsAbi) {
    let [handle, class, out, length, result] = api_try!(emu, call.args(emu));
    if class as u32 != KEY_HANDLE_TAGS_INFORMATION {
        return policy::unsupported_with_abi(emu, "ntdll", "NtQueryKey", emu.pc(), call);
    }
    log_red!(
        emu,
        "NtQueryKey handle={handle:#x} class=KeyHandleTagsInformation"
    );
    let needed = size_of::<u32>() as u32;
    let status = match emu.handle_management.registry_tags(handle) {
        None => STATUS_INVALID_HANDLE,
        Some(_) if !emu.maps.validate_write_range(result, needed as u64) => STATUS_ACCESS_VIOLATION,
        Some(_) if (length as u32) < needed => {
            emu.maps.write_dword(result, needed);
            STATUS_BUFFER_TOO_SMALL
        }
        Some(_) if !emu.maps.validate_write_range(out, needed as u64) => STATUS_ACCESS_VIOLATION,
        Some(tags) => {
            emu.maps.write_dword(out, tags as u32);
            emu.maps.write_dword(result, needed);
            STATUS_SUCCESS
        }
    };
    call.set_return(emu, status);
    call.cleanup_args(emu, 5);
}

pub(crate) fn set_key(emu: &mut Emu, call: WindowsAbi) {
    let [handle, class, data, length] = api_try!(emu, call.args(emu));
    if class as u32 != KEY_SET_HANDLE_TAGS_INFORMATION {
        return policy::unsupported_with_abi(emu, "ntdll", "NtSetInformationKey", emu.pc(), call);
    }
    log_red!(
        emu,
        "NtSetInformationKey handle={handle:#x} class=KeySetHandleTagsInformation"
    );
    let status = if length as u32 != size_of::<u32>() as u32 {
        STATUS_INFO_LENGTH_MISMATCH
    } else if let Some(value) = emu.maps.read_dword(data) {
        match emu.handle_management.registry_tags_mut(handle) {
            Some(tags) => {
                *tags = value as u16;
                STATUS_SUCCESS
            }
            None => STATUS_INVALID_HANDLE,
        }
    } else {
        STATUS_ACCESS_VIOLATION
    };
    call.set_return(emu, status);
    call.cleanup_args(emu, 4);
}

fn value_information(class: u32, name: &str, value: &Value) -> (usize, Vec<u8>) {
    let mut bytes = Vec::new();
    let mut fields = |values: &[u32]| {
        bytes.extend(values.iter().flat_map(|v| v.to_le_bytes()));
    };
    let data_len = value.data.len() as u32;
    if matches!(
        class,
        KEY_VALUE_PARTIAL_INFORMATION | KEY_VALUE_PARTIAL_INFORMATION_ALIGN64
    ) {
        if class == KEY_VALUE_PARTIAL_INFORMATION {
            fields(&[0, value.kind, data_len]);
        } else {
            fields(&[value.kind, data_len]);
        }
        let header = bytes.len();
        bytes.extend_from_slice(&value.data);
        return (header, bytes);
    }
    let name_len = (name.encode_utf16().count() * 2) as u32;
    let full = class != KEY_VALUE_BASIC_INFORMATION;
    let offset = (5 * size_of::<u32>() + name_len as usize).next_multiple_of(size_of::<u64>());
    if full {
        fields(&[
            0,
            value.kind,
            if data_len == 0 {
                u32::MAX
            } else {
                offset as u32
            },
            data_len,
            name_len,
        ]);
    } else {
        fields(&[0, value.kind, name_len]);
    }
    let header = bytes.len();
    bytes.extend(name.encode_utf16().flat_map(u16::to_le_bytes));
    if full && data_len != 0 {
        bytes.resize(offset, 0);
        bytes.extend_from_slice(&value.data);
    }
    (header, bytes)
}

fn copy_value(
    emu: &mut Emu,
    handle: u64,
    name_ptr: u64,
    class: u32,
    out: u64,
    size: usize,
    result: u64,
) -> Result<(), u64> {
    if class > KEY_VALUE_PARTIAL_INFORMATION_ALIGN64 {
        return Err(STATUS_INVALID_PARAMETER);
    }
    if class >= KEY_VALUE_FULL_INFORMATION_ALIGN64 && !out.is_multiple_of(8) {
        return Err(STATUS_DATATYPE_MISALIGNMENT);
    }
    if result == 0
        || !emu.maps.validate_write_range(result, 4)
        || (size != 0 && (out == 0 || !emu.maps.validate_write_range(out, size as u64)))
    {
        return Err(STATUS_ACCESS_VIOLATION);
    }
    let name = read_unicode_string(emu, name_ptr)?;
    let path = emu
        .handle_management
        .registry_key(handle)
        .ok_or(STATUS_INVALID_HANDLE)?;
    if path.is_empty() {
        return Err(STATUS_KEY_DELETED);
    }
    if !emu.handle_management.registry_can_query(handle) {
        return Err(STATUS_ACCESS_DENIED);
    }
    let (stored, value) = emu
        .registry
        .named_value(path, &name)
        .ok_or(STATUS_OBJECT_NAME_NOT_FOUND)?;
    let name_len = stored.encode_utf16().count() * 2;
    let (header, mut bytes) = value_information(class, stored, value);
    let needed = bytes.len();
    if !emu.maps.write_dword(result, needed as u32) {
        return Err(STATUS_ACCESS_VIOLATION);
    }
    if size < header {
        return Err(STATUS_BUFFER_TOO_SMALL);
    }
    bytes.truncate(size);
    if matches!(
        class,
        KEY_VALUE_BASIC_INFORMATION
            | KEY_VALUE_FULL_INFORMATION
            | KEY_VALUE_FULL_INFORMATION_ALIGN64
    ) && size < header + name_len
        && !size.is_multiple_of(2)
        && let Some(last) = bytes.last_mut()
    {
        *last = 0;
    }
    if !emu.maps.write_bytes(out, &bytes) {
        return Err(STATUS_ACCESS_VIOLATION);
    }
    if size < needed {
        Err(STATUS_BUFFER_OVERFLOW)
    } else {
        Ok(())
    }
}

pub(crate) fn query_value(emu: &mut Emu, call: WindowsAbi) {
    let [handle, name, class, out, size, result] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "NtQueryValueKey handle=0x{:x} class={}",
        handle,
        class as u32
    );
    let result = copy_value(
        emu,
        handle,
        name,
        class as u32,
        out,
        size as u32 as usize,
        result,
    );
    call.set_return(emu, result.err().unwrap_or(STATUS_SUCCESS));
    call.cleanup_args(emu, 6);
}
