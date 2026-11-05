use crate::emu;

pub fn LZClose(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    log_red!(emu, "kernel32!LZClose hFile:0x{:x}", h_file);
    emu.set_last_error(0);
}
