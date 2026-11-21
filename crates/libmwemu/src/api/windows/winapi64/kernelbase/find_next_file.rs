use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn FindNextFileA(emu: &mut emu::Emu) {
    find_next_file(emu, "FindNextFileA", false);
}

fn find_next_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let name = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "kernelbase!{api} '{}' 0x{:x}", name, arg2);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn FindNextFileW(emu: &mut emu::Emu) {
    find_next_file(emu, "FindNextFileW", true);
}
