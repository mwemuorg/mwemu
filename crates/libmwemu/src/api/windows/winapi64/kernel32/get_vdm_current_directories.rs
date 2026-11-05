use crate::emu;

pub fn GetVDMCurrentDirectories(emu: &mut emu::Emu) {
    let drive_count = emu.regs().rcx as u32;
    let dirs_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetVDMCurrentDirectories drives:{} ptr:0x{:x}",
        drive_count,
        dirs_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
