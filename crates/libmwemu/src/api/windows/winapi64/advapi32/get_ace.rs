use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetAce(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "advapi32!GetAce p1:0x{:x} p2:0x{:x}", p1, p2);
    let out_ptr = api_try!(emu, call.arg(emu, 2));
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    for _ in 0..3 {}
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
