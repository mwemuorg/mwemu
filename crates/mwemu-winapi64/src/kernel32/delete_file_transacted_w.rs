use crate::emu;

pub fn DeleteFileTransactedW(emu: &mut emu::Emu) {
    let lpFileName = emu.regs().rcx as u32;
    let _hTransaction = emu.regs().rdx as u32;

    let lpFileName = emu.maps.read_wide_string(lpFileName as u64);

    log_red!(emu, "kernel32!DeleteFileTransactedW {}", lpFileName);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
