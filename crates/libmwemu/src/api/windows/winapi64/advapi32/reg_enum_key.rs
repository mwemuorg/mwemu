use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn RegEnumKeyA(emu: &mut emu::Emu) {
    reg_enum_key(emu, "RegEnumKeyA");
}

fn reg_enum_key(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let index = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "advapi32!{api} hkey:0x{:x} index:{}", hkey, index);
    emu.set_last_error(0);
    call.set_return(emu, 259);
    for _ in 0..8 {}
}

pub fn RegEnumKeyW(emu: &mut emu::Emu) {
    reg_enum_key(emu, "RegEnumKeyW");
}
