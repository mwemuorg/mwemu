use crate::emu::Emu;
use crate::winapi::libraries::kernel32::file_time::win_performance;

pub fn QueryPerformanceCounter(emu: &mut Emu) {
    win_performance(emu, false);
}
