use super::RRF_RT_ANY;
use super::data_out;
use super::finish;
use super::key_path;
use super::read_name;
use super::stack_arg;
use super::type_flag;
use super::write_data;
use crate::emu;
use crate::windows::constants::errors::*;
use crate::windows::registry_store as store;

pub(super) fn get_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let sub = read_name(emu, emu.regs().rdx, wide).unwrap_or_default();
    let name = read_name(emu, emu.regs().r8, wide).unwrap_or_default();
    let flags = emu.regs().r9 as u32;
    let type_ptr = stack_arg(emu, 4);
    let data_ptr = stack_arg(emu, 5);
    let size_ptr = stack_arg(emu, 6);

    let base = match key_path(hkey) {
        Ok(base) => base,
        Err(err) => return finish(emu, api, &name, err),
    };
    let path = store::join(&base, &sub);
    let what = format!("{} `{}`", path, name);
    if !store::key_exists(&path) {
        return finish(emu, api, &what, ERROR_FILE_NOT_FOUND);
    }
    let Some(value) = store::get_value(&path, &name) else {
        return finish(emu, api, &what, ERROR_FILE_NOT_FOUND);
    };
    if flags & RRF_RT_ANY & type_flag(value.kind) == 0 {
        return finish(emu, api, &what, ERROR_UNSUPPORTED_TYPE);
    }
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, value.kind);
    }
    let err = write_data(emu, data_ptr, size_ptr, &data_out(&value, wide));
    finish(emu, api, &what, err);
}
