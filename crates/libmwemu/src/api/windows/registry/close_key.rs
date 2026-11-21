use super::finish;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::registry_store as store;
use crate::windows::constants::errors::*;

pub(super) fn close_key(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let slots = 1;
    let hkey = api_try!(emu, call.arg(emu, 0));
    let err = if store::predefined_path(hkey).is_some()
        || emu.handle_management.close_registry_key(hkey)
    {
        ERROR_SUCCESS
    } else {
        ERROR_INVALID_HANDLE
    };
    finish(emu, slots, "RegCloseKey", &format!("0x{:x}", hkey), err);
}
