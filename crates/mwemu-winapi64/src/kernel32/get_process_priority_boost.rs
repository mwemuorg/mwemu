use crate::emu;

pub fn GetProcessPriorityBoost(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let disable_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetProcessPriorityBoost h:0x{:x} ptr:0x{:x}",
        h_process,
        disable_ptr
    );
    if disable_ptr != 0 {
        emu.maps.write_dword(disable_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
