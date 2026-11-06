use crate::{emu, windows::structures};

pub fn GetSystemTime(emu: &mut emu::Emu) {
    let out_time = emu.regs().rcx;

    log_red!(emu, "kernel32!GetSystemTime ptr: 0x{:x}'", out_time);
    if emu.regs().rcx != 0 && emu.maps.is_mapped(emu.regs().rcx) {
        for i in 0..8u64 {
            emu.maps.write_word(emu.regs().rcx + i * 2, 0);
        }
    }

    let systime = structures::SystemTime::now();
    systime.save(out_time, &mut emu.maps);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
