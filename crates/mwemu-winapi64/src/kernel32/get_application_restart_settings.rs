use crate::emu;

pub fn GetApplicationRestartSettings(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let cmdline_ptr = emu.regs().rdx;
    let size_ptr = emu.regs().r8;
    let flags_ptr = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!GetApplicationRestartSettings h:0x{:x}",
        h_process
    );
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, 0);
    }
    emu.regs_mut().rax = 0x80004005u64;
}
