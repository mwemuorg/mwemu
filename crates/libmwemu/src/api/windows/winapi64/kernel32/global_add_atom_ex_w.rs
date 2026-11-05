use crate::emu;
use crate::windows::constants;

pub fn GlobalAddAtomExW(emu: &mut emu::Emu) {
    let lpString = emu.regs().rcx as u32;
    let _Flags = emu.regs().rdx as u32;

    let lpString = emu.maps.read_wide_string(lpString as u64);

    log_red!(emu, "kernel32!GlobalAddAtomExW {}", lpString);

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
