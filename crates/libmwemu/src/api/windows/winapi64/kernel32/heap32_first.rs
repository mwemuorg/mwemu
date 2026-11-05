use crate::emu;
use crate::windows::constants;

pub fn Heap32First(emu: &mut emu::Emu) {
    let _lphe = emu.regs().rcx as u32;
    let _th32ProcessID = emu.regs().rdx as u32;
    let _th32HeapID = emu.regs().r8 as u32;

    log_red!(emu, "kernel32!Heap32First");

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
