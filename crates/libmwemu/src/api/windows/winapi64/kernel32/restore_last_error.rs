use crate::emu;

pub fn RestoreLastError(emu: &mut emu::Emu) {
    let error = emu.regs().rcx as u32;

    log_red!(emu, "kernel32!RestoreLastError error:0x{:x}", error);

    emu.set_last_error(error);
}
