mod close_key;

use crate::winapi::common::strings;
use crate::windows::constants::errors::*;
use close_key::close_key;
mod create_key;
use create_key::create_key;
mod delete_key;
use delete_key::delete_key;
mod delete_value;
use delete_value::delete_value;
mod enum_key;
use enum_key::enum_key;
mod enum_value;
use enum_value::enum_value;
mod flush_key;
use flush_key::flush_key;
mod get_value;
use get_value::get_value;
mod open_key;
use open_key::open_key;
mod query_info_key;
use query_info_key::query_info_key;
mod query_value;
use query_value::query_value;
mod set_value;
use set_value::set_value;

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

fn read_name(emu: &emu::Emu, ptr: u64, wide: bool) -> Option<String> {
    if ptr == 0 {
        None
    } else {
        Some(strings::read_text(emu, ptr, wide))
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
    strings::write_string(emu, ptr, name, wide);
    emu.maps.write_dword(cch_ptr, len as u32);
    ERROR_SUCCESS
}

/*
LSTATUS RegOpenKeyExW(HKEY hKey, LPCWSTR lpSubKey, DWORD ulOptions,
                      REGSAM samDesired, PHKEY phkResult);
LSTATUS RegOpenKeyW(HKEY hKey, LPCWSTR lpSubKey, PHKEY phkResult);
*/
/*
LSTATUS RegCreateKeyExW(HKEY hKey, LPCWSTR lpSubKey, DWORD Reserved,
                        LPWSTR lpClass, DWORD dwOptions, REGSAM samDesired,
                        const LPSECURITY_ATTRIBUTES lpSecurityAttributes,
                        PHKEY phkResult, LPDWORD lpdwDisposition);
LSTATUS RegCreateKeyW(HKEY hKey, LPCWSTR lpSubKey, PHKEY phkResult);
*/
/* LSTATUS RegCloseKey(HKEY hKey); */
/* LSTATUS RegFlushKey(HKEY hKey); */
/*
LSTATUS RegSetValueExW(HKEY hKey, LPCWSTR lpValueName, DWORD Reserved,
                       DWORD dwType, const BYTE *lpData, DWORD cbData);
*/
/*
LSTATUS RegQueryValueExW(HKEY hKey, LPCWSTR lpValueName, LPDWORD lpReserved,
                         LPDWORD lpType, LPBYTE lpData, LPDWORD lpcbData);
*/
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
/* LSTATUS RegDeleteValueW(HKEY hKey, LPCWSTR lpValueName); */
/*
LSTATUS RegDeleteKeyW(HKEY hKey, LPCWSTR lpSubKey);
LSTATUS RegDeleteKeyExW(HKEY hKey, LPCWSTR lpSubKey, REGSAM samDesired, DWORD Reserved);
LSTATUS RegDeleteTreeW(HKEY hKey, LPCWSTR lpSubKey);
*/
/*
LSTATUS RegEnumKeyExW(HKEY hKey, DWORD dwIndex, LPWSTR lpName,
                      LPDWORD lpcchName, LPDWORD lpReserved, LPWSTR lpClass,
                      LPDWORD lpcchClass, PFILETIME lpftLastWriteTime);
*/
/*
LSTATUS RegEnumValueW(HKEY hKey, DWORD dwIndex, LPWSTR lpValueName,
                      LPDWORD lpcchValueName, LPDWORD lpReserved,
                      LPDWORD lpType, LPBYTE lpData, LPDWORD lpcbData);
*/
/*
LSTATUS RegQueryInfoKeyW(HKEY hKey, LPWSTR lpClass, LPDWORD lpcchClass,
                         LPDWORD lpReserved, LPDWORD lpcSubKeys,
                         LPDWORD lpcbMaxSubKeyLen, LPDWORD lpcbMaxClassLen,
                         LPDWORD lpcValues, LPDWORD lpcbMaxValueNameLen,
                         LPDWORD lpcbMaxValueLen, LPDWORD lpcbSecurityDescriptor,
                         PFILETIME lpftLastWriteTime);
*/
