use crate::{emu, windows::structures};

pub fn SystemTimeToFileTime(emu: &mut emu::Emu) {
    let in_ptr = emu.regs().rcx;
    let out_ptr = emu.regs().rdx;

    let now = structures::SystemTime::now();
    now.save(out_ptr, &mut emu.maps);

    log_red!(emu, "kernel32!SystemTimeToFileTime");
}
