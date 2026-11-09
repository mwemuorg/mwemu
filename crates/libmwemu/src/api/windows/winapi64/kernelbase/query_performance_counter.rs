use crate::emu;
use crate::winapi::helper;

pub fn QueryPerformanceCounter(emu: &mut emu::Emu) {
    let counter_ptr = emu.regs().rcx;
    helper::advance_tick(emu, 1);
    if counter_ptr != 0 {
        emu.maps.write_qword(counter_ptr, emu.tick as u64);
    }
    log_red!(emu, "kernelbase!QueryPerformanceCounter => {}", emu.tick);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
