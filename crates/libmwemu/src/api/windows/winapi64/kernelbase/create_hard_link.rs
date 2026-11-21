use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateHardLinkA(emu: &mut emu::Emu) {
    create_hard_link(emu, "CreateHardLinkA", false);
}

fn create_hard_link(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let name = strings::read_text(emu, arg1, wide);
    let hndl = helper::handler_create(&format!("kb_{}", name));
    log_red!(emu, "kernelbase!{api} '{}' => 0x{:x}", name, hndl);
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}

pub fn CreateHardLinkW(emu: &mut emu::Emu) {
    create_hard_link(emu, "CreateHardLinkW", true);
}
