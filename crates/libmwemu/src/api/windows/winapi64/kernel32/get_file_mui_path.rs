use crate::emu;

pub fn GetFileMUIPath(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx as u32;
    let file_path_ptr = emu.regs().rdx;
    let language_ptr = emu.regs().r8;
    let language_count_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetFileMUIPath flags:0x{:x} path:0x{:x}",
        flags,
        file_path_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
