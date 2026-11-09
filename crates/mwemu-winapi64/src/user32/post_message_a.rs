use crate::emu;

pub(super) fn PostMessageA(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!PostMessageA hwnd:0x{:x} msg:0x{:x} wParam:0x{:x} lParam:0x{:x}",
        emu.regs().rcx,
        emu.regs().rdx,
        emu.regs().r8,
        emu.regs().r9
    );
    emu.regs_mut().rax = 1;
}
