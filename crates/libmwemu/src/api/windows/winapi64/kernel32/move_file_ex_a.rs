use crate::emu;

pub fn MoveFileExA(emu: &mut emu::Emu) {
    let lpExistingFileName = emu.regs().rcx as u32;
    let lpNewFileName = emu.regs().rdx as u32;
    let _dwFlags = emu.regs().r8 as u32;

    let lpExistingFileName = emu.maps.read_string(lpExistingFileName as u64);
    let lpNewFileName = emu.maps.read_string(lpNewFileName as u64);

    log_red!(
        emu,
        "kernel32!MoveFileExA {} {}",
        lpExistingFileName,
        lpNewFileName
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
