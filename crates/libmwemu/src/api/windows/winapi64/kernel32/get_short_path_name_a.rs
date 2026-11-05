use crate::emu;
use crate::windows::constants;

pub fn GetShortPathNameA(emu: &mut emu::Emu) {
    let lpszLongPath = emu.regs().rcx as u32;
    let lpszShortPath = emu.regs().rdx as u32;
    let _cchBuffer = emu.regs().r8 as u32;

    let lpszLongPath = emu.maps.read_string(lpszLongPath as u64);
    let lpszShortPath = emu.maps.read_string(lpszShortPath as u64);

    log_red!(
        emu,
        "kernel32!GetShortPathNameA {} {}",
        lpszLongPath,
        lpszShortPath
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
