use crate::emu;
use crate::winapi::helper;

pub fn Process32Next(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let lppe = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!Process32Next hndl: {:x} lppe: 0x{:x}",
        handle,
        lppe
    );

    emu.maps.write_string(lppe + 44, "explorer.exe\x00");

    if !helper::handler_exist(handle) {
        emu.regs_mut().rax = 0;
        return;
    }

    emu.regs_mut().rax = 0; // trigger exit loop
}
