
use crate::emu;
use crate::winapi::helper;
pub fn _lread(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let buffer = emu.regs().rdx;
    let count = emu.regs().r8 as usize;
    log_red!(
        emu,
        "kernel32!_lread handle:0x{:x} buffer:0x{:x} count:{}",
        handle,
        buffer,
        count
    );
    if helper::handler_exist(handle) && buffer != 0 && count > 0 {
        emu.set_last_error(0);
        emu.regs_mut().rax = count as u64;
    } else {
        emu.regs_mut().rax = -1i64 as u64;
    }
}
