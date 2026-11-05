use crate::emu;
use crate::winapi::helper;

pub fn CreateHardLinkTransactedA(emu: &mut emu::Emu) {
    let lpFileName = emu.regs().rcx as u32;
    let lpExistingFileName = emu.regs().rdx as u32;
    let _lpSecurityAttributes = emu.regs().r8 as u32;
    let _hTransaction = emu.regs().r9 as u32;

    let lpFileName = emu.maps.read_string(lpFileName as u64);
    let lpExistingFileName = emu.maps.read_string(lpExistingFileName as u64);

    log_red!(
        emu,
        "kernel32!CreateHardLinkTransactedA {} {}",
        lpFileName,
        lpExistingFileName
    );

    emu.regs_mut().rax = helper::handler_create("CreateHardLinkTransactedA");
}
