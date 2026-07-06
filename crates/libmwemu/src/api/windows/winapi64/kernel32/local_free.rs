use crate::emu;

pub fn LocalFree(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;

    log_red!(emu, "kernel32!LocalFree flags: {:x}", addr);

    let heap_management = emu.heap_mut();
    heap_management.free(addr);
    emu.regs_mut().rax = 0;
}
