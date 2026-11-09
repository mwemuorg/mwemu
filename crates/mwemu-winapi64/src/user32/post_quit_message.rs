use crate::emu;

pub(super) fn PostQuitMessage(emu: &mut emu::Emu) {
    let exit_code = emu.regs().rcx as i32;
    log_red!(emu, "user32!PostQuitMessage code:{}", exit_code);
}
