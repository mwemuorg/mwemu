use crate::emu::Emu;
use crate::winapi::libraries::kernel32::file_time::win_performance;

pub fn QueryPerformanceFrequency(emu: &mut Emu) {
    win_performance(emu, true);
}
