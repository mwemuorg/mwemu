use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InitializeCriticalSectionEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cs_ptr = api_try!(emu, call.arg(emu, 0));
    let spin_count = api_try!(emu, call.arg(emu, 1));
    let flags = api_try!(emu, call.arg(emu, 2));
    if cs_ptr != 0 {
        emu.maps.write_qword(cs_ptr, 0);
    }
    log_red!(
        emu,
        "kernelbase!InitializeCriticalSectionEx ptr:0x{:x} spin:{} flags:0x{:x}",
        cs_ptr,
        spin_count,
        flags
    );
    call.set_return(emu, 1);
}
