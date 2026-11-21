use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn RegQueryValueExA(emu: &mut emu::Emu) {
    reg_query_value_ex(emu, "RegQueryValueExA", false);
}

fn reg_query_value_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 1));
    let _reserved = api_try!(emu, call.arg(emu, 2));
    let type_ptr = api_try!(emu, call.arg(emu, 3));
    let data_ptr = api_try!(emu, call.arg(emu, 4));
    let size_ptr = api_try!(emu, call.arg(emu, 5));
    let name = strings::read_text(emu, name_ptr, wide);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, 0);
    }
    log_red!(emu, "kernelbase!{api} hkey:0x{:x} '{}'", hkey, name);
    call.set_return(emu, 2);
}

pub fn RegQueryValueExW(emu: &mut emu::Emu) {
    reg_query_value_ex(emu, "RegQueryValueExW", true);
}
