use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn RegOpenKeyA(emu: &mut emu::Emu) {
    reg_open_key(emu, "RegOpenKeyA", false);
}

fn reg_open_key(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hKey = api_try!(emu, call.arg(emu, 0));
    let subkey_ptr = api_try!(emu, call.arg(emu, 1));
    let result = api_try!(emu, call.arg(emu, 2));

    let subkey = strings::read_text(emu, subkey_ptr, wide);
    emu.maps.write_dword(
        result,
        helper::handler_create(&format!("key://{}", subkey)) as u32,
    );

    log_red!(emu, "kernel32!{api} `{}`", subkey);
    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn RegOpenKeyW(emu: &mut emu::Emu) {
    reg_open_key(emu, "RegOpenKeyW", true);
}
