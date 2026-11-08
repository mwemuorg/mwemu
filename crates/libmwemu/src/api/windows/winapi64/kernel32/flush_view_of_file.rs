use crate::emu;
use crate::winapi::helper;
pub fn FlushViewOfFile(emu: &mut emu::Emu) {
    let lp_base = emu.regs().rcx;
    let num_bytes = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!FlushViewOfFile base:0x{:x} bytes:{}",
        lp_base,
        num_bytes
    );
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
