
use crate::windows::constants::errors::*;
use crate::emu;

pub fn GetNumberFormatW(emu: &mut emu::Emu) {
    let _Locale = emu.regs().rcx as u32;
    let _dwFlags = emu.regs().rdx as u32;
    let _lpValue = emu.regs().r8 as u32;
    let _lpFormat = emu.regs().r9 as u32;
    let lpNumberStr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _cchNumber = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    let lpNumberStr = emu.maps.read_wide_string(lpNumberStr);

    log_red!(emu, "kernel32!GetNumberFormatW {}", lpNumberStr);

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
