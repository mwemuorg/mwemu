use crate::emu;

pub fn QueryProcessCycleTime(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let p_cycle_time = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!QueryProcessCycleTime hProcess:0x{:x} pCycleTime:0x{:x}",
        h_process,
        p_cycle_time
    );
    if p_cycle_time != 0 {
        emu.maps.write_qword(p_cycle_time, emu.tick);
    }
    emu.regs_mut().rax = 1;
}
