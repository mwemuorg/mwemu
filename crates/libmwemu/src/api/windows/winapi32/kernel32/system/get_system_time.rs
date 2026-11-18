use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::structures;

pub fn GetSystemTime(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let out_time = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "kernel32!GetSystemTime ptr: 0x{:x}'", out_time);
    let systime = structures::SystemTime::now();
    systime.save(out_time, &mut emu.maps);

    call.cleanup_args(emu, 1);
}
