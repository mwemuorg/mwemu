use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::structures;

pub fn SystemTimeToFileTime(emu: &mut emu::Emu) {
    let out_ptr = arg(emu, 1) as u64;
    log_red!(emu, "kernel32!SystemTimeToFileTime");
    structures::SystemTime::now().save(out_ptr, &mut emu.maps);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
