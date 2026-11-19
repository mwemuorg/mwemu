use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn RegDeleteKeyA(emu: &mut emu::Emu) {
    reg_delete_key(emu, "RegDeleteKeyA", false);
}

fn reg_delete_key(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg_u32(emu, 0));
    let subkey_ptr = api_try!(emu, call.arg(emu, 1));
    let subkey = strings::read_text(emu, subkey_ptr, wide);
    log_red!(emu, "advapi32!{api} hkey:0x{:x} '{}'", hkey, subkey);
    call.cleanup_args(emu, 2);
    call.set_return(emu, 0);
}

pub fn RegDeleteKeyW(emu: &mut emu::Emu) {
    reg_delete_key(emu, "RegDeleteKeyW", true);
}
