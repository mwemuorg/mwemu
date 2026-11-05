use crate::emu;

pub fn GetDurationFormatEx(emu: &mut emu::Emu) {
    let locale_ptr = emu.regs().rcx;
    let flags = emu.regs().rdx as u32;
    let duration_ptr = emu.regs().r8;
    let duration_val = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetDurationFormatEx locale:0x{:x} flags:0x{:x}",
        locale_ptr,
        flags
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
