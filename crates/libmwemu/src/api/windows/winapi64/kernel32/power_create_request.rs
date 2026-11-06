
use crate::emu;
use crate::winapi::helper;
pub fn PowerCreateRequest(emu: &mut emu::Emu) {
    let p_context = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!PowerCreateRequest pContext:0x{:x}",
        p_context
    );
    let handle = helper::handler_create("power://");
    emu.regs_mut().rax = handle;
}
