
use crate::windows::constants::errors::*;
use crate::emu;

pub fn NotifyUILanguageChange(emu: &mut emu::Emu) {
    let _dwFlags = emu.regs().rcx as u32;
    let _pcwstrNewLanguage = emu.regs().rdx as u32;
    let _pcwstrPreviousLanguage = emu.regs().r8 as u32;
    let _dwReserved = emu.regs().r9 as u32;
    let _pdwStatusRtrn = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    log_red!(emu, "kernel32!NotifyUILanguageChange");
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
