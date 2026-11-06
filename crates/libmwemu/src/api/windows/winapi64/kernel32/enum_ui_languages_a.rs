use crate::emu;

pub fn EnumUILanguagesA(emu: &mut emu::Emu) {
    let callback = emu.regs().rcx;
    log_red!(emu, "kernel32!EnumUILanguagesA callback:0x{:x}", callback);
    let _ = emu.maps.is_mapped(emu.regs().rcx);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
