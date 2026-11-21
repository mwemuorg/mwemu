use super::finish;
use super::key_path;
use super::open_handle;
use super::read_name;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::registry_store as store;
use crate::windows::constants::errors::*;

pub(super) fn open_key(emu: &mut emu::Emu, api: &str, wide: bool, ex: bool) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let sub = read_name(emu, api_try!(emu, call.arg(emu, 1)), wide).unwrap_or_default();
    let result_ptr = if ex {
        api_try!(emu, call.arg(emu, 4))
    } else {
        api_try!(emu, call.arg(emu, 2))
    };

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
