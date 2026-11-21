use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::structures;

pub fn GetStartupInfoA(emu: &mut emu::Emu) {
    get_startup_info(emu, "GetStartupInfoA");
}

fn get_startup_info(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let startup_info_ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernel32!{api}");
    let handle = helper::handler_create(api);
    if startup_info_ptr > 0 {
        let startupinfo = structures::StartupInfo64::new();
        startupinfo.save(startup_info_ptr, &mut emu.maps);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn GetStartupInfoW(emu: &mut emu::Emu) {
    get_startup_info(emu, "GetStartupInfoW");
}
