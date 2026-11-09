use crate::emu;

pub fn GetExitCodeThread(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let exit_code_ptr = emu.regs().rdx;
    log_red!(emu, "kernel32!GetExitCodeThread h:0x{:x}", handle);
    if exit_code_ptr != 0 {
        emu.maps.write_dword(exit_code_ptr, 259); // STILL_ACTIVE
    }
    emu.regs_mut().rax = 1;
}
