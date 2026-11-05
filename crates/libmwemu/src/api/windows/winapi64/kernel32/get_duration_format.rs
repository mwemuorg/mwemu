use crate::emu;
use crate::windows::constants;

pub fn GetDurationFormat(emu: &mut emu::Emu) {
    let _Locale = emu.regs().rcx as u32;
    let _dwFlags = emu.regs().rdx as u32;
    let _lpDuration = emu.regs().r8 as u32;
    let _ullDuration = emu.regs().r9 as u32;
    let _lpFormat = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _lpDurationStr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let _cchDuration = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

    log_red!(emu, "kernel32!GetDurationFormat");

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
