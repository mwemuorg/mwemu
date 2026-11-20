use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory::{pointer_size, read_pointer, writable_pointer, write_pointer};
use crate::winapi::common::ntdll::read_unicode_string;
use crate::winapi::common::trace;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::winapi::registry_store::join;
use crate::winapi::registry_store::{
    HKEY_LOCAL_MACHINE, HKEY_PERFORMANCE_DATA, HKEY_USERS, predefined_path,
};
use crate::windows::constants::errors::*;
use crate::windows::constants::security::{KEY_WOW64_32KEY, KEY_WOW64_64KEY};

use super::delete_key::delete_subkey;

// RPC_HKEY identifies an open key. The guest registry stands in for the RPC server.
fn open_path(emu: &Emu, hkey: u64) -> Result<&str, u64> {
    let path = emu
        .handle_management
        .registry_key(hkey)
        .ok_or(ERROR_INVALID_HANDLE)?;
    if emu.registry.key_exists(path) {
        Ok(path)
    } else {
        Err(ERROR_KEY_DELETED)
    }
}

fn read_rrp_name(emu: &Emu, descriptor: u64) -> Result<String, u64> {
    if descriptor == 0 {
        return Err(ERROR_INVALID_PARAMETER);
    }
    let mut name = read_unicode_string(emu, descriptor).map_err(|status| match status {
        STATUS_ACCESS_VIOLATION => ERROR_NOACCESS,
        _ => ERROR_INVALID_PARAMETER,
    })?;
    // Some clients include the required terminator in Length; others reserve it in MaximumLength.
    if !name.ends_with('\0') {
        let length = emu.maps.read_word(descriptor).ok_or(ERROR_NOACCESS)?;
        let maximum = emu.maps.read_word(descriptor + 2).ok_or(ERROR_NOACCESS)?;
        if maximum - length < size_of::<u16>() as u16 {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let buffer = read_pointer(emu, descriptor + pointer_size(emu)).ok_or(ERROR_NOACCESS)?;
        let end = buffer.checked_add(length as u64).ok_or(ERROR_NOACCESS)?;
        if buffer == 0 || emu.maps.read_word(end).ok_or(ERROR_NOACCESS)? != 0 {
            return Err(ERROR_INVALID_PARAMETER);
        }
    } else {
        name.pop();
    }
    if name.contains('\0') {
        return Err(ERROR_INVALID_PARAMETER);
    }
    Ok(name)
}

fn check_view(access: u32) -> Result<(), u64> {
    let views = KEY_WOW64_32KEY | KEY_WOW64_64KEY;
    match access & views {
        0 => Ok(()),
        value if value == views => Err(ERROR_INVALID_PARAMETER),
        // The guest registry currently stores one view.
        _ => Err(ERROR_NOT_SUPPORTED),
    }
}

pub fn BaseRegOpenKey(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let subkey = api_try!(emu, call.arg(emu, 1));
    let options = api_try!(emu, call.arg_u32(emu, 2));
    let desired_access = api_try!(emu, call.arg_u32(emu, 3));
    let result_ptr = api_try!(emu, call.arg(emu, 4));
    let name = read_rrp_name(emu, subkey);
    log_red!(
        emu,
        "advapi32!BaseRegOpenKey hkey={hkey:#x} subkey={:?} options={options:#x} desired_access={desired_access:#x} result_ptr={result_ptr:#x}",
        name.as_deref().unwrap_or("<invalid>")
    );
    let result = (|| {
        if result_ptr == 0 || !writable_pointer(emu, result_ptr) {
            return Err(ERROR_NOACCESS);
        }
        let base = open_path(emu, hkey)?;
        check_view(desired_access)?;
        if options != 0 {
            return Err(ERROR_NOT_SUPPORTED);
        }
        let path = join(base, &name?);
        if !emu.registry.key_exists(&path) {
            return Err(ERROR_FILE_NOT_FOUND);
        }
        Ok(emu
            .handle_management
            .open_registry_key_access(path, desired_access))
    })();
    if result_ptr != 0 && writable_pointer(emu, result_ptr) {
        write_pointer(emu, result_ptr, result.unwrap_or(0).into());
    }
    call.set_return(emu, result.err().unwrap_or(ERROR_SUCCESS));
    call.cleanup_args(emu, 5);
}

pub fn BaseRegDeleteKeyEx(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let subkey = api_try!(emu, call.arg(emu, 1));
    let access_mask = api_try!(emu, call.arg_u32(emu, 2));
    let reserved = api_try!(emu, call.arg_u32(emu, 3));
    let name = read_rrp_name(emu, subkey);
    log_red!(
        emu,
        "advapi32!BaseRegDeleteKeyEx hkey={hkey:#x} subkey={:?} subkey_ptr={subkey:#x} access_mask={access_mask:#x} reserved={reserved:#x}",
        name.as_deref().unwrap_or("<invalid>")
    );
    let status = open_path(emu, hkey).map(str::to_owned).and_then(|base| {
        check_view(access_mask)?;
        let path = join(&base, &name?);
        if path == base {
            return Err(ERROR_INVALID_PARAMETER);
        }
        delete_subkey(emu, &path, false)
    });
    call.set_return(emu, status.err().unwrap_or(ERROR_SUCCESS));
    call.cleanup_args(emu, 4);
}

pub fn BaseRegDeleteValue(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [hkey, value_name] = api_try!(emu, call.args(emu));
    let name = read_rrp_name(emu, value_name);
    log_red!(
        emu,
        "advapi32!BaseRegDeleteValue hkey={hkey:#x} value_name={:?} value_name_ptr={value_name:#x}",
        name.as_deref().unwrap_or("<invalid>")
    );
    let status = open_path(emu, hkey).map(str::to_owned).and_then(|path| {
        emu.registry
            .delete_value(&path, &name?)
            .then_some(())
            .ok_or(ERROR_FILE_NOT_FOUND)
    });
    call.set_return(emu, status.err().unwrap_or(ERROR_SUCCESS));
    call.cleanup_args(emu, 2);
}

pub fn BaseRegFlushKey(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "advapi32!BaseRegFlushKey hkey={hkey:#x}");
    let status = open_path(emu, hkey).err().unwrap_or(ERROR_SUCCESS);
    call.set_return(emu, status);
    call.cleanup_args(emu, 1);
}

pub fn BaseRegCloseKey(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let key_ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "advapi32!BaseRegCloseKey key_ptr={key_ptr:#x}");
    let result = if key_ptr == 0 || !writable_pointer(emu, key_ptr) {
        ERROR_NOACCESS
    } else if let Some(hkey) = read_pointer(emu, key_ptr) {
        if emu.handle_management.close_registry_key(hkey) {
            write_pointer(emu, key_ptr, 0);
            ERROR_SUCCESS
        } else {
            ERROR_INVALID_HANDLE
        }
    } else {
        ERROR_NOACCESS
    };
    call.set_return(emu, result);
    call.cleanup_args(emu, 1);
}

fn connect_registry(emu: &mut Emu, api: &str, encoding: Encoding) {
    let call = WinApi(Stdcall);
    let [server, hkey, output] = api_try!(emu, call.args(emu));
    trace::export(emu, call, "advapi32", api);
    let local = encoding.local_computer(emu, server);
    if matches!(local, Ok(false))
        || (local == Ok(true) && u64::from(hkey as u32) == HKEY_PERFORMANCE_DATA)
    {
        policy::unsupported(emu, "advapi32", api, emu.pc());
        return;
    }
    let result = local.and_then(|_| {
        if output == 0 || !writable_pointer(emu, output) {
            return Err(ERROR_NOACCESS);
        }
        let root = u64::from(hkey as u32);
        if !matches!(root, HKEY_LOCAL_MACHINE | HKEY_USERS) {
            return Err(ERROR_INVALID_HANDLE);
        }
        let path = predefined_path(root).ok_or(ERROR_INVALID_HANDLE)?;
        let handle = emu.handle_management.open_registry_key(path.to_owned());
        write_pointer(emu, output, u64::from(handle));
        Ok(())
    });
    call.set_return(emu, result.err().unwrap_or(ERROR_SUCCESS));
    call.cleanup_args(emu, 3);
}

pub fn RegConnectRegistryA(emu: &mut Emu) {
    connect_registry(emu, "RegConnectRegistryA", Encoding::Ansi);
}
pub fn RegConnectRegistryW(emu: &mut Emu) {
    connect_registry(emu, "RegConnectRegistryW", Encoding::Wide);
}
