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
    log_red!(emu, "advapi32!{api} hkey:0x{:x} idx:{}", hkey, index);
    for _ in 0..8 {}
    emu.set_last_error(0);
    call.set_return(emu, 259);
}

pub fn RegEnumValueW(emu: &mut emu::Emu) {
    reg_enum_value(emu, "RegEnumValueW");
}
