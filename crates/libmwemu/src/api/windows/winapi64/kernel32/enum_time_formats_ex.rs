use crate::emu;

pub fn EnumTimeFormatsEx(emu: &mut emu::Emu) {
    let callback = emu.regs().rcx;
    log_red!(emu, "kernel32!EnumTimeFormatsEx callback:0x{:x}", callback);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
