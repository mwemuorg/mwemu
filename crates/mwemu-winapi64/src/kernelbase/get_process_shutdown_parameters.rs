use crate::emu;

pub fn GetProcessShutdownParameters(emu: &mut emu::Emu) {
    let level_ptr = emu.regs().rcx;
    let flags_ptr = emu.regs().rdx;
    if level_ptr != 0 {
        emu.maps.write_dword(level_ptr, 0x280); // default = 640
    }
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0);
    }
    log_red!(emu, "kernelbase!GetProcessShutdownParameters => 0x280");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
