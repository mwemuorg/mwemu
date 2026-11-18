use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn FlsGetValue2(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let param1 = api_try!(emu, call.arg(emu, 0));
    let param2 = api_try!(emu, call.arg(emu, 1));
    let param3 = api_try!(emu, call.arg(emu, 2));
    let param4 = api_try!(emu, call.arg(emu, 3));

    log_red!(
        emu,
        "kernel32!FlsGetValue2 p1:0x{:x} p2:0x{:x} p3:0x{:x} p4:0x{:x}",
        param1,
        param2,
        param3,
        param4
    );
    let out_ptr = api_try!(emu, call.arg(emu, 3));
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    emu.set_last_error(0);
    call.set_return(emu, 1);
}
