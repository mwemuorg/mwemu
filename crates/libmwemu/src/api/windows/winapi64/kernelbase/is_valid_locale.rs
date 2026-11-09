use crate::emu;
use crate::winapi::helper;

pub fn IsValidLocale(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx;
    let flags = emu.regs().rdx;
    log_red!(
        emu,
        "kernelbase!IsValidLocale locale:0x{:x} flags:0x{:x}",
        locale,
        flags
    );
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
