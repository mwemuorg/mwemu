use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi64;

pub fn GetCurrentProcess(emu: &mut emu::Emu) {
    helper::advance_tick(emu, 1);
    winapi64::kernel32::GetCurrentProcess(emu);
}
