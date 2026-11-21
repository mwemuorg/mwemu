use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn RegEnumValueA(emu: &mut emu::Emu) {
    reg_enum_value(emu, "RegEnumValueA");
}

fn reg_enum_value(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let index = api_try!(emu, call.arg(emu, 1));
    let name_ptr = api_try!(emu, call.arg(emu, 2));
    let name_len_ptr = api_try!(emu, call.arg(emu, 3));
    log_red!(emu, "kernelbase!{api} hkey:0x{:x} index:{}", hkey, index);
    let out_ptr = api_try!(emu, call.arg(emu, 3));
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 259);
}

pub fn RegEnumValueW(emu: &mut emu::Emu) {
    reg_enum_value(emu, "RegEnumValueW");
}
