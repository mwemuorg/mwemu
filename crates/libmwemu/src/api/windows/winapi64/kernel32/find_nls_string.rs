use crate::emu;

pub fn FindNLSString(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx as u32;
    let flags = emu.regs().rdx as u32;
    let source_ptr = emu.regs().r8;
    let source_count = emu.regs().r9 as i32;
    log_red!(
        emu,
        "kernel32!FindNLSString locale:0x{:x} flags:0x{:x} src:0x{:x}",
        locale,
        flags,
        source_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = (-1i64) as u64;
}
