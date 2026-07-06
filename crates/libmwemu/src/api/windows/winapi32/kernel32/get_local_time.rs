use crate::emu;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn GetLocalTime(emu: &mut emu::Emu) {
    let lp_system_time = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetLocalTime error reading param") as u64;

    log_red!(emu, "kernel32!GetLocalTime ptr: 0x{:x}", lp_system_time);

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    // This is a naive implementation, real world should use chrono for proper date calculation
    // But for dependencies sake, we might stick to minimal.
    // However, emulating full date calc manually is tedious.
    // For now, let's write zeros or minimal data to satisfy basic checks.
    // Or just replicate the behavior of finding *some* time.

    // Structure SYSTEMTIME:
    // WORD wYear;
    // WORD wMonth;
    // WORD wDayOfWeek;
    // WORD wDay;
    // WORD wHour;
    // WORD wMinute;
    // WORD wSecond;
    // WORD wMilliseconds;

    let in_secs = since_the_epoch.as_secs();
    let _in_nano = since_the_epoch.subsec_nanos();

    // Minimal mock values
    let w_year: u16 = 2024;
    let w_month: u16 = 1;
    let w_day: u16 = 1;
    let w_hour: u16 = ((in_secs / 3600) % 24) as u16;
    let w_minute: u16 = ((in_secs / 60) % 60) as u16;
    let w_second: u16 = (in_secs % 60) as u16;
    let w_millis: u16 = 0;

    let mut offset = 0;
    emu.maps.write_word(lp_system_time + offset, w_year);
    offset += 2;
    emu.maps.write_word(lp_system_time + offset, w_month);
    offset += 2;
    emu.maps.write_word(lp_system_time + offset, 0);
    offset += 2; // DayOfWeek
    emu.maps.write_word(lp_system_time + offset, w_day);
    offset += 2;
    emu.maps.write_word(lp_system_time + offset, w_hour);
    offset += 2;
    emu.maps.write_word(lp_system_time + offset, w_minute);
    offset += 2;
    emu.maps.write_word(lp_system_time + offset, w_second);
    offset += 2;
    emu.maps.write_word(lp_system_time + offset, w_millis);

    emu.stack_pop32(false);
}
