use crate::emu;

pub fn FileTimeToSystemTime(emu: &mut emu::Emu) {
    let p_file_time = emu.regs().rcx;
    let p_system_time = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!FileTimeToSystemTime ft:0x{:x} st:0x{:x}",
        p_file_time,
        p_system_time
    );

    if p_file_time == 0 || p_system_time == 0 {
        emu.set_last_error(87);
        emu.regs_mut().rax = 0;
        return;
    }

    let lo = emu.maps.read_dword(p_file_time).unwrap_or(0) as u64;
    let hi = emu.maps.read_dword(p_file_time + 4).unwrap_or(0) as u64;
    let ft = (hi << 32) | lo;

    // FILETIME is 100-nanosecond intervals since 1601-01-01
    let secs_since_1601 = ft / 10_000_000;
    let remaining_100ns = ft % 10_000_000;
    let millis = remaining_100ns / 10_000;

    // Days since 1601-01-01
    let total_days = secs_since_1601 / 86400;
    let day_secs = secs_since_1601 % 86400;

    let hour = (day_secs / 3600) as u16;
    let minute = ((day_secs % 3600) / 60) as u16;
    let second = (day_secs % 60) as u16;

    // Day of week (1601-01-01 was Monday = 1)
    let dow = ((total_days + 1) % 7) as u16;

    // Convert days since 1601-01-01 to year/month/day
    let (year, month, day) = days_to_ymd(total_days);

    // SYSTEMTIME: wYear, wMonth, wDayOfWeek, wDay, wHour, wMinute, wSecond, wMilliseconds
    emu.maps.write_word(p_system_time, year);
    emu.maps.write_word(p_system_time + 2, month);
    emu.maps.write_word(p_system_time + 4, dow);
    emu.maps.write_word(p_system_time + 6, day);
    emu.maps.write_word(p_system_time + 8, hour);
    emu.maps.write_word(p_system_time + 10, minute);
    emu.maps.write_word(p_system_time + 12, second);
    emu.maps.write_word(p_system_time + 14, millis as u16);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

fn is_leap_year(y: u64) -> bool {
    (y.is_multiple_of(4) && !y.is_multiple_of(100)) || y.is_multiple_of(400)
}

fn days_in_month(y: u64, m: u64) -> u64 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(y) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

fn days_to_ymd(mut total_days: u64) -> (u16, u16, u16) {
    // 400-year cycle has 146097 days
    let n400 = total_days / 146097;
    total_days %= 146097;

    // 100-year cycle has 36524 days (no leap on century)
    let mut n100 = total_days / 36524;
    if n100 == 4 {
        n100 = 3;
    }
    total_days -= n100 * 36524;

    // 4-year cycle has 1461 days
    let n4 = total_days / 1461;
    total_days %= 1461;

    // Single years have 365 days
    let mut n1 = total_days / 365;
    if n1 == 4 {
        n1 = 3;
    }
    total_days -= n1 * 365;

    let year = 1601 + n400 * 400 + n100 * 100 + n4 * 4 + n1;

    let mut month = 1u64;
    while month <= 12 {
        let dim = days_in_month(year, month);
        if total_days < dim {
            break;
        }
        total_days -= dim;
        month += 1;
    }

    (year as u16, month as u16, (total_days + 1) as u16)
}
