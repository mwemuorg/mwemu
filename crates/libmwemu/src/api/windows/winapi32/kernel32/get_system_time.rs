use crate::emu;
use crate::windows::structures;

pub fn GetSystemTime(emu: &mut emu::Emu) {
    let out_time = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    log_red!(emu, "kernel32!GetSystemTime ptr: 0x{:x}'", out_time);
    let systime = structures::SystemTime::now();
    systime.save(out_time, &mut emu.maps);

    emu.stack_pop32(false);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
