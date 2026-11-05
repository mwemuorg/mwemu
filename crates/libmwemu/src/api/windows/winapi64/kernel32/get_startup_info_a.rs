use crate::{emu, windows::structures};

pub fn GetStartupInfoA(emu: &mut emu::Emu) {
    let startup_info_ptr = emu.regs().rcx;

    log_red!(emu, "kernel32!GetStartupInfoA");
    if startup_info_ptr > 0 {
        let startupinfo = structures::StartupInfo64::new();
        startupinfo.save(startup_info_ptr, &mut emu.maps);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
