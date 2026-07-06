use crate::emu;

pub fn LocalAlloc(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx;
    let size = emu.regs().rdx;

    log_red!(emu, "kernel32!LocalAlloc flags: {:x} sz: {}", flags, size);

    let heap_management = emu.heap_mut();
    let base = heap_management.allocate(size as usize);
    emu.regs_mut().rax = base.unwrap();
}
