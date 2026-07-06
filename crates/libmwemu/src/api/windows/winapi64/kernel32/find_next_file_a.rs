use crate::{emu, windows::constants};

pub fn FindNextFileA(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let find_data = emu.regs().rdx;

    log_red!(emu, "kernel32!FindNextFileA");

    // TODO: implement

    emu.regs_mut().rax = constants::ERROR_NO_MORE_FILES;
}
