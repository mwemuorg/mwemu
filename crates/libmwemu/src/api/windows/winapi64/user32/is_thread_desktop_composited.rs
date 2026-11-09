use crate::emu;

pub fn IsThreadDesktopComposited(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "user32!IsThreadDesktopComposited h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
}
