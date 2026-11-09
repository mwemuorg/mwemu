use crate::emu;

pub fn FindNLSStringEx(emu: &mut emu::Emu) {
    let locale_ptr = emu.regs().rcx;
    let flags = emu.regs().rdx as u32;
    let source_ptr = emu.regs().r8;
    let source_count = emu.regs().r9 as i32;
    log_red!(
        emu,
        "kernel32!FindNLSStringEx locale:0x{:x} flags:0x{:x} src:0x{:x}",
        locale_ptr,
        flags,
        source_ptr
    );
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = (-1i64) as u64;
}
