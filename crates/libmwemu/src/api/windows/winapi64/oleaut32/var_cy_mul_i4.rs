use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn VarCyMulI4(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let src = api_try!(emu, call.arg(emu, 0));
    let dst = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "oleaut32!VarCyMulI4 src:0x{:x}", src);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
