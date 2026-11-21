use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn QueryPerformanceFrequency(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let freq_ptr = api_try!(emu, call.arg(emu, 0));
    if freq_ptr != 0 {
        emu.maps.write_qword(freq_ptr, 10000000);
    }
    log_red!(emu, "kernelbase!QueryPerformanceFrequency => 10000000");
    call.set_return(emu, 1);
}
