use super::finish;
use super::key_path;
use super::stack_arg;
use crate::emu;
use crate::windows::constants::errors::*;
use crate::windows::registry_store as store;

pub(super) fn query_info_key(emu: &mut emu::Emu, api: &str) {
    let hkey = emu.regs().rcx;
    let class_ptr = emu.regs().rdx;
    let class_cch_ptr = emu.regs().r8;
    let path = match key_path(hkey) {
        Ok(path) => path,
        Err(err) => return finish(emu, api, "", err),
    };
    let Some(info) = store::key_info(&path) else {
        return finish(emu, api, &path, ERROR_KEY_DELETED);
    };
    // keys carry no class
    if class_ptr != 0 && class_cch_ptr != 0 {
        emu.maps.write_word(class_ptr, 0);
    }
    if class_cch_ptr != 0 {
        emu.maps.write_dword(class_cch_ptr, 0);
    }
    let fields = [
        (4, info.subkeys),
        (5, info.max_subkey_len),
        (6, 0),
        (7, info.values),
        (8, info.max_value_name_len),
        (9, info.max_value_len),
        (10, 0),
    ];
    for (index, value) in fields {
        let ptr = stack_arg(emu, index);
        if ptr != 0 {
            emu.maps.write_dword(ptr, value);
        }
    }
    let time_ptr = stack_arg(emu, 11);
    if time_ptr != 0 {
        emu.maps.write_qword(time_ptr, 0);
    }
    finish(emu, api, &path, ERROR_SUCCESS);
}
