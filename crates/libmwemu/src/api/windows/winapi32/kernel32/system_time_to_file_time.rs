use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SystemTimeToFileTime(emu: &mut emu::Emu) {
    let in_ptr = arg(emu, 0) as u64;
    let out_ptr = arg(emu, 1) as u64;

    log_red!(
        emu,
        "kernel32!SystemTimeToFileTime st:0x{:x} ft:0x{:x}",
        in_ptr,
        out_ptr
    );

    if in_ptr == 0 || out_ptr == 0 {
        emu.set_last_error(87);
        stdcall_ret(emu, 2);
        emu.regs_mut().rax = 0;
        return;
    }

    let year = emu.maps.read_word(in_ptr).unwrap_or(2024) as u64;
    let month = emu.maps.read_word(in_ptr + 2).unwrap_or(1) as u64;
    let _dow = emu.maps.read_word(in_ptr + 4).unwrap_or(0);
    let day = emu.maps.read_word(in_ptr + 6).unwrap_or(1) as u64;
    let hour = emu.maps.read_word(in_ptr + 8).unwrap_or(0) as u64;
    let minute = emu.maps.read_word(in_ptr + 10).unwrap_or(0) as u64;
    let second = emu.maps.read_word(in_ptr + 12).unwrap_or(0) as u64;
    let millis = emu.maps.read_word(in_ptr + 14).unwrap_or(0) as u64;

    let total_days = ymd_to_days(year, month, day);
    let total_secs = total_days * 86400 + hour * 3600 + minute * 60 + second;
    let filetime = total_secs * 10_000_000 + millis * 10_000;

    emu.maps
        .write_dword(out_ptr, (filetime & 0xFFFF_FFFF) as u32);
    emu.maps.write_dword(out_ptr + 4, (filetime >> 32) as u32);

    emu.set_last_error(0);
    stdcall_ret(emu, 2);
    emu.regs_mut().rax = 1;
}

fn is_leap_year(y: u64) -> bool {
    (y.is_multiple_of(4) && !y.is_multiple_of(100)) || y.is_multiple_of(400)
}

fn ymd_to_days(year: u64, month: u64, day: u64) -> u64 {
    fn leaps_before(y: u64) -> u64 {
        if y <= 1601 {
            return 0;
        }
        let base = 1600u64;
        let a = (y - 1) / 4 - base / 4;
        let b = (y - 1) / 100 - base / 100;
        let c = (y - 1) / 400 - base / 400;
        a - b + c
    }

    let years_elapsed = year - 1601;
    let leap_count = leaps_before(year);
    let mut days = years_elapsed * 365 + leap_count;

    let month_days = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for m in 1..month {
        days += month_days[m as usize];
        if m == 2 && is_leap_year(year) {
            days += 1;
        }
    }
    days + day - 1
}
