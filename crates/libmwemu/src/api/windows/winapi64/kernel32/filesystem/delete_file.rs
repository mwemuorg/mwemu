use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn DeleteFileA(emu: &mut emu::Emu) {
    delete_file(emu, "DeleteFileA", false);
}

fn delete_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let filename_ptr = api_try!(emu, call.arg(emu, 0));

    let filename = strings::read_text(emu, filename_ptr, wide);

    log_red!(emu, "kernel32!{api} `{}` (stub: not deleting)", filename);

    emu.set_last_error(0);
    call.set_return(emu, 1); // Success
}

pub fn DeleteFileW(emu: &mut emu::Emu) {
    delete_file(emu, "DeleteFileW", true);
}
