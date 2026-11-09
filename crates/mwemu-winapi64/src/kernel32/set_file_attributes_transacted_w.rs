use crate::emu;

pub fn SetFileAttributesTransactedW(emu: &mut emu::Emu) {
    let lpFileName = emu.regs().rcx as u32;
    let _dwFileAttributes = emu.regs().rdx as u32;
    let _hTransaction = emu.regs().r8 as u32;

    let lpFileName = emu.maps.read_wide_string(lpFileName as u64);

    log_red!(emu, "kernel32!SetFileAttributesTransactedW {}", lpFileName);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
