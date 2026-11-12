use crate::emu;
use crate::windows::structures;

pub fn GetStartupInfoA(emu: &mut emu::Emu) {
    get_startup_info(emu, "GetStartupInfoA");
}

fn get_startup_info(emu: &mut emu::Emu, api: &str) {
    let startup_info_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

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
