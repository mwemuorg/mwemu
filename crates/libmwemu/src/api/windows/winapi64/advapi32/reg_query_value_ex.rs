use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn RegQueryValueExA(emu: &mut emu::Emu) {
    reg_query_value_ex(emu, "RegQueryValueExA", false);
}

pub fn RegQueryValueExW(emu: &mut emu::Emu) {
    reg_query_value_ex(emu, "RegQueryValueExW", true);
}

fn reg_query_value_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _hkey = api_try!(emu, call.arg(emu, 0));
    let value_ptr = api_try!(emu, call.arg(emu, 1));
    let _reserved = api_try!(emu, call.arg(emu, 2));
    let type_ptr = api_try!(emu, call.arg(emu, 3));
    let data_ptr = api_try!(emu, call.arg(emu, 4));
    let data_size_ptr = api_try!(emu, call.arg(emu, 5));
    let value = if value_ptr != 0 {
        strings::read_text(emu, value_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "advapi32!{api} `{}`", value);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    if data_ptr != 0 {
        strings::write_string(emu, data_ptr, "", wide);
    }
    if wide {
        if data_size_ptr != 0 {
            emu.maps.write_dword(data_size_ptr, 2);
        }
    } else {
        if data_size_ptr != 0 {
            emu.maps.write_dword(data_size_ptr, 1);
        }
    }

    call.set_return(emu, 0);
}
