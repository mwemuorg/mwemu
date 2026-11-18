use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn RegCreateKeyExA(emu: &mut emu::Emu) {
    reg_create_key_ex(emu, "RegCreateKeyExA", false);
}

fn reg_create_key_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hKey = api_try!(emu, call.arg(emu, 0));
    let subkey_ptr = api_try!(emu, call.arg(emu, 1));
    let reserved = api_try!(emu, call.arg(emu, 2));
    let class_ptr = api_try!(emu, call.arg(emu, 3));
    let options = api_try!(emu, call.arg(emu, 4));
    let security_attr = api_try!(emu, call.arg(emu, 5));

    let subkey = strings::read_text(emu, subkey_ptr, wide);
    let mut class_name = "".to_string();
    if class_ptr > 0 {
        class_name = strings::read_text(emu, class_ptr, wide);
    }

    log_red!(emu, "kernel32!{api} {} {}", subkey, class_name);
    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 9);
}

pub fn RegCreateKeyExW(emu: &mut emu::Emu) {
    reg_create_key_ex(emu, "RegCreateKeyExW", true);
}
