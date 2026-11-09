use crate::emu;

pub fn GetExitCodeProcessImplementation(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let exit_code_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetExitCodeProcessImplementation h:0x{:x} ptr:0x{:x}",
        h_process,
        exit_code_ptr
    );
    if exit_code_ptr != 0 {
        emu.maps.write_dword(exit_code_ptr, 259);
    }
    emu.regs_mut().rax = 1;
}
