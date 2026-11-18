use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn RegQueryMultipleValuesA(emu: &mut emu::Emu) {
    reg_query_multiple_values(emu, "RegQueryMultipleValuesA", false);
}

fn reg_query_multiple_values(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 1));
    let reserved = api_try!(emu, call.arg(emu, 2));
    let type_ptr = api_try!(emu, call.arg(emu, 3));
    let name = strings::read_text(emu, name_ptr, wide);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    log_red!(emu, "advapi32!{api} hkey:0x{:x} '{}'", hkey, name);
    call.set_return(emu, 2);
}

pub fn RegQueryMultipleValuesW(emu: &mut emu::Emu) {
    reg_query_multiple_values(emu, "RegQueryMultipleValuesW", true);
}
