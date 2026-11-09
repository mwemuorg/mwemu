
use crate::windows::constants::errors::*;
use crate::emu;

pub fn GetBinaryTypeW(emu: &mut emu::Emu) {
    let lpApplicationName = emu.regs().rcx as u32;
    let _lpBinaryType = emu.regs().rdx as u32;

    let lpApplicationName = emu.maps.read_wide_string(lpApplicationName as u64);

    log_red!(emu, "kernel32!GetBinaryTypeW {}", lpApplicationName);

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
