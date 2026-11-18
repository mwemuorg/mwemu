use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SizeofResource(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "kernelbase!SizeofResource p1:0x{:x}", p1);
    emu.set_last_error(0);
    call.set_return(emu, 4096);
}
