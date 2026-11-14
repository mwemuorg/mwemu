use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::structures;

pub fn GetSystemTime(emu: &mut emu::Emu) {
    let out_time = WinApiAbi::arg(emu, 0);

    log_red!(emu, "kernel32!GetSystemTime ptr: 0x{:x}'", out_time);
    let systime = structures::SystemTime::now();
    systime.save(out_time, &mut emu.maps);

    emu.stack_pop32(false);
}
