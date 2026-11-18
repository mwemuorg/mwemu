use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn RegSetValueExA(emu: &mut emu::Emu) {
    reg_set_value_ex(emu, "RegSetValueExA", false);
}

fn reg_set_value_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hKey = api_try!(emu, call.arg(emu, 0));
    let value_name_ptr = api_try!(emu, call.arg(emu, 1));
    let reserved = api_try!(emu, call.arg(emu, 2));
    let value_type = api_try!(emu, call.arg(emu, 3));
    let data_ptr = api_try!(emu, call.arg(emu, 4));
    let data_size = api_try!(emu, call.arg(emu, 5));

    let value_name = strings::read_text(emu, value_name_ptr, wide);

    log_red!(
        emu,
        "kernel32!{api} `{}` type: {} data: 0x{:x}",
        value_name,
        value_type,
        data_ptr
    );
    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 6);
}

pub fn RegSetValueExW(emu: &mut emu::Emu) {
    reg_set_value_ex(emu, "RegSetValueExW", true);
}
