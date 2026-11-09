use super::finish;
use super::key_path;
use super::open_handle;
use super::read_name;
use super::stack_arg;
use crate::emu;
use crate::windows::constants::errors::*;
use crate::windows::registry_store as store;

pub(super) fn open_key(emu: &mut emu::Emu, api: &str, wide: bool, ex: bool) {
    let hkey = emu.regs().rcx;
    let sub = read_name(emu, emu.regs().rdx, wide).unwrap_or_default();
    let result_ptr = if ex { stack_arg(emu, 4) } else { emu.regs().r8 };

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
