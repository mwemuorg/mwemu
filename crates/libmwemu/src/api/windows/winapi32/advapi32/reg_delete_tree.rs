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
    let name_ptr = api_try!(emu, call.arg(emu, 1));
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "advapi32!{api} hkey:0x{:x} '{}'", hkey, name);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 2);
}

pub fn RegDeleteTreeW(emu: &mut emu::Emu) {
    reg_delete_tree(emu, "RegDeleteTreeW", true);
}
