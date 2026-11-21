use super::finish;
use super::key_path;
use super::read_name;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::memory::{writable_pointer, write_pointer};
use crate::winapi::policy::api_try;
use crate::winapi::registry_store as store;
use crate::windows::constants::errors::*;

pub(super) fn open_key(emu: &mut emu::Emu, api: &str, wide: bool, ex: bool) {
    let call = WinApi(Stdcall);
    let slots = if ex { 5 } else { 3 };
    let hkey = api_try!(emu, call.arg(emu, 0));
    let sub = match read_name(emu, api_try!(emu, call.arg(emu, 1)), wide) {
        Ok(value) => value.unwrap_or_default(),
        Err(error) => return finish(emu, slots, api, "", error),
    };
    let result_ptr = if ex {
        api_try!(emu, call.arg(emu, 4))
    } else {
        api_try!(emu, call.arg(emu, 2))
    };

    let base = match key_path(emu, hkey) {
        Ok(base) => base,
        Err(err) => return finish(emu, slots, api, &sub, err),
    };
    let path = store::join(&base, &sub);
    let err = if result_ptr == 0 || !writable_pointer(emu, result_ptr) {
        ERROR_INVALID_PARAMETER
    } else if !emu.registry.key_exists(&path) {
        ERROR_FILE_NOT_FOUND
    } else {
        let handle = emu.handle_management.open_registry_key(path.clone());
        write_pointer(emu, result_ptr, handle as u64);
        ERROR_SUCCESS
    };
    finish(emu, slots, api, &path, err);
}

pub fn RegOpenCurrentUser(emu: &mut emu::Emu) {
    open_current_user(emu, false);
}

pub(crate) fn open_current_user(emu: &mut emu::Emu, native: bool) {
    let call = WinApi(Stdcall);
    let access = api_try!(emu, call.arg_u32(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let result = if output == 0 || !writable_pointer(emu, output) {
        if native {
            STATUS_ACCESS_VIOLATION
        } else {
            ERROR_INVALID_PARAMETER
        }
    } else {
        let handle = emu
            .handle_management
            .open_registry_key_access("HKEY_CURRENT_USER".into(), access);
        write_pointer(emu, output, handle as u64);
        ERROR_SUCCESS
    };
    log_red!(
        emu,
        "{} access=0x{:x}",
        if native {
            "RtlOpenCurrentUser"
        } else {
            "RegOpenCurrentUser"
        },
        access
    );
    call.set_return(emu, result);
    call.cleanup_args(emu, 2);
}
