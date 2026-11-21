use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GopherFindFirstFileA(emu: &mut emu::Emu) {
    gopher_find_first_file(emu, "GopherFindFirstFileA");
}

fn gopher_find_first_file(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let a0 = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let a1 = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    log_red!(emu, "wininet!{api} arg0:0x{:x} arg1:0x{:x}", a0, a1);
    emu.set_last_error(0);
    call.set_return(emu, 0xFFFFFFFFFFFFFFFF);
    call.cleanup_args(emu, 6);
}

pub fn GopherFindFirstFileW(emu: &mut emu::Emu) {
    gopher_find_first_file(emu, "GopherFindFirstFileW");
}
