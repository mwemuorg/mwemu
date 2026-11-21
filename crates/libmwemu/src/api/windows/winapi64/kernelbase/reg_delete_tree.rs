use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn RegDeleteTreeA(emu: &mut emu::Emu) {
    reg_delete_tree(emu, "RegDeleteTreeA", false);
}

fn reg_delete_tree(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let subkey_ptr = api_try!(emu, call.arg(emu, 1));
    let subkey = if subkey_ptr != 0 {
        strings::read_text(emu, subkey_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "kernelbase!{api} hkey:0x{:x} '{}'", hkey, subkey);
    emu.set_last_error(0);
    call.set_return(emu, 0); // ERROR_SUCCESS
}

pub fn RegDeleteTreeW(emu: &mut emu::Emu) {
    reg_delete_tree(emu, "RegDeleteTreeW", true);
}
