use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetMonitorInfoA(emu: &mut emu::Emu) {
    get_monitor_info(emu, "GetMonitorInfoA");
}

fn get_monitor_info(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let hmonitor = api_try!(emu, call.arg(emu, 0));
    let info_ptr = api_try!(emu, call.arg(emu, 1));
    if info_ptr != 0 {
        emu.maps.write_dword(info_ptr + 4, 0);
        emu.maps.write_dword(info_ptr + 8, 0);
        emu.maps.write_dword(info_ptr + 12, 1920);
        emu.maps.write_dword(info_ptr + 16, 1080);
        emu.maps.write_dword(info_ptr + 20, 0);
        emu.maps.write_dword(info_ptr + 24, 0);
        emu.maps.write_dword(info_ptr + 28, 1920);
        emu.maps.write_dword(info_ptr + 32, 1080);
        emu.maps.write_dword(info_ptr + 36, 1);
    }
    log_red!(
        emu,
        "user32!{api} hmon:0x{:x} info:0x{:x}",
        hmonitor,
        info_ptr
    );
    call.set_return(emu, 1);
}

pub fn GetMonitorInfoW(emu: &mut emu::Emu) {
    get_monitor_info(emu, "GetMonitorInfoW");
}
