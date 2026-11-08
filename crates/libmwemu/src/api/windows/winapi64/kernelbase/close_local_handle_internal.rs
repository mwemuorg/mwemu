use crate::emu;
use crate::winapi::helper;

pub fn CLOSE_LOCAL_HANDLE_INTERNAL(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    helper::handler_close(handle);
    log_red!(
        emu,
        "kernelbase!CLOSE_LOCAL_HANDLE_INTERNAL handle:0x{:x}",
        handle
    );
    emu.regs_mut().rax = 1;
}
