use crate::emu64;
use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::winapi64;
use crate::windows::constants::errors::*;
use crate::windows::constants::nls::*;

const PERFORMANCE_COUNTER_FREQUENCY: u64 = 10_000_000;
const HUNDRED_NS_PER_MILLISECOND: i64 = 10_000;

#[test]
fn get_tick_count64() {
    helpers::setup();
    let mut emu = emu64();

    let tick = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetTickCount64, &[]);
    assert!(tick < u64::MAX, "GetTickCount64 returned max u64");
}

#[test]
fn query_performance_frequency_64() {
    helpers::setup();
    let mut emu = emu64();

    let freq_addr = 0x100000u64;
    emu.maps
        .create_map("freq", freq_addr, 0x1000, Permission::READ_WRITE);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::QueryPerformanceFrequency,
        &[freq_addr],
    );
    assert_eq!(ret, 1, "QueryPerformanceFrequency must return TRUE");
    let freq = emu.maps.read_qword(freq_addr).unwrap();
    assert!(freq > 0, "frequency must be non-zero");
}

#[test]
fn get_tick_count64_advances_emulated_time() {
    helpers::setup();
    let mut emu = emu64();

    emu.tick = 12345;
    let ret = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetTickCount64, &[]);
    assert_eq!(ret, 12345, "GetTickCount64 should return emu.tick");
    let again = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetTickCount64, &[]);
    assert_eq!(
        again, 12346,
        "by default every read advances the clock by 1 ms"
    );
}

#[test]
fn get_tick_count64_realistic() {
    helpers::setup();
    let mut emu = emu64();
    emu.cfg.realistic_tick = true;

    emu.tick = 12345;
    let ret = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetTickCount64, &[]);
    // 64 Hz timer: 12345 ms is 790 interrupts of 15.625 ms = 12343 ms
    assert_eq!(
        ret, 12343,
        "GetTickCount64 should round down to a timer interrupt"
    );
    let again = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetTickCount64, &[]);
    assert_eq!(again, ret, "back-to-back reads must not advance the clock");
}

#[test]
fn get_tick_count_realistic_back_to_back() {
    helpers::setup();
    let mut emu = emu64();
    emu.cfg.realistic_tick = true;

    emu.tick = 1000;
    let reads: Vec<u64> = (0..6)
        .map(|_| helpers::call_winapi64(&mut emu, winapi64::kernel32::GetTickCount, &[]))
        .collect();
    assert!(
        reads.iter().all(|&t| t == 1000),
        "reads without elapsed time must be equal: {reads:?}"
    );
    emu.tick = 1016;
    let later = helpers::call_winapi64(&mut emu, winapi64::kernel32::GetTickCount, &[]);
    assert_eq!(
        later, 1015,
        "the clock moves in 15.625 ms steps once time passes"
    );
}

#[test]
fn query_performance_frequency() {
    helpers::setup();
    let mut emu = emu64();

    let out_addr = 0x100000u64;
    emu.maps
        .create_map("qpf", out_addr, 0x1000, Permission::READ_WRITE);

    let ret = helpers::call_winapi64(
        &mut emu,
        winapi64::kernel32::QueryPerformanceFrequency,
        &[out_addr],
    );
    assert_eq!(ret, 1, "QueryPerformanceFrequency should return TRUE");
    assert_eq!(
        emu.maps.read_qword(out_addr).unwrap(),
        PERFORMANCE_COUNTER_FREQUENCY,
        "frequency should be 10MHz"
    );
}

#[test]
fn nt_delay_execution() {
    helpers::setup();
    let mut emu = emu64();

    let interval_addr = 0x100000u64;
    emu.maps
        .create_map("delay", interval_addr, 0x1000, Permission::READ_WRITE);

    let start_tick = emu.tick;

    let interval = -500 * HUNDRED_NS_PER_MILLISECOND;
    emu.maps.write_qword(interval_addr, interval as u64);

    helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtDelayExecution,
        &[0, interval_addr],
    );

    assert!(
        emu.tick > start_tick,
        "NtDelayExecution should advance tick (start: {}, now: {})",
        start_tick,
        emu.tick
    );
}

#[test]
fn calendar_info_uses_shared_ansi_unicode_sizes_and_numeric_outputs() {
    const GUARD: u8 = 0xcc;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let mut targets = vec![
            ("kernel32.dll", "GetCalendarInfoA", false),
            ("kernel32.dll", "GetCalendarInfoW", true),
        ];
        if is64 {
            targets.push(("kernelbase.dll", "GetCalendarInfoW", true));
        }
        for (library, api, wide) in targets {
            for (kind, text) in [
                (CAL_SCALNAME, "Gregorian Calendar"),
                (CAL_SDAYNAME1, "Monday"),
                (CAL_SMONTHNAME1, "January"),
            ] {
                let expected: Vec<u8> = if wide {
                    text.encode_utf16()
                        .chain([0])
                        .flat_map(u16::to_le_bytes)
                        .collect()
                } else {
                    text.bytes().chain([0]).collect()
                };
                let count = text.len() as u64 + 1;
                let mut args = [LOCALE_USER_DEFAULT, CAL_GREGORIAN as u64, kind, 0, 0, 0];
                emu.set_last_error(GUARD as u64);
                assert_eq!(
                    helpers::call_winapi_export(&mut emu, library, api, &args),
                    count,
                    "{api} size query"
                );
                assert_eq!(emu.get_last_error(), GUARD as u64);
                assert!(emu.maps.write_bytes(data, &[GUARD; 64]));
                args[3] = data;
                args[4] = count - 1;
                assert_eq!(
                    helpers::call_winapi_export(&mut emu, library, api, &args),
                    0
                );
                assert_eq!(emu.get_last_error(), ERROR_INSUFFICIENT_BUFFER);
                assert_eq!(
                    emu.maps.try_read_bytes(data, 64).as_deref(),
                    Some([GUARD; 64].as_slice())
                );
                args[4] = count;
                assert_eq!(
                    helpers::call_winapi_export(&mut emu, library, api, &args),
                    count
                );
                assert_eq!(
                    emu.maps.try_read_bytes(data, expected.len()).as_deref(),
                    Some(expected.as_slice())
                );
                assert_eq!(
                    emu.maps.read_byte(data + expected.len() as u64),
                    Some(GUARD)
                );
            }
            assert!(emu.maps.write_bytes(data, &[GUARD; 8]));
            let args = [
                EN_US_LOCALE as u64,
                CAL_GREGORIAN as u64,
                CAL_ICALINTVALUE | CAL_RETURN_NUMBER,
                0,
                0,
                data,
            ];
            let unit_size = if wide {
                size_of::<u16>()
            } else {
                size_of::<u8>()
            };
            assert_eq!(
                helpers::call_winapi_export(&mut emu, library, api, &args),
                (size_of::<u32>() / unit_size) as u64
            );
            assert_eq!(emu.maps.read_dword(data), Some(CAL_GREGORIAN));
            assert_eq!(
                emu.maps.read_byte(data + size_of::<u32>() as u64),
                Some(GUARD)
            );
            let mut invalid = args;
            invalid[5] = 0;
            assert_eq!(
                helpers::call_winapi_export(&mut emu, library, api, &invalid),
                0
            );
            assert_eq!(emu.get_last_error(), ERROR_INVALID_PARAMETER);
            invalid = args;
            invalid[2] = CAL_SCALNAME | CAL_RETURN_NUMBER;
            assert_eq!(
                helpers::call_winapi_export(&mut emu, library, api, &invalid),
                0
            );
            assert_eq!(emu.get_last_error(), ERROR_INVALID_FLAGS);
        }
    }
}
