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
    let value_type = api_try!(emu, call.arg(emu, 3));
    let data_ptr = api_try!(emu, call.arg(emu, 4));
    let data_size = api_try!(emu, call.arg(emu, 5));
    let name = strings::read_text(emu, name_ptr, wide);
    let data_str = if value_type == 1 || value_type == 2 {
        strings::read_text(emu, data_ptr, wide)
    } else {
        format!("binary[{}]", data_size)
    };
    log_red!(
        emu,
        "advapi32!{api} hkey:0x{:x} '{}' type:{} data:'{}'",
        hkey,
        name,
        value_type,
        data_str
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}

pub fn RegSetValueExW(emu: &mut emu::Emu) {
    reg_set_value_ex(emu, "RegSetValueExW", true);
}
