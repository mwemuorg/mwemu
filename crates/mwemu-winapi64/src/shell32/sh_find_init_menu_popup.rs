use crate::emu;
use crate::winapi::helper;

pub fn SHFind_InitMenuPopup(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "shell32!SHFind_InitMenuPopup arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("shfind_initmenupopup");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}
