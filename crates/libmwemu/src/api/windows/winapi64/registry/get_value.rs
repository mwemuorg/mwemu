use super::RRF_RT_ANY;
use super::data_out;
use super::finish;
use super::key_path;
use super::read_name;
use super::type_flag;
use super::write_data;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::registry_store as store;
use crate::windows::constants::errors::*;

pub(super) fn get_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let sub = read_name(emu, api_try!(emu, call.arg(emu, 1)), wide).unwrap_or_default();
    let name = read_name(emu, api_try!(emu, call.arg(emu, 2)), wide).unwrap_or_default();
    let flags = api_try!(emu, call.arg_u32(emu, 3));
    let type_ptr = api_try!(emu, call.arg(emu, 4));
    let data_ptr = api_try!(emu, call.arg(emu, 5));
    let size_ptr = api_try!(emu, call.arg(emu, 6));

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
