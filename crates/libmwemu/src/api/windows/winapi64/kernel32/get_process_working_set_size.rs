use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetProcessWorkingSetSize(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let min_ptr = api_try!(emu, call.arg(emu, 1));
    let max_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "kernel32!GetProcessWorkingSetSize h:0x{:x}", handle);
    if min_ptr != 0 {
        emu.maps.write_qword(min_ptr, 0x20000);
    }
    if max_ptr != 0 {
        emu.maps.write_qword(max_ptr, 0x1400000);
    }
    call.set_return(emu, 1);
}
