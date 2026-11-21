use super::finish;
use super::key_path;
use super::write_name;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::registry_store as store;
use crate::windows::constants::errors::*;

pub(super) fn enum_key(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let index = api_try!(emu, call.arg_u32(emu, 1)) as usize;
    let name_ptr = api_try!(emu, call.arg(emu, 2));
    let cch_ptr = api_try!(emu, call.arg(emu, 3));
    let class_cch_ptr = api_try!(emu, call.arg(emu, 6));
    let time_ptr = api_try!(emu, call.arg(emu, 7));

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
