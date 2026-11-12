use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::msvcrt::allocate_memory;

pub fn localtime(emu: &mut emu::Emu) {
    let time_ptr = WinApiAbi::arg(emu, 0);

    let timestamp = if time_ptr != 0 {
        emu.maps.read_dword(time_ptr).unwrap_or(0) as i64
    } else {
        0
    };
    let tm_addr = allocate_memory(emu, 36).unwrap_or(0);
    if tm_addr != 0 {
        // struct tm: sec, min, hour, mday, mon, year, wday, yday, isdst
        let secs = timestamp % 60;
        let mins = (timestamp / 60) % 60;
        let hours = (timestamp / 3600) % 24;
        let days = timestamp / 86400;
        let wday = ((days + 4) % 7) as u32; // 1970-01-01 was Thursday
        let mut y = 1970i64;
        let mut rem = days;
        loop {
            let ylen: i64 = if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) {
                366
            } else {
                365
            };
            if rem < ylen {
                break;
            }
            rem -= ylen;
            y += 1;
        }
        let leap = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);
        let mdays: [i64; 12] = [
            31,
            if leap { 29 } else { 28 },
            31,
            30,
            31,
            30,
            31,
            31,
            30,
            31,
            30,
            31,
        ];
        let mut mon = 0u32;
        for md in &mdays {
            if rem < *md {
                break;
            }
            rem -= *md;
            mon += 1;
        }
        emu.maps.write_dword(tm_addr, secs as u32);
        emu.maps.write_dword(tm_addr + 4, mins as u32);
        emu.maps.write_dword(tm_addr + 8, hours as u32);
        emu.maps.write_dword(tm_addr + 12, (rem + 1) as u32);
        emu.maps.write_dword(tm_addr + 16, mon);
        emu.maps.write_dword(tm_addr + 20, (y - 1900) as u32);
        emu.maps.write_dword(tm_addr + 24, wday);
        emu.maps.write_dword(tm_addr + 28, (days % 365) as u32);
        emu.maps.write_dword(tm_addr + 32, 0); // isdst
    }
    log_red!(emu, "msvcrt!localtime ts:{} => 0x{:x}", timestamp, tm_addr);
    emu.regs_mut().rax = tm_addr;
}
