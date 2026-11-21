use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn QueryPerformanceFrequency(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let out_ptr = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    log_red!(emu, "kernel32!QueryPerformanceFrequency");
    emu.maps.write_qword(out_ptr, 10_000_000);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 1);
}
