use crate::emu;

pub fn SetFileShortNameW(emu: &mut emu::Emu) {
    let _hFile = emu.regs().rcx as u32;
    let lpShortName = emu.regs().rdx as u32;

    let lpShortName = emu.maps.read_wide_string(lpShortName as u64);

    log_red!(emu, "kernel32!SetFileShortNameW {}", lpShortName);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
