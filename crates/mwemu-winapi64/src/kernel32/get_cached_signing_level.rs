use crate::emu;

pub fn GetCachedSigningLevel(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let flags_ptr = emu.regs().rdx;
    let signing_level_ptr = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GetCachedSigningLevel h:0x{:x} flags:0x{:x} level:0x{:x}",
        h_file,
        flags_ptr,
        signing_level_ptr
    );
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    if signing_level_ptr != 0 {
        emu.maps.write_byte(signing_level_ptr, 0);
    }
    emu.regs_mut().rax = 0;
}
