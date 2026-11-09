use crate::emu;

pub fn GetNamedPipeServerSessionId(emu: &mut emu::Emu) {
    let h_pipe = emu.regs().rcx;
    let session_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetNamedPipeServerSessionId h:0x{:x} ptr:0x{:x}",
        h_pipe,
        session_ptr
    );
    if session_ptr != 0 {
        emu.maps.write_dword(session_ptr, 1);
    }
    emu.regs_mut().rax = 1;
}
