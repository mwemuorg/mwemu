use super::REG_CREATED_NEW_KEY;
use super::REG_OPENED_EXISTING_KEY;
use super::finish;
use super::key_path;
use super::read_name;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::memory::{writable_pointer, write_pointer};
use crate::winapi::policy::api_try;
use crate::winapi::registry::create_path;
use crate::winapi::registry_store as store;
use crate::windows::constants::errors::*;

pub(super) fn create_key(emu: &mut emu::Emu, api: &str, wide: bool, ex: bool) {
    let call = WinApi(Stdcall);
    let slots = if ex { 9 } else { 3 };
    let hkey = api_try!(emu, call.arg(emu, 0));
    let sub = match read_name(emu, api_try!(emu, call.arg(emu, 1)), wide) {
        Ok(value) => value.unwrap_or_default(),
        Err(error) => return finish(emu, slots, api, "", error),
    };
    let (result_ptr, disposition_ptr) = if ex {
        (
            api_try!(emu, call.arg(emu, 7)),
            api_try!(emu, call.arg(emu, 8)),
        )
    } else {
        (api_try!(emu, call.arg(emu, 2)), 0)
    };

    let base = match key_path(emu, hkey) {
        Ok(base) => base,
        Err(err) => return finish(emu, slots, api, &sub, err),
    };
    let path = store::join(&base, &sub);
    if result_ptr == 0
        || !writable_pointer(emu, result_ptr)
        || (disposition_ptr != 0 && !emu.maps.validate_write_range(disposition_ptr, 4))
    {
        return finish(emu, slots, api, &path, ERROR_INVALID_PARAMETER);
    }
    let Some(created) = create_path(emu, &path) else {
        return finish(emu, slots, api, &path, ERROR_ACCESS_DENIED);
    };
    let handle = emu.handle_management.open_registry_key(path.clone());
    write_pointer(emu, result_ptr, handle as u64);
    if disposition_ptr != 0 {
        let disposition = if created {
            REG_CREATED_NEW_KEY
        } else {
            REG_OPENED_EXISTING_KEY
        };
        emu.maps.write_dword(disposition_ptr, disposition);
    }
    let what = format!("{} created: {}", path, created);
    finish(emu, slots, api, &what, ERROR_SUCCESS);
}
