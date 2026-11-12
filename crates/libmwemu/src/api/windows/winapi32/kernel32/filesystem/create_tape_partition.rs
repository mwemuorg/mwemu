use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn CreateTapePartition(emu: &mut emu::Emu) {
    let _hDevice = WinApiAbi::arg(emu, 0) as u32;
    let _dwPartitionMethod = WinApiAbi::arg(emu, 1) as u32;
    let _dwCount = WinApiAbi::arg(emu, 2) as u32;
    let _dwSize = WinApiAbi::arg(emu, 3) as u32;

    log_red!(emu, "kernel32!CreateTapePartition");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
