
use crate::windows::constants::errors::*;
use crate::emu;

pub fn LZOpenFileW(emu: &mut emu::Emu) {
    let lpFileName = emu.regs().rcx as u32;
    let _lpReOpenBuf = emu.regs().rdx as u32;
    let _wStyle = emu.regs().r8 as u32;

    let lpFileName = emu.maps.read_wide_string(lpFileName as u64);

    log_red!(emu, "kernel32!LZOpenFileW {}", lpFileName);

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
