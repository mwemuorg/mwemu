use super::REG_CREATED_NEW_KEY;
use super::REG_OPENED_EXISTING_KEY;
use super::finish;
use super::key_path;
use super::open_handle;
use super::read_name;
use super::stack_arg;
use crate::emu;
use crate::winapi::registry_store as store;
use crate::windows::constants::ERROR_ACCESS_DENIED;
use crate::windows::constants::ERROR_INVALID_PARAMETER;
use crate::windows::constants::ERROR_SUCCESS;

pub(super) fn create_key(emu: &mut emu::Emu, api: &str, wide: bool, ex: bool) {
    let hkey = emu.regs().rcx;
    let sub = read_name(emu, emu.regs().rdx, wide).unwrap_or_default();
    let (result_ptr, disposition_ptr) = if ex {
        (stack_arg(emu, 7), stack_arg(emu, 8))
    } else {
        (emu.regs().r8, 0)
    };

    let base = match key_path(hkey) {
        Ok(base) => base,
        Err(err) => return finish(emu, api, &sub, err),
    };
    let path = store::join(&base, &sub);
    if result_ptr == 0 {
        return finish(emu, api, &path, ERROR_INVALID_PARAMETER);
    }
    let Some(created) = store::create_key(&path) else {
        return finish(emu, api, &path, ERROR_ACCESS_DENIED);
    };
    emu.maps.write_qword(result_ptr, open_handle(&path));
    if disposition_ptr != 0 {
        let disposition = if created {
            REG_CREATED_NEW_KEY
        } else {
            REG_OPENED_EXISTING_KEY
        };
        emu.maps.write_dword(disposition_ptr, disposition);
    }
    let what = format!("{} created: {}", path, created);
    finish(emu, api, &what, ERROR_SUCCESS);
}
