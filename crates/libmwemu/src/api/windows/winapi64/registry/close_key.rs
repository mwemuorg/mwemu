use super::HANDLE_PREFIX;
use super::finish;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::winapi::registry_store as store;
use crate::windows::constants::errors::*;

pub(super) fn close_key(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
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
