use crate::emu;

pub fn FindStringOrdinal(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx as u32;
    let source_ptr = emu.regs().rdx;
    let source_count = emu.regs().r8 as i32;
    let value_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!FindStringOrdinal flags:0x{:x} src:0x{:x} val:0x{:x}",
        flags,
        source_ptr,
        value_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = (-1i64) as u64;
}
