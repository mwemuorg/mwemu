use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::structures;

pub fn GetStartupInfoA(emu: &mut emu::Emu) {
    get_startup_info(emu, "GetStartupInfoA");
}

fn get_startup_info(emu: &mut emu::Emu, api: &str) {
    let startup_info_ptr = WinApiAbi::arg(emu, 0);

    log_red!(emu, "kernel32!{api}");
    if startup_info_ptr > 0 {
        let startupinfo = structures::StartupInfo32::new();
        startupinfo.save(startup_info_ptr, &mut emu.maps);
    }

    emu.stack_pop32(false);
}

pub fn GetStartupInfoW(emu: &mut emu::Emu) {
    get_startup_info(emu, "GetStartupInfoW");
}
