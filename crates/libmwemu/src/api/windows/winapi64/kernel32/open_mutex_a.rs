use crate::emu;
use crate::winapi::helper;

pub fn OpenMutexA(emu: &mut emu::Emu) {
    let _dwDesiredAccess = emu.regs().rcx as u32;
    let _bInheritHandle = emu.regs().rdx as u32;
    let lpName = emu.regs().r8 as u32;

    let lpName = emu.maps.read_string(lpName as u64);

    log_red!(emu, "kernel32!OpenMutexA {}", lpName);

    emu.regs_mut().rax = helper::handler_create("OpenMutexA");
}
