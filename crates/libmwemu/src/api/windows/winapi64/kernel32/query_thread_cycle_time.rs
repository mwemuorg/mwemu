use crate::emu;

pub fn QueryThreadCycleTime(emu: &mut emu::Emu) {
    let h_thread = emu.regs().rcx;
    let p_cycle_time = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!QueryThreadCycleTime hThread:0x{:x} pCycleTime:0x{:x}",
        h_thread,
        p_cycle_time
    );
    if p_cycle_time != 0 {
        emu.maps.write_qword(p_cycle_time, emu.tick);
    }
    emu.regs_mut().rax = 1;
}
