use super::data_out;
use super::finish;
use super::key_path;
use super::stack_arg;
use super::write_data;
use super::write_name;
use crate::emu;
use crate::winapi::registry_store as store;
use crate::windows::constants::errors::*;

pub(super) fn enum_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let index = emu.regs().rdx as u32 as usize;
    let name_ptr = emu.regs().r8;
    let cch_ptr = emu.regs().r9;
    let type_ptr = stack_arg(emu, 5);
    let data_ptr = stack_arg(emu, 6);
    let size_ptr = stack_arg(emu, 7);

    let path = match key_path(hkey) {
        Ok(path) => path,
        Err(err) => return finish(emu, api, "", err),
    };
    let what = format!("{} #{}", path, index);
    let Some((name, value)) = store::value_at(&path, index) else {
        return finish(emu, api, &what, ERROR_NO_MORE_ITEMS);
    };
    let mut err = write_name(emu, name_ptr, cch_ptr, &name, wide);
    if err == ERROR_SUCCESS {
        if type_ptr != 0 {
            emu.maps.write_dword(type_ptr, value.kind);
        }
        err = write_data(emu, data_ptr, size_ptr, &data_out(&value, wide));
    }
    finish(emu, api, &format!("{} = `{}`", what, name), err);
}
