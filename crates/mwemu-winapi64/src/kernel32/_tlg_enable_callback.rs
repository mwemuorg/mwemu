
use crate::windows::constants::errors::*;
use crate::emu;
use crate::winapi::helper;

pub fn _tlgEnableCallback(emu: &mut emu::Emu) {
    let _pSourceId = emu.regs().rcx as u32;
    let _callbackType = emu.regs().rdx as u32;
    let _level = emu.regs().r8 as u32;
    let _keywordAny = emu.regs().r9 as u32;
    let _keywordAll = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _pFilterData = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _pCallbackContext = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    log_red!(emu, "kernel32!_tlgEnableCallback");
    let handle = helper::handler_create("TlgEnableCallback");
    let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
