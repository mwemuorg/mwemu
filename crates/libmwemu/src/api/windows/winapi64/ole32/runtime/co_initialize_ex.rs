use crate::emu;
use crate::windows::constants::errors::*;

pub fn CoInitializeEx(emu: &mut emu::Emu) {
    let co_init = emu.regs().rdx;
    log_red!(emu, "ole32!CoInitializeEx dwCoInit: 0x{:x}", co_init);
    emu.set_last_error(0);
    emu.regs_mut().rax = S_OK;
}
