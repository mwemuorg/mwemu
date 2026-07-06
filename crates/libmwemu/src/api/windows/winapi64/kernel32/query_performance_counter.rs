use crate::emu;

pub fn QueryPerformanceCounter(emu: &mut emu::Emu) {
    let counter_ptr = emu.regs().rcx;

    // Use emu.tick directly, maybe scaled up to simulate higher frequency
    let counter_value = (emu.tick as u64) * 1000; // Scale to simulate ~1MHz frequency

    emu.maps.write_qword(counter_ptr, counter_value);

    log_red!(
        emu,
        "kernel32!QueryPerformanceCounter counter: {}",
        counter_value
    );

    emu.regs_mut().rax = 1; // Return TRUE
}
