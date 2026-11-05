use crate::{emu, windows::structures};

pub fn GetSystemTime(emu: &mut emu::Emu) {
    let out_time = emu.regs().rcx;

    log_red!(emu, "kernel32!GetSystemTime ptr: 0x{:x}'", out_time);

    let systime = structures::SystemTime::now();
    systime.save(out_time, &mut emu.maps);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
