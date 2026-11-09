use crate::emu;
use crate::winapi::helper;

pub fn CreateMailslotA(emu: &mut emu::Emu) {
    let lpName = emu.regs().rcx as u32;
    let _nMaxMessageSize = emu.regs().rdx as u32;
    let _lReadTimeout = emu.regs().r8 as u32;
    let _lpSecurityAttributes = emu.regs().r9 as u32;

    let lpName = emu.maps.read_string(lpName as u64);

    log_red!(emu, "kernel32!CreateMailslotA {}", lpName);

    emu.regs_mut().rax = helper::handler_create("CreateMailslotA");
}
