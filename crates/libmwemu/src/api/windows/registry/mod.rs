//! Guest registry operations shared by Windows library gateways.
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::dispatch_api;
use crate::emu;
use crate::winapi::environment_store::Encoding;
use crate::winapi::registry_store::{self as store, Value};
use crate::windows::constants::errors::*;

mod close_key;
mod create_key;
mod delete_key;
mod delete_value;
mod enum_key;
mod enum_value;
mod flush_key;
mod get_value;
pub(crate) mod native;
mod open_key;
mod query_info_key;
mod query_value;
mod remote;
mod set_value;

use close_key::close_key;
use create_key::create_key;
use delete_key::delete_key;
use delete_value::delete_value;
pub use delete_value::{RegDeleteKeyValueA, RegDeleteKeyValueW};
use enum_key::enum_key;
use enum_value::enum_value;
use flush_key::flush_key;
use get_value::get_value;
pub use open_key::RegOpenCurrentUser;
pub(crate) use open_key::open_current_user;
use open_key::open_key;
use query_info_key::query_info_key;
use query_value::query_value;
pub use remote::{
    BaseRegCloseKey, BaseRegDeleteKeyEx, BaseRegDeleteValue, BaseRegFlushKey, BaseRegOpenKey,
    RegConnectRegistryA, RegConnectRegistryW,
};
use set_value::set_value;
pub use set_value::{RegSetKeyValueA, RegSetKeyValueW, RegSetValueA, RegSetValueW};

const REG_CREATED_NEW_KEY: u32 = 1;
const REG_OPENED_EXISTING_KEY: u32 = 2;

/// Run `api` if it is a registry API; false otherwise.
pub fn gateway(api: &str, emu: &mut emu::Emu) -> bool {
    dispatch_api!(api, {
        "RegCloseKey" => RegCloseKey(emu),
        "RegCreateKeyA" => RegCreateKeyA(emu),
        "RegCreateKeyExA" => RegCreateKeyExA(emu),
        "RegCreateKeyExW" => RegCreateKeyExW(emu),
        "RegCreateKeyW" => RegCreateKeyW(emu),
        "RegDeleteKeyA" => RegDeleteKeyA(emu),
        "RegDeleteKeyExA" => RegDeleteKeyExA(emu),
        "RegDeleteKeyExW" => RegDeleteKeyExW(emu),
        "RegDeleteKeyW" => RegDeleteKeyW(emu),
        "RegDeleteTreeA" => RegDeleteTreeA(emu),
        "RegDeleteTreeW" => RegDeleteTreeW(emu),
        "RegDeleteValueA" => RegDeleteValueA(emu),
        "RegDeleteValueW" => RegDeleteValueW(emu),
        "RegEnumKeyExA" => RegEnumKeyExA(emu),
        "RegEnumKeyExW" => RegEnumKeyExW(emu),
        "RegEnumValueA" => RegEnumValueA(emu),
        "RegEnumValueW" => RegEnumValueW(emu),
        "RegFlushKey" => RegFlushKey(emu),
        "RegGetValueA" => RegGetValueA(emu),
        "RegGetValueW" => RegGetValueW(emu),
        "RegOpenCurrentUser" => RegOpenCurrentUser(emu),
        "RegOpenKeyA" => RegOpenKeyA(emu),
        "RegOpenKeyExA" => RegOpenKeyExA(emu),
        "RegOpenKeyExW" => RegOpenKeyExW(emu),
        "RegOpenKeyW" => RegOpenKeyW(emu),
        "RegQueryInfoKeyA" => RegQueryInfoKeyA(emu),
        "RegQueryInfoKeyW" => RegQueryInfoKeyW(emu),
        "RegQueryValueExA" => RegQueryValueExA(emu),
        "RegQueryValueExW" => RegQueryValueExW(emu),
        "RegSetValueExA" => RegSetValueExA(emu),
        "RegSetValueExW" => RegSetValueExW(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

pub(crate) fn kernel_gateway(api: &str, emu: &mut emu::Emu) -> bool {
    !matches!(
        api,
        "RegCreateKeyA"
            | "RegCreateKeyW"
            | "RegDeleteKeyA"
            | "RegDeleteKeyW"
            | "RegOpenKeyA"
            | "RegOpenKeyW"
    ) && gateway(api, emu)
}

pub fn RegCloseKey(emu: &mut emu::Emu) {
    close_key(emu);
}

pub fn RegCreateKeyA(emu: &mut emu::Emu) {
    create_key(emu, "RegCreateKeyA", false, false);
}

pub fn RegCreateKeyExA(emu: &mut emu::Emu) {
    create_key(emu, "RegCreateKeyExA", false, true);
}

pub fn RegCreateKeyExW(emu: &mut emu::Emu) {
    create_key(emu, "RegCreateKeyExW", true, true);
}

pub fn RegCreateKeyW(emu: &mut emu::Emu) {
    create_key(emu, "RegCreateKeyW", true, false);
}

pub fn RegDeleteKeyA(emu: &mut emu::Emu) {
    delete_key(emu, "RegDeleteKeyA", false, false);
}

pub fn RegDeleteKeyExA(emu: &mut emu::Emu) {
    delete_key(emu, "RegDeleteKeyExA", false, false);
}

pub fn RegDeleteKeyExW(emu: &mut emu::Emu) {
    delete_key(emu, "RegDeleteKeyExW", true, false);
}

pub fn RegDeleteKeyW(emu: &mut emu::Emu) {
    delete_key(emu, "RegDeleteKeyW", true, false);
}

pub fn RegDeleteTreeA(emu: &mut emu::Emu) {
    delete_key(emu, "RegDeleteTreeA", false, true);
}

pub fn RegDeleteTreeW(emu: &mut emu::Emu) {
    delete_key(emu, "RegDeleteTreeW", true, true);
}

pub fn RegDeleteValueA(emu: &mut emu::Emu) {
    delete_value(emu, "RegDeleteValueA", false);
}

pub fn RegDeleteValueW(emu: &mut emu::Emu) {
    delete_value(emu, "RegDeleteValueW", true);
}

pub fn RegEnumKeyExA(emu: &mut emu::Emu) {
    enum_key(emu, "RegEnumKeyExA", false);
}

pub fn RegEnumKeyExW(emu: &mut emu::Emu) {
    enum_key(emu, "RegEnumKeyExW", true);
}

pub fn RegEnumValueA(emu: &mut emu::Emu) {
    enum_value(emu, "RegEnumValueA", false);
}

pub fn RegEnumValueW(emu: &mut emu::Emu) {
    enum_value(emu, "RegEnumValueW", true);
}

pub fn RegFlushKey(emu: &mut emu::Emu) {
    flush_key(emu);
}

pub fn RegGetValueA(emu: &mut emu::Emu) {
    get_value(emu, "RegGetValueA", false);
}

pub fn RegGetValueW(emu: &mut emu::Emu) {
    get_value(emu, "RegGetValueW", true);
}

pub fn RegOpenKeyA(emu: &mut emu::Emu) {
    open_key(emu, "RegOpenKeyA", false, false);
}

pub fn RegOpenKeyExA(emu: &mut emu::Emu) {
    open_key(emu, "RegOpenKeyExA", false, true);
}

pub fn RegOpenKeyExW(emu: &mut emu::Emu) {
    open_key(emu, "RegOpenKeyExW", true, true);
}

pub fn RegOpenKeyW(emu: &mut emu::Emu) {
    open_key(emu, "RegOpenKeyW", true, false);
}

pub fn RegQueryInfoKeyA(emu: &mut emu::Emu) {
    query_info_key(emu, "RegQueryInfoKeyA");
}

pub fn RegQueryInfoKeyW(emu: &mut emu::Emu) {
    query_info_key(emu, "RegQueryInfoKeyW");
}

pub fn RegQueryValueExA(emu: &mut emu::Emu) {
    query_value(emu, "RegQueryValueExA", false);
}

pub fn RegQueryValueExW(emu: &mut emu::Emu) {
    query_value(emu, "RegQueryValueExW", true);
}

pub fn RegSetValueExA(emu: &mut emu::Emu) {
    set_value(emu, "RegSetValueExA", false);
}

pub fn RegSetValueExW(emu: &mut emu::Emu) {
    set_value(emu, "RegSetValueExW", true);
}

fn read_name(emu: &emu::Emu, ptr: u64, wide: bool) -> Result<Option<String>, u64> {
    if ptr == 0 {
        return Ok(None);
    }
    let units = Encoding::from_wide(wide).read(emu, ptr)?;
    String::from_utf16(&units)
        .map(Some)
        .map_err(|_| ERROR_INVALID_PARAMETER)
}

fn key_path(emu: &emu::Emu, hkey: u64) -> Result<String, u64> {
    if let Some(path) = store::predefined_path(hkey) {
        return Ok(path.to_owned());
    }
    let path = emu
        .handle_management
        .registry_key(hkey)
        .ok_or(ERROR_INVALID_HANDLE)?;
    if emu.registry.key_exists(path) {
        Ok(path.to_owned())
    } else {
        Err(ERROR_KEY_DELETED)
    }
}

fn finish(emu: &mut emu::Emu, slots: usize, api: &str, what: &str, err: u64) {
    let call = WinApi(Stdcall);
    log_red!(emu, "advapi32!{} {} = {}", api, what, err);
    call.set_return(emu, err);
    call.cleanup_args(emu, slots);
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
        .flat_map(|b| Encoding::Ansi.decode_byte(*b).to_le_bytes())
        .collect()
}

/// Stored data in the form the caller reads.
fn data_out(value: &Value, wide: bool) -> Vec<u8> {
    if wide || !is_string(value.kind) {
        return value.data.clone();
    }
    let words: Vec<_> = value
        .data
        .as_chunks::<2>()
        .0
        .iter()
        .copied()
        .map(u16::from_le_bytes)
        .collect();
    Encoding::Ansi.encode(&words)
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
    let Some(capacity) = emu.maps.read_dword(size_ptr) else {
        return ERROR_NOACCESS;
    };
    if !emu.maps.write_dword(size_ptr, data.len() as u32) {
        return ERROR_NOACCESS;
    }
    if data_ptr == 0 {
        return ERROR_SUCCESS;
    }
    if data.len() > capacity as usize {
        return ERROR_MORE_DATA;
    }
    if emu.maps.write_bytes(data_ptr, data) {
        ERROR_SUCCESS
    } else {
        ERROR_NOACCESS
    }
}

/// Copy a name to a buffer of `*cch_ptr` characters; on success `*cch_ptr`
/// is the length without the terminator.
fn write_name(emu: &mut emu::Emu, ptr: u64, cch_ptr: u64, name: &str, wide: bool) -> u64 {
    if ptr == 0 || cch_ptr == 0 {
        return ERROR_INVALID_PARAMETER;
    }
    let Some(capacity) = emu.maps.read_dword(cch_ptr) else {
        return ERROR_NOACCESS;
    };
    let encoding = Encoding::from_wide(wide);
    let encoded = encoding.terminated(&name.encode_utf16().collect::<Vec<_>>());
    let units = encoded.len() / encoding.width();
    if units > capacity as usize {
        return ERROR_MORE_DATA;
    }
    if !emu.maps.validate_write_range(cch_ptr, 4) || !emu.maps.write_bytes(ptr, &encoded) {
        return ERROR_NOACCESS;
    }
    emu.maps.write_dword(cch_ptr, (units - 1) as u32);
    ERROR_SUCCESS
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

pub use enum_key::{RegEnumKeyA, RegEnumKeyW};
pub use query_value::{RegQueryValueA, RegQueryValueW};
