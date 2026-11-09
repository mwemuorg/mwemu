use crate::emu;
use crate::winapi::helper;
pub fn FreeUserPhysicalPages(emu: &mut emu::Emu) {
    let process = emu.regs().rcx;
    let base_address = emu.regs().rdx;
    let page_array = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!FreeUserPhysicalPages proc:0x{:x} addr:0x{:x}",
        process,
        base_address
    );
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}
