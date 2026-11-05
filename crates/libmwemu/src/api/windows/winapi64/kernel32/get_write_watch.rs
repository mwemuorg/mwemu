use crate::emu;

pub fn GetWriteWatch(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx as u32;
    let base_addr = emu.regs().rdx;
    let region_size = emu.regs().r8;
    let addresses_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetWriteWatch flags:0x{:x} base:0x{:x} size:0x{:x}",
        flags,
        base_addr,
        region_size
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
