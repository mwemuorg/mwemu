use crate::emu;
use crate::winapi::helper;

pub fn CreateJobObjectW(emu: &mut emu::Emu) {
    let _lpJobAttributes = emu.regs().rcx as u32;
    let lpName = emu.regs().rdx as u32;

    let lpName = emu.maps.read_wide_string(lpName as u64);

    log_red!(emu, "kernel32!CreateJobObjectW {}", lpName);

    emu.regs_mut().rax = helper::handler_create("CreateJobObjectW");
}
