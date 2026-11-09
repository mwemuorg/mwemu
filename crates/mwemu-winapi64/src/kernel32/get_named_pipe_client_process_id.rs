use crate::emu;

pub fn GetNamedPipeClientProcessId(emu: &mut emu::Emu) {
    let h_pipe = emu.regs().rcx;
    let pid_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetNamedPipeClientProcessId h:0x{:x} ptr:0x{:x}",
        h_pipe,
        pid_ptr
    );
    if pid_ptr != 0 {
        emu.maps.write_dword(pid_ptr, 0x123);
    }
    emu.regs_mut().rax = 1;
}
