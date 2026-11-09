
use crate::windows::constants::errors::*;
use crate::emu;

pub fn Heap32First(emu: &mut emu::Emu) {
    let _lphe = emu.regs().rcx as u32;
    let _th32ProcessID = emu.regs().rdx as u32;
    let _th32HeapID = emu.regs().r8 as u32;

    log_red!(emu, "kernel32!Heap32First");
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
