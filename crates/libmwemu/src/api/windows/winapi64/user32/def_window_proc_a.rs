use crate::emu;

pub(super) fn DefWindowProcA(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!DefWindowProcA hwnd:0x{:x} msg:0x{:x}",
        emu.regs().rcx,
        emu.regs().rdx
    );
    emu.regs_mut().rax = 0;
}
