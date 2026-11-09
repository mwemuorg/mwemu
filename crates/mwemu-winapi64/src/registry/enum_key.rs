use super::finish;
use super::key_path;
use super::stack_arg;
use super::write_name;
use crate::emu;
use crate::windows::constants::errors::*;
use crate::windows::registry_store as store;

pub(super) fn enum_key(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let index = emu.regs().rdx as u32 as usize;
    let name_ptr = emu.regs().r8;
    let cch_ptr = emu.regs().r9;
    let class_cch_ptr = stack_arg(emu, 6);
    let time_ptr = stack_arg(emu, 7);

    let path = match key_path(hkey) {
        Ok(path) => path,
        Err(err) => return finish(emu, api, "", err),
    };
    let what = format!("{} #{}", path, index);
    let Some(name) = store::subkeys(&path).into_iter().nth(index) else {
        return finish(emu, api, &what, ERROR_NO_MORE_ITEMS);
    };
    let err = write_name(emu, name_ptr, cch_ptr, &name, wide);
    if err == ERROR_SUCCESS {
        // keys carry no class
        if class_cch_ptr != 0 {
            emu.maps.write_dword(class_cch_ptr, 0);
        }
        if time_ptr != 0 {
            emu.maps.write_qword(time_ptr, 0);
        }
    }
    finish(emu, api, &format!("{} = {}", what, name), err);
}
