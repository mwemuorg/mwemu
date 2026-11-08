use crate::emu;

pub(super) fn DispatchMessageA(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "user32!DispatchMessageA arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
