use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn SearchPathA(emu: &mut emu::Emu) {
    search_path(emu, "SearchPathA", false);
}

fn search_path(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _path = api_try!(emu, call.arg(emu, 0));
    let filename = api_try!(emu, call.arg(emu, 1));
    let _ext = api_try!(emu, call.arg(emu, 2));
    let name_s = if filename != 0 {
        strings::read_text(emu, filename, wide)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!{api} '{}' => 0 (not found)", name_s);
    emu.set_last_error(2); // ERROR_FILE_NOT_FOUND
    call.set_return(emu, 0);
}

pub fn SearchPathW(emu: &mut emu::Emu) {
    search_path(emu, "SearchPathW", true);
}
