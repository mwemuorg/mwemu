use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::structures;

pub fn SystemTimeToFileTime(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let out_ptr = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    log_red!(emu, "kernel32!SystemTimeToFileTime");
    structures::SystemTime::now().save(out_ptr, &mut emu.maps);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
