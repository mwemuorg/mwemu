use crate::emu;

pub fn GetAsyncKeyState(emu: &mut emu::Emu) {
    let vkey = emu.regs().rcx as u32;
    log_red!(emu, "user32!GetAsyncKeyState vKey=0x{:x}", vkey);
    emu.regs_mut().rax = 0; // key not pressed
}
