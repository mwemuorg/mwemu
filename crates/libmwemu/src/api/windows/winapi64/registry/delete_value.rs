use super::finish;
use super::key_path;
use super::read_name;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::registry_store as store;
use crate::windows::constants::errors::*;

pub(super) fn delete_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let name = read_name(emu, api_try!(emu, call.arg(emu, 1)), wide).unwrap_or_default();
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
