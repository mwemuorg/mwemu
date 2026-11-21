use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ActivateActCtx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_act_ctx = api_try!(emu, call.arg(emu, 0));
    let lp_cookie = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "kernel32!ActivateActCtx ctx:0x{:x} cookie:0x{:x}",
        h_act_ctx,
        lp_cookie
    );
    if lp_cookie != 0 {
        emu.maps.write_qword(lp_cookie, 0x1001);
    }
    call.set_return(emu, 1);
}
