use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn FindNextFileA(emu: &mut emu::Emu) {
    find_next_file(emu, "FindNextFileA");
}

fn find_next_file(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let h_find = api_try!(emu, call.arg(emu, 0));
    let find_data_ptr = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "kernel32!{api} h:0x{:x} data:0x{:x} => FALSE (no more files)",
        h_find,
        find_data_ptr
    );
    call.set_return(emu, 0);
    emu.set_last_error(18);
}

pub fn FindNextFileW(emu: &mut emu::Emu) {
    find_next_file(emu, "FindNextFileW");
}
