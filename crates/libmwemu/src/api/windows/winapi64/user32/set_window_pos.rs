use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub(super) fn SetWindowPos(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    let arg5 = WinApiAbi::arg(emu, 4);
    let arg6 = WinApiAbi::arg(emu, 5);
    let arg7 = WinApiAbi::arg(emu, 6);
    log_red!(
        emu,
        "user32!SetWindowPos arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x} arg7:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        arg6,
        arg7
    );
    emu.regs_mut().rax = 1;
}
