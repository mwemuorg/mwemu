use crate::emu;
use crate::winapi::helper;
pub fn VirtualFree(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;
    let sz = emu.regs().rdx;
    log_red!(emu, "kernel32!VirtualFree {} bytes at 0x{:x}", sz, addr);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    // zero out the memory?
    for i in 0..sz {
        emu.maps.write_byte(addr + i, 0);
    }
    // TODO: do something (dump+free or not free)
    emu.regs_mut().rax = 1;
}
