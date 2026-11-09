use crate::emu;
use crate::winapi::helper;

pub fn FreePropVariantArray(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    helper::handler_close(handle);
    log_red!(emu, "ole32!FreePropVariantArray handle:0x{:x}", handle);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
