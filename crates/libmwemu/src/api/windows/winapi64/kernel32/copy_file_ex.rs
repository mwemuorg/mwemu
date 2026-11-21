use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn CopyFileExA(emu: &mut Emu) {
    invoke(emu, false, "CopyFileExA");
}

pub fn CopyFileExW(emu: &mut Emu) {
    invoke(emu, true, "CopyFileExW");
}

fn invoke(emu: &mut Emu, wide: bool, api: &str) {
    let call = WinApi(Stdcall);
    let source = strings::read_text(emu, api_try!(emu, call.arg(emu, 0)), wide);
    let destination = strings::read_text(emu, api_try!(emu, call.arg(emu, 1)), wide);
    log_red!(emu, "kernel32!{} '{}' -> '{}'", api, source, destination);
    call.set_return(emu, 1);
}
