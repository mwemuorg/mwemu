use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi64;

pub fn GetLastError(emu: &mut emu::Emu) {
    helper::advance_tick(emu, 1);
    winapi64::kernel32::GetLastError(emu);
}
