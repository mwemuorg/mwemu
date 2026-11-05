use crate::emu;

pub fn SetFileShortNameA(emu: &mut emu::Emu) {
    let _hFile = emu.regs().rcx as u32;
    let lpShortName = emu.regs().rdx as u32;

    let lpShortName = emu.maps.read_string(lpShortName as u64);

    log_red!(emu, "kernel32!SetFileShortNameA {}", lpShortName);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
