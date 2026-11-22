use super::data_out;
use super::finish;
use super::key_path;
use super::write_data;
use super::write_name;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub(super) fn enum_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let slots = 8;
    let hkey = api_try!(emu, call.arg(emu, 0));
    let index = api_try!(emu, call.arg_u32(emu, 1)) as usize;
    let name_ptr = api_try!(emu, call.arg(emu, 2));
    let cch_ptr = api_try!(emu, call.arg(emu, 3));
    let type_ptr = api_try!(emu, call.arg(emu, 5));
    let data_ptr = api_try!(emu, call.arg(emu, 6));
    let size_ptr = api_try!(emu, call.arg(emu, 7));

    let path = match key_path(emu, hkey) {
        Ok(path) => path,
        Err(err) => return finish(emu, slots, api, "", err),
    };
    let what = format!("{} #{}", path, index);
    let Some((name, value)) = emu.registry.value_at(&path, index) else {
        return finish(emu, slots, api, &what, ERROR_NO_MORE_ITEMS);
    };
    let mut err = write_name(emu, name_ptr, cch_ptr, &name, wide);
    if err == ERROR_SUCCESS {
        if type_ptr != 0 {
            emu.maps.write_dword(type_ptr, value.kind);
        }
        err = write_data(emu, data_ptr, size_ptr, &data_out(&value, wide));
    }
    finish(emu, slots, api, &format!("{} = `{}`", what, name), err);
}
