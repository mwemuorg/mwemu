use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn RegQueryValueExA(emu: &mut emu::Emu) {
    reg_query_value_ex(emu, "RegQueryValueExA", false);
}

pub(super) fn RegQueryValueExW(emu: &mut emu::Emu) {
    reg_query_value_ex(emu, "RegQueryValueExW", true);
}

fn reg_query_value_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let value_ptr = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let type_ptr = api_try!(emu, call.arg_u32(emu, 3)) as u64;
    let data_ptr = api_try!(emu, call.arg_u32(emu, 4)) as u64;
    let data_size_ptr = api_try!(emu, call.arg_u32(emu, 5)) as u64;
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

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 6);
}
