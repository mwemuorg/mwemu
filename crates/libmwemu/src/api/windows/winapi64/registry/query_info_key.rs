use super::finish;
use super::key_path;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::registry_store as store;
use crate::windows::constants::errors::*;

pub(super) fn query_info_key(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let [
        hkey,
        class_ptr,
        class_cch_ptr,
        _,
        subkeys,
        max_subkey,
        max_class,
        values,
        max_value_name,
        max_value,
        security_size,
        time_ptr,
    ] = api_try!(emu, call.args(emu));
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
        (subkeys, info.subkeys),
        (max_subkey, info.max_subkey_len),
        (max_class, 0),
        (values, info.values),
        (max_value_name, info.max_value_name_len),
        (max_value, info.max_value_len),
        (security_size, 0),
    ];
    for (ptr, value) in fields {
        if ptr != 0 {
            emu.maps.write_dword(ptr, value);
        }
    }
    if time_ptr != 0 {
        emu.maps.write_qword(time_ptr, 0);
    }
    finish(emu, api, &path, ERROR_SUCCESS);
}
