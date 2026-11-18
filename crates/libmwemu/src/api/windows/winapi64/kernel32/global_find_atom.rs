use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GlobalFindAtomA(emu: &mut emu::Emu) {
    global_find_atom(emu, "GlobalFindAtomA", false);
}

fn global_find_atom(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let name = api_try!(emu, call.arg(emu, 0));
    let name_s = if name != 0 {
        strings::read_text(emu, name, wide)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!{api} '{}' => 0", name_s);
    call.set_return(emu, 0);
    emu.set_last_error(2);
}

pub fn GlobalFindAtomW(emu: &mut emu::Emu) {
    global_find_atom(emu, "GlobalFindAtomW", true);
}
