use crate::emu;
use crate::winapi::helper;

pub fn CreateJobObjectA(emu: &mut emu::Emu) {
    let _lpJobAttributes = emu.regs().rcx as u32;
    let lpName = emu.regs().rdx as u32;

    let lpName = emu.maps.read_string(lpName as u64);

    log_red!(emu, "kernel32!CreateJobObjectA {}", lpName);

    emu.regs_mut().rax = helper::handler_create("CreateJobObjectA");
}
