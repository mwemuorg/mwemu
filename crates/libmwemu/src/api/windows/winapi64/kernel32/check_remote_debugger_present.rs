use crate::emu;

pub fn CheckRemoteDebuggerPresent(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let pb_debugger_present = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!CheckRemoteDebuggerPresent hProcess:0x{:x} pbDebuggerPresent:0x{:x}",
        h_process,
        pb_debugger_present
    );
    if pb_debugger_present != 0 {
        emu.maps.write_dword(pb_debugger_present, 0);
    }
    emu.regs_mut().rax = 1;
}
