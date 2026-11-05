use crate::emu;
use crate::winapi::helper;

pub fn CreateSemaphoreExA(emu: &mut emu::Emu) {
    let _lpSemaphoreAttributes = emu.regs().rcx as u32;
    let _lInitialCount = emu.regs().rdx as u32;
    let _lMaximumCount = emu.regs().r8 as u32;
    let lpName = emu.regs().r9 as u32;
    let _dwFlags = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _dwDesiredAccess = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);

    let lpName = emu.maps.read_string(lpName as u64);

    log_red!(emu, "kernel32!CreateSemaphoreExA {}", lpName);

    emu.regs_mut().rax = helper::handler_create("CreateSemaphoreExA");
}
