use crate::emu64;
use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::winapi64;

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
