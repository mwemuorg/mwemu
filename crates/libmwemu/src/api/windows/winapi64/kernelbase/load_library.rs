use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn LoadLibraryA(emu: &mut emu::Emu) {
    load_library(emu, "LoadLibraryA", false);
}

fn load_library(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let name_ptr = api_try!(emu, call.arg(emu, 0));
    let name = strings::read_text(emu, name_ptr, wide);
    let hndl = helper::handler_create(&format!("module_{}", name));
    log_red!(emu, "kernelbase!{api} '{}' => 0x{:x}", name, hndl);
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}

pub fn LoadLibraryW(emu: &mut emu::Emu) {
    load_library(emu, "LoadLibraryW", true);
}
