use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::tests::helpers::{call_winapi32, call_winapi64, setup};
use crate::{emu32, emu64};

fn emu64_with_stack() -> Emu {
    setup();
    let mut emu = emu64();
    emu.cfg.console_enabled = false;
    emu.running_script = true;
    emu.init_stack64();
    emu
}

fn emu32_with_stack() -> Emu {
    setup();
    let mut emu = emu32();
    emu.cfg.console_enabled = false;
    emu.running_script = true;
    emu.init_stack32();
    emu
}

fn alloc_rw(emu: &mut Emu, name: &str, size: u64) -> u64 {
    let base = emu.maps.alloc(size).unwrap();
    emu.maps
        .create_map(name, base, size, Permission::READ_WRITE)
        .unwrap();
    base
}

#[test]
fn find_resource_a_returns_null_when_not_found_64() {
    use crate::winapi::winapi64::kernel32::FindResourceA;

    let mut emu = emu64_with_stack();
    let buf = alloc_rw(&mut emu, "data", 0x1000);
    let result = call_winapi64(&mut emu, FindResourceA, &[buf, 1, 3]);
    assert_eq!(
        result, 0,
        "FindResourceA must return NULL when resource is not found"
    );
}

#[test]
fn find_resource_a_returns_null_when_not_found_32() {
    use crate::winapi::winapi32::kernel32::FindResourceA;

    let mut emu = emu32_with_stack();
    let buf = alloc_rw(&mut emu, "data", 0x1000);
    let result = call_winapi32(&mut emu, FindResourceA, &[buf as u32, 1, 3]);
    assert_eq!(
        result, 0,
        "FindResourceA must return NULL when resource is not found"
    );
}

#[test]
fn find_resource_w_returns_null_when_not_found_64() {
    use crate::winapi::winapi64::kernel32::FindResourceW;

    let mut emu = emu64_with_stack();
    let buf = alloc_rw(&mut emu, "data", 0x1000);
    let result = call_winapi64(&mut emu, FindResourceW, &[buf, 1, 3]);
    assert_eq!(
        result, 0,
        "FindResourceW must return NULL when resource is not found"
    );
}

#[test]
fn get_locale_info_w_does_not_corrupt_r9_64() {
    use crate::winapi::winapi64::kernel32::GetLocaleInfoW;

    let mut emu = emu64_with_stack();
    let buf = alloc_rw(&mut emu, "outbuf", 0x1000);
    let sentinel: u64 = 0xDEAD_BEEF_CAFE_BABE;

    // r9 = cchData (a buffer SIZE, NOT a pointer). We set cchData to an
    // address that happens to be mapped and plant a sentinel there. If
    // the handler mistakenly dereferences r9, the sentinel gets clobbered.
    let trap = alloc_rw(&mut emu, "trap", 0x1000);
    emu.maps.write_qword(trap, sentinel);
    let cch_data: u64 = trap; // numeric value = a valid address

    call_winapi64(&mut emu, GetLocaleInfoW, &[0x0409, 0x59, buf, cch_data]);

    let val = emu.maps.read_qword(trap);
    assert_eq!(
        val,
        Some(sentinel),
        "GetLocaleInfoW must not write to the address in r9 (cchData is a size, not a pointer)"
    );
}

#[test]
fn blind_write_to_rdx_must_not_corrupt_memory_64() {
    use crate::winapi::winapi64::kernel32::GetLocaleInfoW;

    let mut emu = emu64_with_stack();
    let buf = alloc_rw(&mut emu, "outbuf", 0x1000);
    let sentinel: u64 = 0xAAAA_BBBB_CCCC_DDDD;

    // For GetLocaleInfoW rdx = LCType (flags). We pass `buf` as LCType so
    // that if the handler blindly writes to rdx it will corrupt the sentinel.
    emu.maps.write_qword(buf, sentinel);
    call_winapi64(&mut emu, GetLocaleInfoW, &[0x0409, buf, buf + 0x100, 256]);

    let val = emu.maps.read_qword(buf);
    assert_eq!(
        val,
        Some(sentinel),
        "API must not blindly write to rdx when it is not an output pointer"
    );
}

#[test]
fn handler_exist_gate_must_not_cause_api_failure_64() {
    use crate::winapi::winapi64::gdi32::RestoreDC;

    let mut emu = emu64_with_stack();
    let fake_hdc: u64 = 0x12340001;
    let saved_dc: u64 = u64::MAX; // -1 = most recent

    let result = call_winapi64(&mut emu, RestoreDC, &[fake_hdc, saved_dc]);
    assert_eq!(
        result, 1,
        "RestoreDC must succeed even for unregistered handles"
    );
}

#[test]
fn handler_exist_gate_must_not_cause_api_failure_32() {
    use crate::winapi::winapi32::gdi32::RestoreDC;

    let mut emu = emu32_with_stack();
    let fake_hdc: u32 = 0x1234_0001;
    let saved_dc: u32 = 0xFFFF_FFFF; // -1

    let result = call_winapi32(&mut emu, RestoreDC, &[fake_hdc, saved_dc]);
    assert_eq!(
        result, 1,
        "RestoreDC must succeed even for unregistered handles"
    );
}
