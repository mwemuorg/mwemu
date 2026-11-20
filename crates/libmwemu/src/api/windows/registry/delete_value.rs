use super::finish;
use super::key_path;
use super::read_name;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::registry_store::join;
use crate::windows::constants::errors::*;

pub(super) fn delete_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    delete_value_at(emu, api, wide, false);
}

fn delete_value_at(emu: &mut emu::Emu, api: &str, wide: bool, has_subkey: bool) {
    let call = WinApi(Stdcall);
    let slots = if has_subkey { 3 } else { 2 };
    let hkey = api_try!(emu, call.arg(emu, 0));
    let subkey = if has_subkey {
        match read_name(emu, api_try!(emu, call.arg(emu, 1)), wide) {
            Ok(value) => value.unwrap_or_default(),
            Err(e) => return finish(emu, slots, api, "", e),
        }
    } else {
        String::new()
    };
    let name = match read_name(emu, api_try!(emu, call.arg(emu, slots - 1)), wide) {
        Ok(value) => value.unwrap_or_default(),
        Err(error) => return finish(emu, slots, api, "", error),
    };
    let path = match key_path(emu, hkey) {
        Ok(path) => join(&path, &subkey),
        Err(err) => return finish(emu, slots, api, &name, err),
    };
    let err = if emu.registry.delete_value(&path, &name) {
        ERROR_SUCCESS
    } else {
        ERROR_FILE_NOT_FOUND
    };
    finish(emu, slots, api, &format!("{} `{}`", path, name), err);
}

pub fn RegDeleteKeyValueA(emu: &mut emu::Emu) {
    delete_value_at(emu, "RegDeleteKeyValueA", false, true);
}
pub fn RegDeleteKeyValueW(emu: &mut emu::Emu) {
    delete_value_at(emu, "RegDeleteKeyValueW", true, true);
}
