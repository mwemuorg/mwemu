use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn RegSetKeyValueA(emu: &mut emu::Emu) {
    reg_set_key_value(emu, "RegSetKeyValueA", false);
}

fn reg_set_key_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 1));
    let reserved = api_try!(emu, call.arg(emu, 2));
    let reg_type = api_try!(emu, call.arg(emu, 3));
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(
        emu,
        "advapi32!{api} hkey:0x{:x} '{}' type:{}",
        hkey,
        name,
        reg_type
    );
    call.set_return(emu, 0);
    call.cleanup_args(emu, 6);
}

pub fn RegSetKeyValueW(emu: &mut emu::Emu) {
    reg_set_key_value(emu, "RegSetKeyValueW", true);
}
