use super::HANDLE_PREFIX;
use super::finish;
use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::errors::*;
use crate::windows::registry_store as store;

pub(super) fn close_key(emu: &mut emu::Emu) {
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
