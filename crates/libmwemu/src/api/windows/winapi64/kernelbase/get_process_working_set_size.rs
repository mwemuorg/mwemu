use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetProcessWorkingSetSize(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let min_ptr = api_try!(emu, call.arg(emu, 1));
    let max_ptr = api_try!(emu, call.arg(emu, 2));
    if min_ptr != 0 {
        emu.maps.write_qword(min_ptr, 204800); // 200 KB min
    }
    if max_ptr != 0 {
        emu.maps.write_qword(max_ptr, 1413120); // ~1.3 MB max
    }
    log_red!(emu, "kernelbase!GetProcessWorkingSetSize h:0x{:x}", handle);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
