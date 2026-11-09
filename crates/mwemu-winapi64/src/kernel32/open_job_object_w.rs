use crate::emu;
use crate::winapi::helper;

pub fn OpenJobObjectW(emu: &mut emu::Emu) {
    let _dwDesiredAccess = emu.regs().rcx as u32;
    let _bInheritHandle = emu.regs().rdx as u32;
    let lpName = emu.regs().r8 as u32;

    let lpName = emu.maps.read_wide_string(lpName as u64);

    log_red!(emu, "kernel32!OpenJobObjectW {}", lpName);

    emu.regs_mut().rax = helper::handler_create("OpenJobObjectW");
}
