/*
    Reg* APIs over registry_store. advapi32, kernel32 and kernelbase all
    export them, so each gateway tries `gateway` first.

    Key handles are `registry://<full path>` handles; the predefined HKEY_*
    values are accepted directly. The ANSI variants convert names and string
    data, which the store keeps as UTF-16.
*/

use crate::emu;
use crate::winapi::helper;
use crate::winapi::registry_store::{self as store, Value};
use crate::windows::constants::*;

const HANDLE_PREFIX: &str = "registry://";

const REG_CREATED_NEW_KEY: u32 = 1;
const REG_OPENED_EXISTING_KEY: u32 = 2;

/// RegGetValue type filter bits.
const RRF_RT_ANY: u32 = 0xffff;

/// Run `api` if it is a registry API; false otherwise.
pub fn gateway(api: &str, emu: &mut emu::Emu) -> bool {
    let wide = api.ends_with('W');
    match api {
        "RegOpenKeyExA" | "RegOpenKeyExW" => open_key(emu, api, wide, true),
        "RegOpenKeyA" | "RegOpenKeyW" => open_key(emu, api, wide, false),
        "RegCreateKeyExA" | "RegCreateKeyExW" => create_key(emu, api, wide, true),
        "RegCreateKeyA" | "RegCreateKeyW" => create_key(emu, api, wide, false),
        "RegCloseKey" => close_key(emu),
        "RegFlushKey" => flush_key(emu),
        "RegSetValueExA" | "RegSetValueExW" => set_value(emu, api, wide),
        "RegQueryValueExA" | "RegQueryValueExW" => query_value(emu, api, wide),
        "RegGetValueA" | "RegGetValueW" => get_value(emu, api, wide),
        "RegDeleteValueA" | "RegDeleteValueW" => delete_value(emu, api, wide),
        "RegDeleteKeyA" | "RegDeleteKeyW" | "RegDeleteKeyExA" | "RegDeleteKeyExW" => {
            delete_key(emu, api, wide, false)
        }
        "RegDeleteTreeA" | "RegDeleteTreeW" => delete_key(emu, api, wide, true),
        "RegEnumKeyExA" | "RegEnumKeyExW" => enum_key(emu, api, wide),
        "RegEnumValueA" | "RegEnumValueW" => enum_value(emu, api, wide),
        "RegQueryInfoKeyA" | "RegQueryInfoKeyW" => query_info_key(emu, api),
        _ => return false,
    }
    true
}

fn stack_arg(emu: &emu::Emu, index: u64) -> u64 {
    // after the gateway popped the return address, the 5th argument is at rsp+0x20
    emu.maps
        .read_qword(emu.regs().rsp + 0x20 + 8 * (index - 4))
        .unwrap_or(0)
}

fn read_name(emu: &emu::Emu, ptr: u64, wide: bool) -> Option<String> {
    if ptr == 0 {
        None
    } else if wide {
        Some(emu.maps.read_wide_string(ptr))
    } else {
        Some(emu.maps.read_string(ptr))
    }
}

/// Full path of an open key handle.
fn key_path(hkey: u64) -> Result<String, u64> {
    if let Some(path) = store::predefined_path(hkey) {
        return Ok(path.to_string());
    }
    let uri = helper::handler_get_uri(hkey);
    let path = uri
        .strip_prefix(HANDLE_PREFIX)
        .ok_or(ERROR_INVALID_HANDLE)?;
    if store::key_exists(path) {
        Ok(path.to_string())
    } else {
        Err(ERROR_KEY_DELETED)
    }
}

fn open_handle(path: &str) -> u64 {
    helper::handler_create(&format!("{}{}", HANDLE_PREFIX, path))
}

fn finish(emu: &mut emu::Emu, api: &str, what: &str, err: u64) {
    log_red!(emu, "advapi32!{} {} = {}", api, what, err);
    emu.regs_mut().rax = err;
}

fn is_string(kind: u32) -> bool {
    matches!(
        kind,
        store::REG_SZ | store::REG_EXPAND_SZ | store::REG_MULTI_SZ
    )
}

/// Data as the caller passed it, converted to the stored form.
fn data_in(kind: u32, data: &[u8], wide: bool) -> Vec<u8> {
    if wide || !is_string(kind) {
        return data.to_vec();
    }
    data.iter()
        .flat_map(|b| (*b as u16).to_le_bytes())
        .collect()
}

/// Stored data in the form the caller reads.
fn data_out(value: &Value, wide: bool) -> Vec<u8> {
    if wide || !is_string(value.kind) {
        return value.data.clone();
    }
    value
        .data
        .chunks(2)
        .map(|c| match c {
            [lo, 0] => *lo,
            _ => b'?',
        })
        .collect()
}

/// Copy `data` to an optional (lpData, lpcbData) pair.
fn write_data(emu: &mut emu::Emu, data_ptr: u64, size_ptr: u64, data: &[u8]) -> u64 {
    if size_ptr == 0 {
        return if data_ptr == 0 {
            ERROR_SUCCESS
        } else {
            ERROR_INVALID_PARAMETER
        };
    }
    let capacity = emu.maps.read_dword(size_ptr).unwrap_or(0) as usize;
    emu.maps.write_dword(size_ptr, data.len() as u32);
    if data_ptr == 0 {
        return ERROR_SUCCESS;
    }
    if data.len() > capacity {
        return ERROR_MORE_DATA;
    }
    emu.maps.write_bytes(data_ptr, data);
    ERROR_SUCCESS
}

/// Copy a name to a buffer of `*cch_ptr` characters; on success `*cch_ptr`
/// is the length without the terminator.
fn write_name(emu: &mut emu::Emu, ptr: u64, cch_ptr: u64, name: &str, wide: bool) -> u64 {
    if ptr == 0 || cch_ptr == 0 {
        return ERROR_INVALID_PARAMETER;
    }
    let capacity = emu.maps.read_dword(cch_ptr).unwrap_or(0) as usize;
    let len = if wide {
        name.encode_utf16().count()
    } else {
        name.len()
    };
    if len + 1 > capacity {
        return ERROR_MORE_DATA;
    }
    if wide {
        emu.maps.write_wide_string(ptr, name);
    } else {
        emu.maps.write_string(ptr, name);
    }
    emu.maps.write_dword(cch_ptr, len as u32);
    ERROR_SUCCESS
}

/*
LSTATUS RegOpenKeyExW(HKEY hKey, LPCWSTR lpSubKey, DWORD ulOptions,
                      REGSAM samDesired, PHKEY phkResult);
LSTATUS RegOpenKeyW(HKEY hKey, LPCWSTR lpSubKey, PHKEY phkResult);
*/
fn open_key(emu: &mut emu::Emu, api: &str, wide: bool, ex: bool) {
    let hkey = emu.regs().rcx;
    let sub = read_name(emu, emu.regs().rdx, wide).unwrap_or_default();
    let result_ptr = if ex { stack_arg(emu, 4) } else { emu.regs().r8 };

    let base = match key_path(hkey) {
        Ok(base) => base,
        Err(err) => return finish(emu, api, &sub, err),
    };
    let path = store::join(&base, &sub);
    let err = if result_ptr == 0 {
        ERROR_INVALID_PARAMETER
    } else if !store::key_exists(&path) {
        ERROR_FILE_NOT_FOUND
    } else {
        emu.maps.write_qword(result_ptr, open_handle(&path));
        ERROR_SUCCESS
    };
    finish(emu, api, &path, err);
}

/*
LSTATUS RegCreateKeyExW(HKEY hKey, LPCWSTR lpSubKey, DWORD Reserved,
                        LPWSTR lpClass, DWORD dwOptions, REGSAM samDesired,
                        const LPSECURITY_ATTRIBUTES lpSecurityAttributes,
                        PHKEY phkResult, LPDWORD lpdwDisposition);
LSTATUS RegCreateKeyW(HKEY hKey, LPCWSTR lpSubKey, PHKEY phkResult);
*/
fn create_key(emu: &mut emu::Emu, api: &str, wide: bool, ex: bool) {
    let hkey = emu.regs().rcx;
    let sub = read_name(emu, emu.regs().rdx, wide).unwrap_or_default();
    let (result_ptr, disposition_ptr) = if ex {
        (stack_arg(emu, 7), stack_arg(emu, 8))
    } else {
        (emu.regs().r8, 0)
    };

    let base = match key_path(hkey) {
        Ok(base) => base,
        Err(err) => return finish(emu, api, &sub, err),
    };
    let path = store::join(&base, &sub);
    if result_ptr == 0 {
        return finish(emu, api, &path, ERROR_INVALID_PARAMETER);
    }
    let Some(created) = store::create_key(&path) else {
        return finish(emu, api, &path, ERROR_ACCESS_DENIED);
    };
    emu.maps.write_qword(result_ptr, open_handle(&path));
    if disposition_ptr != 0 {
        let disposition = if created {
            REG_CREATED_NEW_KEY
        } else {
            REG_OPENED_EXISTING_KEY
        };
        emu.maps.write_dword(disposition_ptr, disposition);
    }
    let what = format!("{} created: {}", path, created);
    finish(emu, api, &what, ERROR_SUCCESS);
}

/* LSTATUS RegCloseKey(HKEY hKey); */
fn close_key(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let err = if store::predefined_path(hkey).is_some() {
        ERROR_SUCCESS
    } else if helper::handler_get_uri(hkey).starts_with(HANDLE_PREFIX) {
        helper::handler_close(hkey);
        ERROR_SUCCESS
    } else {
        ERROR_INVALID_HANDLE
    };
    finish(emu, "RegCloseKey", &format!("0x{:x}", hkey), err);
}

/* LSTATUS RegFlushKey(HKEY hKey); */
fn flush_key(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let err = key_path(hkey).err().unwrap_or(ERROR_SUCCESS);
    finish(emu, "RegFlushKey", &format!("0x{:x}", hkey), err);
}

/*
LSTATUS RegSetValueExW(HKEY hKey, LPCWSTR lpValueName, DWORD Reserved,
                       DWORD dwType, const BYTE *lpData, DWORD cbData);
*/
fn set_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let name = read_name(emu, emu.regs().rdx, wide).unwrap_or_default();
    let kind = emu.regs().r9 as u32;
    let data_ptr = stack_arg(emu, 4);
    let size = stack_arg(emu, 5) as u32 as usize;

    let path = match key_path(hkey) {
        Ok(path) => path,
        Err(err) => return finish(emu, api, &name, err),
    };
    let data = if size == 0 {
        Vec::new()
    } else {
        match emu.maps.try_read_bytes(data_ptr, size) {
            Some(bytes) => bytes.to_vec(),
            None => return finish(emu, api, &name, ERROR_INVALID_PARAMETER),
        }
    };
    let value = Value {
        kind,
        data: data_in(kind, &data, wide),
    };
    store::set_value(&path, &name, value);
    let what = format!("{} `{}` type {} ({} bytes)", path, name, kind, size);
    finish(emu, api, &what, ERROR_SUCCESS);
}

/*
LSTATUS RegQueryValueExW(HKEY hKey, LPCWSTR lpValueName, LPDWORD lpReserved,
                         LPDWORD lpType, LPBYTE lpData, LPDWORD lpcbData);
*/
fn query_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let name = read_name(emu, emu.regs().rdx, wide).unwrap_or_default();
    let type_ptr = emu.regs().r9;
    let data_ptr = stack_arg(emu, 4);
    let size_ptr = stack_arg(emu, 5);

    let path = match key_path(hkey) {
        Ok(path) => path,
        Err(err) => return finish(emu, api, &name, err),
    };
    let what = format!("{} `{}`", path, name);
    let Some(value) = store::get_value(&path, &name) else {
        return finish(emu, api, &what, ERROR_FILE_NOT_FOUND);
    };
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, value.kind);
    }
    let err = write_data(emu, data_ptr, size_ptr, &data_out(&value, wide));
    finish(emu, api, &what, err);
}

/// RRF_RT_* bit that admits `kind`.
fn type_flag(kind: u32) -> u32 {
    match kind {
        store::REG_NONE => 0x01,
        store::REG_SZ => 0x02,
        store::REG_EXPAND_SZ => 0x04,
        store::REG_BINARY => 0x08,
        store::REG_DWORD => 0x10,
        store::REG_MULTI_SZ => 0x20,
        store::REG_QWORD => 0x40,
        _ => 0,
    }
}

/*
LSTATUS RegGetValueW(HKEY hkey, LPCWSTR lpSubKey, LPCWSTR lpValue,
                     DWORD dwFlags, LPDWORD pdwType, PVOID pvData,
                     LPDWORD pcbData);
*/
fn get_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let sub = read_name(emu, emu.regs().rdx, wide).unwrap_or_default();
    let name = read_name(emu, emu.regs().r8, wide).unwrap_or_default();
    let flags = emu.regs().r9 as u32;
    let type_ptr = stack_arg(emu, 4);
    let data_ptr = stack_arg(emu, 5);
    let size_ptr = stack_arg(emu, 6);

    let base = match key_path(hkey) {
        Ok(base) => base,
        Err(err) => return finish(emu, api, &name, err),
    };
    let path = store::join(&base, &sub);
    let what = format!("{} `{}`", path, name);
    if !store::key_exists(&path) {
        return finish(emu, api, &what, ERROR_FILE_NOT_FOUND);
    }
    let Some(value) = store::get_value(&path, &name) else {
        return finish(emu, api, &what, ERROR_FILE_NOT_FOUND);
    };
    if flags & RRF_RT_ANY & type_flag(value.kind) == 0 {
        return finish(emu, api, &what, ERROR_UNSUPPORTED_TYPE);
    }
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, value.kind);
    }
    let err = write_data(emu, data_ptr, size_ptr, &data_out(&value, wide));
    finish(emu, api, &what, err);
}

/* LSTATUS RegDeleteValueW(HKEY hKey, LPCWSTR lpValueName); */
fn delete_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let name = read_name(emu, emu.regs().rdx, wide).unwrap_or_default();
    let path = match key_path(hkey) {
        Ok(path) => path,
        Err(err) => return finish(emu, api, &name, err),
    };
    let err = if store::delete_value(&path, &name) {
        ERROR_SUCCESS
    } else {
        ERROR_FILE_NOT_FOUND
    };
    finish(emu, api, &format!("{} `{}`", path, name), err);
}

/*
LSTATUS RegDeleteKeyW(HKEY hKey, LPCWSTR lpSubKey);
LSTATUS RegDeleteKeyExW(HKEY hKey, LPCWSTR lpSubKey, REGSAM samDesired, DWORD Reserved);
LSTATUS RegDeleteTreeW(HKEY hKey, LPCWSTR lpSubKey);
*/
fn delete_key(emu: &mut emu::Emu, api: &str, wide: bool, tree: bool) {
    let hkey = emu.regs().rcx;
    let sub = read_name(emu, emu.regs().rdx, wide);
    let base = match key_path(hkey) {
        Ok(base) => base,
        Err(err) => return finish(emu, api, "", err),
    };
    let err = match sub.as_deref() {
        // RegDeleteTree without a subkey empties the key itself
        None | Some("") if tree => {
            store::clear_key(&base);
            ERROR_SUCCESS
        }
        // RegDeleteKey needs a subkey: a key cannot delete itself
        None | Some("") => ERROR_INVALID_PARAMETER,
        Some(sub) => match store::delete_key(&store::join(&base, sub), tree) {
            Ok(()) => ERROR_SUCCESS,
            Err(store::DeleteError::NotFound) => ERROR_FILE_NOT_FOUND,
            Err(store::DeleteError::HasSubkeys) => ERROR_ACCESS_DENIED,
        },
    };
    let what = store::join(&base, sub.as_deref().unwrap_or(""));
    finish(emu, api, &what, err);
}

/*
LSTATUS RegEnumKeyExW(HKEY hKey, DWORD dwIndex, LPWSTR lpName,
                      LPDWORD lpcchName, LPDWORD lpReserved, LPWSTR lpClass,
                      LPDWORD lpcchClass, PFILETIME lpftLastWriteTime);
*/
fn enum_key(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let index = emu.regs().rdx as u32 as usize;
    let name_ptr = emu.regs().r8;
    let cch_ptr = emu.regs().r9;
    let class_cch_ptr = stack_arg(emu, 6);
    let time_ptr = stack_arg(emu, 7);

    let path = match key_path(hkey) {
        Ok(path) => path,
        Err(err) => return finish(emu, api, "", err),
    };
    let what = format!("{} #{}", path, index);
    let Some(name) = store::subkeys(&path).into_iter().nth(index) else {
        return finish(emu, api, &what, ERROR_NO_MORE_ITEMS);
    };
    let err = write_name(emu, name_ptr, cch_ptr, &name, wide);
    if err == ERROR_SUCCESS {
        // keys carry no class
        if class_cch_ptr != 0 {
            emu.maps.write_dword(class_cch_ptr, 0);
        }
        if time_ptr != 0 {
            emu.maps.write_qword(time_ptr, 0);
        }
    }
    finish(emu, api, &format!("{} = {}", what, name), err);
}

/*
LSTATUS RegEnumValueW(HKEY hKey, DWORD dwIndex, LPWSTR lpValueName,
                      LPDWORD lpcchValueName, LPDWORD lpReserved,
                      LPDWORD lpType, LPBYTE lpData, LPDWORD lpcbData);
*/
fn enum_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let index = emu.regs().rdx as u32 as usize;
    let name_ptr = emu.regs().r8;
    let cch_ptr = emu.regs().r9;
    let type_ptr = stack_arg(emu, 5);
    let data_ptr = stack_arg(emu, 6);
    let size_ptr = stack_arg(emu, 7);

    let path = match key_path(hkey) {
        Ok(path) => path,
        Err(err) => return finish(emu, api, "", err),
    };
    let what = format!("{} #{}", path, index);
    let Some((name, value)) = store::value_at(&path, index) else {
        return finish(emu, api, &what, ERROR_NO_MORE_ITEMS);
    };
    let mut err = write_name(emu, name_ptr, cch_ptr, &name, wide);
    if err == ERROR_SUCCESS {
        if type_ptr != 0 {
            emu.maps.write_dword(type_ptr, value.kind);
        }
        err = write_data(emu, data_ptr, size_ptr, &data_out(&value, wide));
    }
    finish(emu, api, &format!("{} = `{}`", what, name), err);
}

/*
LSTATUS RegQueryInfoKeyW(HKEY hKey, LPWSTR lpClass, LPDWORD lpcchClass,
                         LPDWORD lpReserved, LPDWORD lpcSubKeys,
                         LPDWORD lpcbMaxSubKeyLen, LPDWORD lpcbMaxClassLen,
                         LPDWORD lpcValues, LPDWORD lpcbMaxValueNameLen,
                         LPDWORD lpcbMaxValueLen, LPDWORD lpcbSecurityDescriptor,
                         PFILETIME lpftLastWriteTime);
*/
fn query_info_key(emu: &mut emu::Emu, api: &str) {
    let hkey = emu.regs().rcx;
    let class_ptr = emu.regs().rdx;
    let class_cch_ptr = emu.regs().r8;
    let path = match key_path(hkey) {
        Ok(path) => path,
        Err(err) => return finish(emu, api, "", err),
    };
    let Some(info) = store::key_info(&path) else {
        return finish(emu, api, &path, ERROR_KEY_DELETED);
    };
    // keys carry no class
    if class_ptr != 0 && class_cch_ptr != 0 {
        emu.maps.write_word(class_ptr, 0);
    }
    if class_cch_ptr != 0 {
        emu.maps.write_dword(class_cch_ptr, 0);
    }
    let fields = [
        (4, info.subkeys),
        (5, info.max_subkey_len),
        (6, 0),
        (7, info.values),
        (8, info.max_value_name_len),
        (9, info.max_value_len),
        (10, 0),
    ];
    for (index, value) in fields {
        let ptr = stack_arg(emu, index);
        if ptr != 0 {
            emu.maps.write_dword(ptr, value);
        }
    }
    let time_ptr = stack_arg(emu, 11);
    if time_ptr != 0 {
        emu.maps.write_qword(time_ptr, 0);
    }
    finish(emu, api, &path, ERROR_SUCCESS);
}
