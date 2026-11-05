use crate::emu;

pub fn GetSystemAppDataKey(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx as u32;
    let path_ptr = emu.regs().rdx;
    let path_len_ptr = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GetSystemAppDataKey flags:0x{:x} path:0x{:x}",
        flags,
        path_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x80004005u64;
}
