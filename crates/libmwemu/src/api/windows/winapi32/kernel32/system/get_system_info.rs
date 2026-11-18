use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetSystemInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let out_sysinfo = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "kernel32!GetSystemInfo sysinfo: 0x{:x}", out_sysinfo);

    // let mut sysinfo = emu::structures::SystemInfo32::new();
    // sysinfo.save(out_sysinfo, &mut emu.maps);

    call.cleanup_args(emu, 1);
}
