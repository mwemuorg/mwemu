use super::data_out;
use super::finish;
use super::key_path;
use super::read_name;
use super::write_data;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::registry_store::{REG_SZ, join};
use crate::windows::constants::errors::*;

pub(super) fn query_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let slots = 6;
    let hkey = api_try!(emu, call.arg(emu, 0));
    let name = match read_name(emu, api_try!(emu, call.arg(emu, 1)), wide) {
        Ok(value) => value.unwrap_or_default(),
        Err(error) => return finish(emu, slots, api, "", error),
    };
    let type_ptr = api_try!(emu, call.arg(emu, 3));
    let data_ptr = api_try!(emu, call.arg(emu, 4));
    let size_ptr = api_try!(emu, call.arg(emu, 5));

    let path = match key_path(emu, hkey) {
        Ok(path) => path,
        Err(err) => return finish(emu, slots, api, &name, err),
    };
    let what = format!("{} `{}`", path, name);
    let Some(value) = emu.registry.get_value(&path, &name) else {
        return finish(emu, slots, api, &what, ERROR_FILE_NOT_FOUND);
    };
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, value.kind);
    }
    let err = write_data(emu, data_ptr, size_ptr, &data_out(&value, wide));
    finish(emu, slots, api, &what, err);
}

fn query_default(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let [key, sub, output, size] = api_try!(emu, call.args(emu));
    let result = (|| {
        let parent = key_path(emu, key)?;
        let path = join(&parent, &read_name(emu, sub, wide)?.unwrap_or_default());
        if !emu.registry.key_exists(&path) {
            return Err(ERROR_FILE_NOT_FOUND);
        }
        let data = match emu.registry.get_value(&path, "") {
            Some(v) if v.kind == REG_SZ => data_out(&v, wide),
            Some(_) => return Err(ERROR_INVALID_DATA),
            None => vec![0; if wide { 2 } else { 1 }],
        };
        Ok(write_data(emu, output, size, &data))
    })();
    finish(emu, 4, api, "", result.unwrap_or_else(|error| error));
}
pub fn RegQueryValueA(emu: &mut emu::Emu) {
    query_default(emu, "RegQueryValueA", false);
}
pub fn RegQueryValueW(emu: &mut emu::Emu) {
    query_default(emu, "RegQueryValueW", true);
}
