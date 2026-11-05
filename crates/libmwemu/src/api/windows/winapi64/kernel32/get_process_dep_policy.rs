use crate::emu;

pub fn GetProcessDEPPolicy(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let flags_ptr = emu.regs().rdx;
    let permanent_ptr = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!GetProcessDEPPolicy h:0x{:x} flags:0x{:x} perm:0x{:x}",
        h_process,
        flags_ptr,
        permanent_ptr
    );
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 1);
    }
    if permanent_ptr != 0 {
        emu.maps.write_dword(permanent_ptr, 1);
    }
    emu.regs_mut().rax = 1;
}
