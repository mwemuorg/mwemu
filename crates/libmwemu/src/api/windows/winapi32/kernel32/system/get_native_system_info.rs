use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::structures;

pub fn GetNativeSystemInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sysinfo_ptr = api_try!(emu, call.arg(emu, 0));

    let mut sysinfo = structures::SystemInfo32::new();
    sysinfo.save(sysinfo_ptr, &mut emu.maps);

    log_red!(emu, "kernel32!GetNativeSystemInfo");

    call.cleanup_args(emu, 1);
}
