use crate::emu;
use crate::winapi::helper;

pub fn MsgWaitForMultipleObjectsEx(emu: &mut emu::Emu) {
    let count = emu.regs().rcx as u32;
    let handles = emu.regs().rdx;
    let ms = emu.regs().r8 as u32;
    let wake_mask = emu.regs().r9 as u32;
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as u32;
    helper::advance_tick(emu, ms as u64);
    log_red!(
        emu,
        "user32!MsgWaitForMultipleObjectsEx count:{} ms:{} mask:0x{:x} flags:0x{:x}",
        count,
        ms,
        wake_mask,
        flags
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
