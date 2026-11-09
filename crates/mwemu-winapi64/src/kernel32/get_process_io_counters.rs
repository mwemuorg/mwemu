use crate::emu;

pub fn GetProcessIoCounters(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let counters_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetProcessIoCounters h:0x{:x} ptr:0x{:x}",
        h_process,
        counters_ptr
    );
    if counters_ptr != 0 {
        for i in 0..6 {
            emu.maps.write_qword(counters_ptr + i * 8, 0);
        }
    }
    emu.regs_mut().rax = 1;
}
