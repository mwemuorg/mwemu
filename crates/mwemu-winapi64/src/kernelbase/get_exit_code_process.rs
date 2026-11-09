use crate::emu;

pub fn GetExitCodeProcess(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let exit_code_ptr = emu.regs().rdx;
    if exit_code_ptr != 0 {
        emu.maps.write_dword(exit_code_ptr, 259); // STILL_ACTIVE
    }
    log_red!(emu, "kernelbase!GetExitCodeProcess handle:0x{:x}", handle);
    emu.regs_mut().rax = 1;
}
