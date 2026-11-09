use super::data_out;
use super::finish;
use super::key_path;
use super::read_name;
use super::stack_arg;
use super::write_data;
use crate::emu;
use crate::windows::constants::errors::*;
use crate::windows::registry_store as store;

pub(super) fn query_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let name = read_name(emu, emu.regs().rdx, wide).unwrap_or_default();
    let type_ptr = emu.regs().r9;
    let data_ptr = stack_arg(emu, 4);
    let size_ptr = stack_arg(emu, 5);

    let path = match key_path(hkey) {
        Ok(path) => path,
        Err(err) => return finish(emu, api, &name, err),
    };
    let what = format!("{} `{}`", path, name);
    let Some(value) = store::get_value(&path, &name) else {
        return finish(emu, api, &what, ERROR_FILE_NOT_FOUND);
    };
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, value.kind);
    }
    let err = write_data(emu, data_ptr, size_ptr, &data_out(&value, wide));
    finish(emu, api, &what, err);
}
