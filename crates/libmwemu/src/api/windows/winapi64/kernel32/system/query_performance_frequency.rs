use crate::emu;

pub fn QueryPerformanceFrequency(emu: &mut emu::Emu) {
    let out_ptr = emu.regs().rcx;

    log_red!(emu, "kernel32!QueryPerformanceFrequency");

    emu.maps.write_qword(out_ptr, 10_000_000);
    emu.regs_mut().rax = 1;
}
