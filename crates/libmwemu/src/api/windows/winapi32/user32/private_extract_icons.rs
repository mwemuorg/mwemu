use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn PrivateExtractIconsA(emu: &mut emu::Emu) {
    private_extract_icons(emu, "PrivateExtractIconsA");
}

fn private_extract_icons(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let a0 = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let a1 = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    log_red!(emu, "user32!{api} arg0:0x{:x} arg1:0x{:x}", a0, a1);
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 8);
}

pub fn PrivateExtractIconsW(emu: &mut emu::Emu) {
    private_extract_icons(emu, "PrivateExtractIconsW");
}
