use crate::emu;

pub fn QueryPerformanceFrequency(emu: &mut emu::Emu) {
    let freq_ptr = emu.regs().rcx;
    if freq_ptr != 0 {
        emu.maps.write_qword(freq_ptr, 10000000);
    }
    log_red!(emu, "kernelbase!QueryPerformanceFrequency => 10000000");
    emu.regs_mut().rax = 1;
}
