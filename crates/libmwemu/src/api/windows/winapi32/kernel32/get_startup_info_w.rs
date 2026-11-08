use crate::emu;
use crate::windows::structures;

pub fn GetStartupInfoW(emu: &mut emu::Emu) {
    let startup_info_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    log_red!(emu, "kernel32!GetStartupInfoW");
    if startup_info_ptr > 0 {
        let startupinfo = structures::StartupInfo32::new();
        startupinfo.save(startup_info_ptr, &mut emu.maps);
    }

    emu.stack_pop32(false);
}
