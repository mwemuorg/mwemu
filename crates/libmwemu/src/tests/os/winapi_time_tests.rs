use crate::api::windows::policy::ApiMode;
use crate::emu64;
use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::winapi64;
use crate::windows::constants::errors::*;
use crate::windows::constants::nls::*;
use crate::windows::constants::system::{TIME_ZONE_ID_INVALID, TIME_ZONE_ID_UNKNOWN};
use itertools::iproduct;

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

#[test]
fn utc_file_time_and_zone_queries_agree_across_architectures_and_libraries() {
    const FILE_TIME: u64 = 0x01da_1234_5678_9abc;
    const ZONE_BYTES: usize = 172;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        let output = data + 0x100;
        for library in ["kernel32.dll", "kernelbase.dll"] {
            for mode in [
                ApiMode::Strict,
                ApiMode::SimulateFail,
                ApiMode::SimulateSuccess,
            ] {
                emu.set_api_mode(mode);
                for name in ["FileTimeToLocalFileTime", "LocalFileTimeToFileTime"] {
                    emu.set_last_error(ERROR_ACCESS_DENIED);
                    assert!(emu.maps.write_qword(data, FILE_TIME));
                    assert!(emu.maps.write_bytes(output - 1, &[0xa5; 10]));
                    assert_eq!(
                        helpers::call_winapi_export(&mut emu, library, name, &[data, output]),
                        1
                    );
                    assert_eq!(emu.maps.read_qword(output), Some(FILE_TIME));
                    assert_eq!(emu.maps.read_byte(output - 1), Some(0xa5));
                    assert_eq!(emu.maps.read_byte(output + 8), Some(0xa5));
                    assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
                    assert_eq!(emu.api_policy.simulation_count, 0);
                    for (input, destination, error) in [
                        (0, output, ERROR_INVALID_PARAMETER),
                        (data, data, ERROR_INVALID_PARAMETER),
                        (data + 0xfff, output, ERROR_NOACCESS),
                        (data, data + 0xfff, ERROR_NOACCESS),
                    ] {
                        assert_eq!(
                            helpers::call_winapi_export(
                                &mut emu,
                                library,
                                name,
                                &[input, destination]
                            ),
                            0
                        );
                        assert_eq!(emu.get_last_error(), error);
                        assert_eq!(emu.maps.read_qword(output), Some(FILE_TIME));
                    }
                }
                emu.set_last_error(ERROR_ACCESS_DENIED);
                assert!(emu.maps.write_bytes(output - 1, &[0xa5; ZONE_BYTES + 2]));
                assert_eq!(
                    helpers::call_winapi_export(
                        &mut emu,
                        library,
                        "GetTimeZoneInformation",
                        &[output]
                    ),
                    TIME_ZONE_ID_UNKNOWN as u64
                );
                assert_eq!(emu.maps.read_dword(output), Some(0));
                assert_eq!(emu.maps.read_wide_string(output + 4), "UTC");
                assert_eq!(emu.maps.read_wide_string(output + 88), "UTC");
                assert_eq!(
                    emu.maps.read_word(output + 70),
                    Some(0),
                    "no standard transition month"
                );
                assert_eq!(
                    emu.maps.read_word(output + 154),
                    Some(0),
                    "no daylight transition month"
                );
                assert_eq!(emu.maps.read_byte(output - 1), Some(0xa5));
                assert_eq!(emu.maps.read_byte(output + ZONE_BYTES as u64), Some(0xa5));
                assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
                assert_eq!(
                    helpers::call_winapi_export(
                        &mut emu,
                        library,
                        "GetTimeZoneInformation",
                        &[data + 0xfff]
                    ),
                    TIME_ZONE_ID_INVALID as u64
                );
                assert_eq!(emu.get_last_error(), ERROR_NOACCESS);
            }
        }
    }
}

fn system_time_bytes(fields: [u16; 8]) -> Vec<u8> {
    fields.into_iter().flat_map(u16::to_le_bytes).collect()
}

#[test]
fn file_and_system_times_share_checked_conversions_and_exact_widths() {
    // SYSTEMTIME fields: year, month, weekday, day, hour, minute, second, milliseconds.
    let cases: [(&str, [u16; 8], u64); 4] = [
        ("FILETIME epoch", [1601, 1, 1, 1, 0, 0, 0, 0], 0),
        (
            "Unix epoch",
            [1970, 1, 4, 1, 0, 0, 0, 0],
            116_444_736_000_000_000,
        ),
        (
            "leap day with milliseconds",
            [2000, 2, 2, 29, 12, 34, 55, 999],
            125_963_012_959_990_000,
        ),
        (
            "last complete supported year",
            [30827, 12, 5, 31, 23, 59, 59, 999],
            9_223_149_887_999_990_000,
        ),
    ];
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        for library in ["kernel32.dll", "kernelbase.dll"] {
            for (name, fields, file_time) in cases {
                let system = data + 0x100;
                let output = data + 0x200;
                assert!(emu.maps.write_bytes(data, &file_time.to_le_bytes()));
                assert!(emu.maps.write_bytes(system, &[0xa5; 24]));
                emu.set_last_error(ERROR_ACCESS_DENIED);
                assert_eq!(
                    helpers::call_winapi_export(
                        &mut emu,
                        library,
                        "FileTimeToSystemTime",
                        &[data, system]
                    ),
                    1,
                    "{library}: {name}"
                );
                assert_eq!(
                    emu.maps.read_bytes_option(system, 16).unwrap().as_ref(),
                    system_time_bytes(fields)
                );
                assert_eq!(
                    emu.maps.read_qword(system + 16),
                    Some(0xa5a5_a5a5_a5a5_a5a5)
                );
                assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
                // wDayOfWeek is ignored on input.
                assert!(emu.maps.write_word(system + 4, u16::MAX));
                assert!(emu.maps.write_bytes(output, &[0xa5; 16]));
                assert_eq!(
                    helpers::call_winapi_export(
                        &mut emu,
                        library,
                        "SystemTimeToFileTime",
                        &[system, output]
                    ),
                    1,
                    "{library}: {name}"
                );
                assert_eq!(emu.maps.read_qword(output), Some(file_time));
                assert_eq!(emu.maps.read_qword(output + 8), Some(0xa5a5_a5a5_a5a5_a5a5));
                assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
            }
            assert!(emu.maps.write_qword(data, i64::MAX as u64));
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    library,
                    "FileTimeToSystemTime",
                    &[data, data + 0x100]
                ),
                1
            );
            assert_eq!(
                emu.maps
                    .read_bytes_option(data + 0x100, 16)
                    .unwrap()
                    .as_ref(),
                system_time_bytes([30828, 9, 4, 14, 2, 48, 5, 477])
            );
            for (left, right, expected) in [
                (0, 0, 0),
                (0, 1, u32::MAX as u64),
                (1, 0, 1),
                (u64::MAX, 0, 1),
            ] {
                assert!(emu.maps.write_qword(data, left));
                assert!(emu.maps.write_qword(data + 8, right));
                emu.set_last_error(ERROR_ACCESS_DENIED);
                assert_eq!(
                    helpers::call_winapi_export(
                        &mut emu,
                        library,
                        "CompareFileTime",
                        &[data, data + 8]
                    ),
                    expected
                );
                assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
            }
        }
    }
}

#[test]
fn time_conversion_rejects_invalid_dates_and_memory_without_partial_outputs() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        for library in ["kernel32.dll", "kernelbase.dll"] {
            for (name, fields) in [
                ("year before epoch", [1600, 1, 0, 1, 0, 0, 0, 0]),
                ("year beyond SYSTEMTIME range", [30828, 1, 0, 1, 0, 0, 0, 0]),
                ("month outside array", [2024, 13, 0, 1, 0, 0, 0, 0]),
                ("century is not a leap year", [1900, 2, 0, 29, 0, 0, 0, 0]),
                ("day zero", [2024, 1, 0, 0, 0, 0, 0, 0]),
                ("hour overflow", [2024, 1, 0, 1, 24, 0, 0, 0]),
                (
                    "milliseconds are not leap seconds",
                    [2024, 1, 0, 1, 0, 0, 59, 1000],
                ),
            ] {
                assert!(emu.maps.write_bytes(data, &system_time_bytes(fields)));
                assert!(emu.maps.write_qword(data + 0x100, u64::MAX));
                assert_eq!(
                    helpers::call_winapi_export(
                        &mut emu,
                        library,
                        "SystemTimeToFileTime",
                        &[data, data + 0x100]
                    ),
                    0,
                    "{name}"
                );
                assert_eq!(emu.get_last_error(), ERROR_INVALID_PARAMETER);
                assert_eq!(emu.maps.read_qword(data + 0x100), Some(u64::MAX));
            }
            assert!(emu.maps.write_qword(data, 1u64 << 63));
            assert!(emu.maps.write_bytes(data + 0x100, &[0xa5; 16]));
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    library,
                    "FileTimeToSystemTime",
                    &[data, data + 0x100]
                ),
                0
            );
            assert_eq!(emu.get_last_error(), ERROR_INVALID_PARAMETER);
            assert_eq!(
                emu.maps
                    .read_bytes_option(data + 0x100, 16)
                    .unwrap()
                    .as_ref(),
                &[0xa5; 16]
            );
            assert!(emu.maps.write_qword(data, 0));
            assert!(emu.maps.write_bytes(data + 0xffc, &[0xa5; 4]));
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    library,
                    "FileTimeToSystemTime",
                    &[data, data + 0xffc]
                ),
                0
            );
            assert_eq!(emu.get_last_error(), ERROR_NOACCESS);
            assert_eq!(
                emu.maps
                    .read_bytes_option(data + 0xffc, 4)
                    .unwrap()
                    .as_ref(),
                &[0xa5; 4]
            );
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    library,
                    "FileTimeToSystemTime",
                    &[data + 0xffc, data + 0x100]
                ),
                0
            );
            assert_eq!(emu.get_last_error(), ERROR_NOACCESS);
        }
    }
}

#[test]
fn fat_time_conversion_rounds_up_at_two_second_and_calendar_boundaries() {
    const LEAP_DAY_235900: u64 = 133_537_247_400_000_000;
    const DOS_LEAP_DAY: u16 = 0x585d;
    const DOS_MARCH_1: u16 = 0x5861;
    const DOS_235900: u16 = 0xbf60;
    const DOS_235902: u16 = 0xbf61;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        for library in ["kernel32.dll", "kernelbase.dll"] {
            for (name, value, expected_date, expected_time) in [
                (
                    "exact even second",
                    LEAP_DAY_235900,
                    DOS_LEAP_DAY,
                    DOS_235900,
                ),
                (
                    "one subsecond tick",
                    LEAP_DAY_235900 + 1,
                    DOS_LEAP_DAY,
                    DOS_235902,
                ),
                (
                    "odd second",
                    LEAP_DAY_235900 + 10_000_000,
                    DOS_LEAP_DAY,
                    DOS_235902,
                ),
                (
                    "next even second",
                    LEAP_DAY_235900 + 20_000_000,
                    DOS_LEAP_DAY,
                    DOS_235902,
                ),
                ("next month", LEAP_DAY_235900 + 590_000_000, DOS_MARCH_1, 0),
                ("round into FAT epoch", 119_600_063_999_990_000, 0x0021, 0),
            ] {
                assert!(emu.maps.write_qword(data, value));
                assert!(emu.maps.write_dword(data + 0x100, u32::MAX));
                assert!(emu.maps.write_dword(data + 0x200, u32::MAX));
                emu.set_last_error(ERROR_ACCESS_DENIED);
                assert_eq!(
                    helpers::call_winapi_export(
                        &mut emu,
                        library,
                        "FileTimeToDosDateTime",
                        &[data, data + 0x100, data + 0x200]
                    ),
                    1,
                    "{name}"
                );
                assert_eq!(emu.maps.read_word(data + 0x100), Some(expected_date));
                assert_eq!(emu.maps.read_word(data + 0x200), Some(expected_time));
                assert_eq!(emu.maps.read_word(data + 0x102), Some(u16::MAX));
                assert_eq!(emu.maps.read_word(data + 0x202), Some(u16::MAX));
                assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
                assert_eq!(
                    helpers::call_winapi_export(
                        &mut emu,
                        library,
                        "DosDateTimeToFileTime",
                        &[expected_date as u64, expected_time as u64, data + 0x300]
                    ),
                    1
                );
                let rounded = emu.maps.read_qword(data + 0x300).unwrap();
                assert!(rounded >= value && rounded - value < 20_000_000, "{name}");
            }
            for (date, time) in [(0, 0), (0xffff, 0xffff), (0x0021, 30), (0x0021, 31)] {
                assert!(emu.maps.write_qword(data + 0x300, u64::MAX));
                assert_eq!(
                    helpers::call_winapi_export(
                        &mut emu,
                        library,
                        "DosDateTimeToFileTime",
                        &[date, time, data + 0x300]
                    ),
                    0
                );
                assert_eq!(emu.get_last_error(), ERROR_INVALID_PARAMETER);
                assert_eq!(emu.maps.read_qword(data + 0x300), Some(u64::MAX));
            }
            assert!(emu.maps.write_qword(data, 159_992_927_990_000_000));
            assert!(emu.maps.write_dword(data + 0x100, u32::MAX));
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    library,
                    "FileTimeToDosDateTime",
                    &[data, data + 0x100, data + 0x200]
                ),
                0
            );
            assert_eq!(emu.get_last_error(), ERROR_INVALID_PARAMETER);
            assert_eq!(emu.maps.read_dword(data + 0x100), Some(u32::MAX));
            assert!(emu.maps.write_qword(data, LEAP_DAY_235900));
            assert_eq!(
                helpers::call_winapi_export(
                    &mut emu,
                    library,
                    "FileTimeToDosDateTime",
                    &[data, data + 0x100, data + 0xfff]
                ),
                0
            );
            assert_eq!(emu.get_last_error(), ERROR_NOACCESS);
            assert_eq!(emu.maps.read_dword(data + 0x100), Some(u32::MAX));
        }
    }
}

#[test]
fn guest_clock_agrees_across_structured_and_file_time_queries() {
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        for dll in ["kernel32.dll", "kernelbase.dll"] {
            for (index, api) in [
                "GetSystemTime",
                "GetLocalTime",
                "GetSystemTimeAsFileTime",
                "GetSystemTimePreciseAsFileTime",
            ]
            .into_iter()
            .enumerate()
            {
                emu.tick = 1234;
                helpers::call_winapi_export(&mut emu, dll, api, &[data + 32 * index as u64]);
            }
            assert_eq!(
                emu.maps.read_qword(data + 64),
                emu.maps.read_qword(data + 96)
            );
            for at in [data, data + 32] {
                assert_eq!(emu.maps.read_word(at), Some(2024));
                assert_eq!(emu.maps.read_word(at + 14), Some(234));
            }
            helpers::call_winapi_export(&mut emu, dll, "SystemTimeToFileTime", &[data, data + 128]);
            assert_eq!(
                emu.maps.read_qword(data + 128),
                emu.maps.read_qword(data + 64)
            );
        }
    }
}

#[test]
fn multimedia_exports_use_guest_ticks_and_validate_the_caps_buffer() {
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, data) = helpers::prepare_winapi(is64);
        for dll in ["kernel32.dll", "kernelbase.dll", "winmm.dll"] {
            for (realistic, expected, final_tick) in [
                (false, [12345, 12346], 12347),
                (true, [12343, 12343], 12345),
            ] {
                emu.cfg.realistic_tick = realistic;
                emu.tick = 12345;
                emu.set_last_error(ERROR_ACCESS_DENIED);
                for tick in expected {
                    assert_eq!(call(&mut emu, dll, "timeGetTime", &[]), tick);
                }
                assert_eq!(emu.tick, final_tick);
                assert_eq!(emu.get_last_error(), ERROR_ACCESS_DENIED);
                assert!(emu.api_policy.last_outcome.is_none());
            }
            emu.maps.write_bytes(data, &[0xa5; 9]);
            assert_ne!(call(&mut emu, dll, "timeGetDevCaps", &[data, 7]), 0);
            assert_eq!(emu.maps.read_byte(data), Some(0xa5));
            assert_eq!(call(&mut emu, dll, "timeGetDevCaps", &[data, 8]), 0);
            assert_eq!(emu.maps.read_dword(data), Some(1));
            assert_eq!(emu.maps.read_dword(data + 4), Some(1_000_000));
            assert_eq!(emu.maps.read_byte(data + 8), Some(0xa5));
        }
    }
}

#[test]
fn native_clock_and_version_outputs_match_win32_at_guest_width() {
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, out) = helpers::prepare_winapi(is64);
        let frequency = out + 16;
        emu.tick = 12345;
        emu.maps.write_bytes(out, &[0xa5; 32]);
        assert_eq!(
            call(
                &mut emu,
                "ntdll.dll",
                "NtQueryPerformanceCounter",
                &[out, frequency]
            ),
            STATUS_SUCCESS
        );
        let count = emu.maps.read_qword(out);
        assert_eq!(count, Some(123_450_000));
        assert_eq!(
            emu.maps.read_qword(frequency),
            Some(PERFORMANCE_COUNTER_FREQUENCY)
        );
        assert_eq!(
            call(&mut emu, "kernel32.dll", "QueryPerformanceCounter", &[out]),
            1
        );
        assert_eq!(emu.maps.read_qword(out), count);
        emu.cfg.winver = Some("26100.7920".into());
        for (dll, api, basic, extended) in [
            ("ntdll.dll", "RtlGetVersion", 276u32, 284u32),
            ("kernel32.dll", "GetVersionExA", 148, 156),
            ("kernelbase.dll", "GetVersionExW", 276, 284),
        ] {
            for size in [basic, extended] {
                emu.maps.write_bytes(out, &[0xa5; 300]);
                emu.maps.write_dword(out, size);
                assert_eq!(
                    call(&mut emu, dll, api, &[out]),
                    u64::from(dll != "ntdll.dll")
                );
                assert_eq!(emu.maps.read_dword(out + 12), Some(26100));
                assert_eq!(emu.maps.read_byte(out + u64::from(size)), Some(0xa5));
            }
        }
    }
}

#[test]
fn kernel_interrupt_time_writes_the_guest_clock_in_100ns_units() {
    use helpers::call_winapi_export as call;
    for is64 in [false, true] {
        let (mut emu, out) = helpers::prepare_winapi(is64);
        emu.tick = 12345;
        for api in [
            "QueryInterruptTime",
            "QueryInterruptTimePrecise",
            "QueryUnbiasedInterruptTime",
            "QueryUnbiasedInterruptTimePrecise",
        ] {
            emu.maps.write_bytes(out, &[0xa5; 9]);
            let result = call(&mut emu, "kernel32.dll", api, &[out]);
            if api == "QueryUnbiasedInterruptTime" {
                assert_eq!(result, 1);
            }
            assert_eq!(emu.maps.read_qword(out), Some(123_450_000), "{api}");
            assert_eq!(emu.maps.read_byte(out + 8), Some(0xa5));
        }
    }
}

#[test]
fn packed_version_matches_the_configured_guest_os_on_each_provider() {
    for (wide, library) in iproduct!([false, true], ["kernel32.dll", "kernelbase.dll"]) {
        let (mut emu, data) = helpers::prepare_winapi(wide);
        emu.cfg.winver = Some("26100.1".into());
        let call = helpers::call_winapi_export;
        emu.maps.write_dword(data, 148);
        assert_eq!(call(&mut emu, library, "GetVersionExA", &[data]), 1);
        let version = call(&mut emu, library, "GetVersion", &[]) as u32;
        assert_eq!(
            u32::from(version as u8),
            emu.maps.read_dword(data + 4).unwrap()
        );
        assert_eq!(
            u32::from((version >> 8) as u8),
            emu.maps.read_dword(data + 8).unwrap()
        );
        assert_eq!(version >> 16, emu.maps.read_dword(data + 12).unwrap());
    }
}
