use super::data_in;
use super::finish;
use super::key_path;
use super::read_name;
use super::stack_arg;
use crate::emu;
use crate::winapi::registry_store as store;
use crate::winapi::registry_store::Value;
use crate::windows::constants::errors::*;

pub(super) fn set_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let name = read_name(emu, emu.regs().rdx, wide).unwrap_or_default();
    let kind = emu.regs().r9 as u32;
    let data_ptr = stack_arg(emu, 4);
    let size = stack_arg(emu, 5) as u32 as usize;

    let path = match key_path(hkey) {
        Ok(path) => path,
        Err(err) => return finish(emu, api, &name, err),
    };
    let data = if size == 0 {
        Vec::new()
    } else {
        match emu.maps.try_read_bytes(data_ptr, size) {
            Some(bytes) => bytes.to_vec(),
            None => return finish(emu, api, &name, ERROR_INVALID_PARAMETER),
        }
    };
    let value = Value {
        kind,
        data: data_in(kind, &data, wide),
    };
    store::set_value(&path, &name, value);
    let what = format!("{} `{}` type {} ({} bytes)", path, name, kind, size);
    finish(emu, api, &what, ERROR_SUCCESS);
}
