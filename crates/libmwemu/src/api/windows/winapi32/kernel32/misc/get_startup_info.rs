use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::structures;

pub fn GetStartupInfoA(emu: &mut emu::Emu) {
    get_startup_info(emu, "GetStartupInfoA");
}

fn get_startup_info(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let startup_info_ptr = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "kernel32!{api}");
    if startup_info_ptr > 0 {
        let startupinfo = structures::StartupInfo32::new();
        startupinfo.save(startup_info_ptr, &mut emu.maps);
    }

    call.cleanup_args(emu, 1);
}

pub fn GetStartupInfoW(emu: &mut emu::Emu) {
    get_startup_info(emu, "GetStartupInfoW");
}
