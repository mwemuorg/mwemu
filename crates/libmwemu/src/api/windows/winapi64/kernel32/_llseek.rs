use crate::api::windows::helper;
use crate::emu;

pub fn _llseek(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let offset = emu.regs().rdx as i32;
    let origin = emu.regs().r8 as u32;

    log_red!(
        emu,
        "kernel32!_llseek handle:0x{:x} offset:{} origin:{}",
        handle,
        offset,
        origin
    );

    if helper::handler_exist(handle) {
        let pos = helper::handler_seek(handle, offset as i64, origin);
        emu.set_last_error(0);
        emu.regs_mut().rax = pos as u64;
    } else {
        emu.regs_mut().rax = -1i64 as u64;
    }
}
