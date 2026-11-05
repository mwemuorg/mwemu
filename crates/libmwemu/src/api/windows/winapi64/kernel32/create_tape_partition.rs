use crate::emu;
use crate::winapi::helper;

pub fn CreateTapePartition(emu: &mut emu::Emu) {
    let _hDevice = emu.regs().rcx as u32;
    let _dwPartitionMethod = emu.regs().rdx as u32;
    let _dwCount = emu.regs().r8 as u32;
    let _dwSize = emu.regs().r9 as u32;

    log_red!(emu, "kernel32!CreateTapePartition");

    emu.regs_mut().rax = helper::handler_create("CreateTapePartition");
}
