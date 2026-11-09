
use crate::windows::constants::errors::*;
use crate::emu;

pub fn BuildCommDCBA(emu: &mut emu::Emu) {
    let _lpDef = emu.regs().rcx as u32;
    let _lpDCB = emu.regs().rdx as u32;

    log_red!(emu, "kernel32!BuildCommDCBA");
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
