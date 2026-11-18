use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn RegSetValueExA(emu: &mut emu::Emu) {
    reg_set_value_ex(emu, "RegSetValueExA", false);
}

fn reg_set_value_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 1));
    let _reserved = api_try!(emu, call.arg(emu, 2));
    let reg_type = api_try!(emu, call.arg(emu, 3));
    let data_ptr = api_try!(emu, call.arg(emu, 4));
    let data_len = api_try!(emu, call.arg(emu, 5));
    let name = strings::read_text(emu, name_ptr, wide);
    let value = if reg_type == 1 || reg_type == 2 {
        strings::read_text(emu, data_ptr, wide)
    } else {
        format!("0x{:x}", data_ptr)
    };
    log_red!(
        emu,
        "kernelbase!{api} hkey:0x{:x} '{}' type:{} value:'{}' len:{}",
        hkey,
        name,
        reg_type,
        value,
        data_len
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}

pub fn RegSetValueExW(emu: &mut emu::Emu) {
    reg_set_value_ex(emu, "RegSetValueExW", true);
}
