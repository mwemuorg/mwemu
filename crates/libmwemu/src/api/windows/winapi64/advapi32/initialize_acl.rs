use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InitializeAcl(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "advapi32!InitializeAcl p1:0x{:x} p2:0x{:x}", p1, p2);

    for _ in 0..3 {}
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
