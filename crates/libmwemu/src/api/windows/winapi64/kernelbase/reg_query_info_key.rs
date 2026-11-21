use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn RegQueryInfoKeyA(emu: &mut emu::Emu) {
    reg_query_info_key(emu, "RegQueryInfoKeyA");
}

fn reg_query_info_key(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let lp_class = api_try!(emu, call.arg(emu, 1));
    let lpcch_class = api_try!(emu, call.arg(emu, 2));
    let _reserved = api_try!(emu, call.arg(emu, 3));
    let lpc_sub_keys = api_try!(emu, call.arg(emu, 4));
    let lpcb_max_sub_key_len = api_try!(emu, call.arg(emu, 5));
    let lpcb_max_class_len = api_try!(emu, call.arg(emu, 6));
    let lpc_values = api_try!(emu, call.arg(emu, 7));
    let lpcb_max_value_name_len = api_try!(emu, call.arg(emu, 8));
    let lpcb_max_value_len = api_try!(emu, call.arg(emu, 9));
    let lpcb_security_desc = api_try!(emu, call.arg(emu, 10));
    let lpft_last_write = api_try!(emu, call.arg(emu, 11));

    log_red!(emu, "kernelbase!{api} hkey:0x{:x}", hkey);

    if lpcch_class != 0 {
        emu.maps.write_dword(lpcch_class, 0);
    }
    if lpc_sub_keys != 0 {
        emu.maps.write_dword(lpc_sub_keys, 0);
    }
    if lpcb_max_sub_key_len != 0 {
        emu.maps.write_dword(lpcb_max_sub_key_len, 0);
    }
    if lpcb_max_class_len != 0 {
        emu.maps.write_dword(lpcb_max_class_len, 0);
    }
    if lpc_values != 0 {
        emu.maps.write_dword(lpc_values, 0);
    }
    if lpcb_max_value_name_len != 0 {
        emu.maps.write_dword(lpcb_max_value_name_len, 0);
    }
    if lpcb_max_value_len != 0 {
        emu.maps.write_dword(lpcb_max_value_len, 0);
    }
    if lpcb_security_desc != 0 {
        emu.maps.write_dword(lpcb_security_desc, 0);
    }
    if lpft_last_write != 0 {
        emu.maps.write_qword(lpft_last_write, 0);
    }
    call.set_return(emu, 0);
}

pub fn RegQueryInfoKeyW(emu: &mut emu::Emu) {
    reg_query_info_key(emu, "RegQueryInfoKeyW");
}
