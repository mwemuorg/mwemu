use crate::emu;
use crate::winapi::helper;

pub fn handler(emu: &mut emu::Emu) {
    let alloc_size = emu.regs().rax;
    log_red!(emu, "kernel32!__chkstk alloc_size:0x{:x}", alloc_size);
    helper::advance_tick(emu, 1);
    let new_sp = emu.regs().rsp.wrapping_sub(alloc_size);
    emu.regs_mut().rsp = new_sp;
}
