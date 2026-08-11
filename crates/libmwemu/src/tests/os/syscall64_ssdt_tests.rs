use std::sync::atomic::Ordering;

use crate::maps::mem64::Permission;
use crate::syscall::windows::syscall64;
use crate::tests::helpers;
use crate::windows::constants::*;
use crate::*;

fn setup_emu64_syscall() -> emu::Emu {
    let mut emu = emu64();
    emu.maps
        .create_map("stack", 0x100000, 0x20000, Permission::READ_WRITE)
        .expect("create stack map");
    emu.regs_mut().rsp = 0x101000;
    emu
}

#[test]
fn nt_query_virtual_memory_success_writes_output() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.maps
        .create_map("target", 0x400000, 0x3000, Permission::READ_WRITE_EXECUTE)
        .expect("create target map");
    emu.maps
        .create_map("io", 0x500000, 0x2000, Permission::READ_WRITE)
        .expect("create io map");

    emu.regs_mut().rax = WIN64_NTQUERYVIRTUALMEMORY;
    emu.regs_mut().rcx = !0; // current process
    emu.regs_mut().rdx = 0x400100;
    emu.regs_mut().r8 = MEMORY_INFORMATION_CLASS_MEMORY_BASIC_INFORMATION;
    emu.regs_mut().r9 = 0x500100; // MEMORY_BASIC_INFORMATION output
    emu.maps.write_qword(
        emu.regs().rsp + 0x28,
        crate::windows::structures::MemoryBasicInformation64::SIZE,
    ); // out length
    emu.maps.write_qword(emu.regs().rsp + 0x30, 0x500080); // return length ptr

    syscall64::gateway(&mut emu);

    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_qword(0x500080).unwrap_or(0),
        crate::windows::structures::MemoryBasicInformation64::SIZE
    );
    assert_eq!(emu.maps.read_qword(0x500100).unwrap_or(0), 0x400000);
}

#[test]
fn nt_allocate_virtual_memory_success_maps_region() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.maps
        .create_map("io", 0x520000, 0x2000, Permission::READ_WRITE)
        .expect("create io map");

    emu.maps.write_qword(0x520000, 0); // base address in/out
    emu.maps.write_qword(0x520008, 0x2000); // region size in/out

    emu.regs_mut().rax = WIN64_NTALLOCATEVIRTUALMEMORY;
    emu.regs_mut().rcx = !0;
    emu.regs_mut().rdx = 0x520000; // base ptr
    emu.regs_mut().r8 = 0; // zero bits
    emu.regs_mut().r9 = 0x520008; // region size ptr
    emu.maps
        .write_dword(emu.regs().rsp + 0x28, MEM_COMMIT | MEM_RESERVE);
    emu.maps.write_dword(emu.regs().rsp + 0x30, PAGE_READWRITE);

    syscall64::gateway(&mut emu);

    let base = emu.maps.read_qword(0x520000).unwrap_or(0);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert!(base != 0);
    assert!(emu.maps.is_mapped(base));
}

#[test]
fn nt_write_then_read_virtual_memory_roundtrip() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.maps
        .create_map("src", 0x530000, 0x1000, Permission::READ_WRITE)
        .expect("create src");
    emu.maps
        .create_map("dst", 0x540000, 0x1000, Permission::READ_WRITE)
        .expect("create dst");
    emu.maps
        .create_map("out", 0x550000, 0x1000, Permission::READ_WRITE)
        .expect("create out");

    let payload = b"mwemu-nt-rw";
    assert!(emu.maps.write_bytes(0x530100, payload));

    emu.regs_mut().rax = WIN64_NTWRITEVIRTUALMEMORY;
    emu.regs_mut().rcx = !0;
    emu.regs_mut().rdx = 0x540100; // target base
    emu.regs_mut().r8 = 0x530100; // source buffer
    emu.regs_mut().r9 = u64::try_from(payload.len()).expect("payload length fits u64");
    emu.maps.write_qword(emu.regs().rsp + 0x28, 0x550080); // bytes written ptr
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_qword(0x550080).unwrap_or(0),
        u64::try_from(payload.len()).expect("payload length fits u64")
    );

    emu.regs_mut().rax = WIN64_NTREADVIRTUALMEMORY;
    emu.regs_mut().rcx = !0;
    emu.regs_mut().rdx = 0x540100; // source in target region
    emu.regs_mut().r8 = 0x550100; // destination buffer
    emu.regs_mut().r9 = u64::try_from(payload.len()).expect("payload length fits u64");
    emu.maps.write_qword(emu.regs().rsp + 0x28, 0x550088); // bytes read ptr
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_qword(0x550088).unwrap_or(0),
        u64::try_from(payload.len()).expect("payload length fits u64")
    );
    assert_eq!(emu.maps.read_bytes(0x550100, payload.len()), payload);
}

#[test]
fn nt_query_information_process_basic_and_cookie() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.maps
        .create_map("peb", 0x70000000, 0x1000, Permission::READ_WRITE)
        .expect("create peb");
    emu.maps
        .create_map("io", 0x560000, 0x2000, Permission::READ_WRITE)
        .expect("create io");

    // ProcessBasicInformation
    emu.regs_mut().rax = WIN64_NTQUERYINFORMATIONPROCESS;
    emu.regs_mut().rcx = !0;
    emu.regs_mut().rdx = PROCESS_INFORMATION_CLASS_PROCESS_BASIC_INFORMATION;
    emu.regs_mut().r8 = 0x560100;
    emu.regs_mut().r9 = 0x30;
    emu.maps.write_qword(emu.regs().rsp + 0x28, 0x560080); // return length ptr
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_qword(0x560080).unwrap_or(0), 0x30);
    assert_eq!(emu.maps.read_qword(0x560108).unwrap_or(0), 0x70000000); // PebBaseAddress

    // ProcessCookie
    emu.regs_mut().rax = WIN64_NTQUERYINFORMATIONPROCESS;
    emu.regs_mut().rcx = !0;
    emu.regs_mut().rdx = PROCESS_INFORMATION_CLASS_PROCESS_COOKIE;
    emu.regs_mut().r8 = 0x560200;
    emu.regs_mut().r9 = 4;
    emu.maps.write_qword(emu.regs().rsp + 0x28, 0);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(0x560200).unwrap_or(1), 0x01234567);
}

#[test]
fn nt_open_and_terminate_process_behavior() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.maps
        .create_map("io", 0x570000, 0x1000, Permission::READ_WRITE)
        .expect("create io");

    emu.regs_mut().rax = WIN64_NTOPENPROCESS;
    emu.regs_mut().rcx = 0x570080; // process handle out
    emu.regs_mut().rdx = 0;
    emu.regs_mut().r8 = 0;
    emu.regs_mut().r9 = 0;
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_qword(0x570080).unwrap_or(0), 0x4);

    emu.is_running.store(1, Ordering::Relaxed);
    emu.regs_mut().rax = WIN64_NTTERMINATEPROCESS;
    emu.regs_mut().rcx = !0;
    emu.regs_mut().rdx = 0;
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.is_running.load(Ordering::Relaxed), 0);
}

// ---------------------------------------------------------------------------
// NtQuerySystemInformation
// ---------------------------------------------------------------------------

// x64 native ABI sizes used by these tests. Sourced from PHNT ntexapi.h.
const NATIVE_SYSTEM_BASIC_INFO_SIZE: u32 = 0x40;
const NATIVE_SYSTEM_PROCESSOR_INFO_SIZE: u32 = 0x0C;
const NATIVE_SYSTEM_FILE_CACHE_INFO_SIZE: u32 = 0x3C;
const NATIVE_SYSTEM_MODULE_INFO_REQUIRED: u32 = 0x130;
const NATIVE_SYSTEM_MODULE_INFO_EX_REQUIRED: u32 = 0x140;
const NATIVE_SYSTEM_CODE_INTEGRITY_POLICY_INFO_SIZE: u32 = 0x20;
const NATIVE_SYSTEM_PROCESS_INFO_PREFIX_SIZE: u32 = 0x100;
const NATIVE_SYSTEM_THREAD_INFO_SIZE: u32 = 0x50;
const NATIVE_SYSTEM_TIME_OF_DAY_INFO_SIZE: u32 = 0x30;
const NATIVE_SYSTEM_DEVICE_INFO_SIZE: u32 = 0x18;
const NATIVE_SYSTEM_EXCEPTION_INFO_SIZE: u32 = 0x10;
const NATIVE_SYSTEM_CODE_INTEGRITY_INFO_SIZE: u32 = 0x08;
const NATIVE_SYSTEM_MEMORY_LIST_INFO_SIZE: u32 = 0xB0;
const NATIVE_SYSTEM_ERROR_PORT_TIMEOUTS_SIZE: u32 = 0x08;
const NATIVE_SYSTEM_RECOMMENDED_SHARED_DATA_ALIGNMENT_SIZE: u32 = 0x04;
const NATIVE_SYSTEM_PROCESSOR_PERFORMANCE_INFO_SIZE: u32 = 0x30;
const NATIVE_SYSTEM_EXTENDED_HANDLE_HEADER_SIZE: u32 = 0x10;
const NATIVE_SYSTEM_KERNEL_DEBUGGER_INFO_EX_SIZE: u32 = 0x03;

/// Create a 0x4000-byte read/write map at `addr` (page-aligned) if not
/// already covered. Tests that need a custom map layout (e.g. RO maps,
/// adjacent maps, holes) must NOT call this helper.
fn ensure_qsi_map(emu: &mut emu::Emu, addr: u64) {
    let base = addr & !0xFFF;
    if emu.maps.is_mapped(base) {
        return;
    }
    let name = format!("qsi_io_{:x}", base);
    emu.maps
        .create_map(&name, base, 0x4000, Permission::READ_WRITE)
        .expect("create qsi io map");
}

/// Set the standard x64 NtQuerySystemInformation register layout and ensure
/// the output buffer and the `ReturnLength` pointer both have readable,
/// writable maps at their pages. Tests that exercise holes, read-only maps,
/// or unmapped `ReturnLength` pointers must NOT use this helper.
fn setup_qsi(emu: &mut emu::Emu, class: u64, buf: u64, len: u32, ret_len: u64) {
    ensure_qsi_map(emu, buf);
    if ret_len != 0 {
        ensure_qsi_map(emu, ret_len);
    }
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = class;
    emu.regs_mut().rdx = buf;
    emu.regs_mut().r8 = u64::from(len);
    emu.regs_mut().r9 = ret_len;
}

fn sentinel_byte(index: usize) -> u8 {
    u8::try_from((index ^ 0xA5) & 0xFF).expect("sentinel byte fits in u8")
}

/// Fill a buffer with a sentinel pattern that is unlikely to be produced by
/// the dispatcher, so tests can prove the dispatcher did not write beyond the
/// native structure.
fn fill_sentinel(emu: &mut emu::Emu, addr: u64, len: usize) {
    let pattern: Vec<u8> = (0..len).map(sentinel_byte).collect();
    assert!(
        emu.maps.write_bytes(addr, &pattern),
        "write sentinel pattern"
    );
}

// ---------------------------------------------------------------------------
// Pointer / range / ReturnLength validation
// ---------------------------------------------------------------------------

#[test]
fn qsi_null_buffer_nonzero_length_returns_invalid_parameter() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0x03; // SystemTimeOfDayInformation
    emu.regs_mut().rdx = 0;
    emu.regs_mut().r8 = 0x30;
    emu.regs_mut().r9 = 0;
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INVALID_PARAMETER);
}

#[test]
fn qsi_null_zero_buffer_unknown_class_returns_invalid_info_class() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    // Unknown class with null/zero-length output must not crash; we only
    // require it to return an error status.
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0xFFF;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().r8 = 0;
    emu.regs_mut().r9 = 0;
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INVALID_INFO_CLASS);
}

#[test]
fn qsi_unmapped_buffer_returns_access_violation() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0x03; // SystemTimeOfDayInformation
    emu.regs_mut().rdx = 0x800000; // unmapped
    emu.regs_mut().r8 = 0x30;
    emu.regs_mut().r9 = 0x101100;
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_ACCESS_VIOLATION);
}

#[test]
fn qsi_read_only_buffer_returns_access_violation() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.maps
        .create_map("qsi_ro", 0x800000, 0x4000, Permission::READ_EXECUTE)
        .expect("create read-only map");
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0x03; // SystemTimeOfDayInformation
    emu.regs_mut().rdx = 0x800100;
    emu.regs_mut().r8 = 0x30;
    emu.regs_mut().r9 = 0x101100;
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_ACCESS_VIOLATION);
}

#[test]
fn qsi_range_crossing_two_writable_maps_succeeds() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    // Two adjacent read/write maps; output buffer spans the boundary.
    emu.maps
        .create_map("qsi_a", 0x800000, 0x1000, Permission::READ_WRITE)
        .expect("create map a");
    emu.maps
        .create_map("qsi_b", 0x801000, 0x1000, Permission::READ_WRITE)
        .expect("create map b");
    let ret_len = 0x101100;
    setup_qsi(&mut emu, 0x03, 0x800FF0, 0x30, ret_len);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(ret_len).unwrap_or(0), 0x30);
}

#[test]
fn qsi_range_crossing_into_unmapped_hole_returns_access_violation() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.maps
        .create_map("qsi_a", 0x800000, 0x1000, Permission::READ_WRITE)
        .expect("create map a");
    // No second map: range crosses into unmapped territory.
    setup_qsi(&mut emu, 0x03, 0x800FF0, 0x30, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_ACCESS_VIOLATION);
}

#[test]
fn qsi_range_crossing_into_read_only_map_returns_access_violation() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.maps
        .create_map("qsi_rw", 0x800000, 0x1000, Permission::READ_WRITE)
        .expect("create rw map");
    emu.maps
        .create_map("qsi_ro", 0x801000, 0x1000, Permission::READ_EXECUTE)
        .expect("create ro map");
    setup_qsi(&mut emu, 0x03, 0x800FF0, 0x30, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_ACCESS_VIOLATION);
}

#[test]
fn qsi_range_ending_at_final_mapped_byte_succeeds() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.maps
        .create_map("qsi_edge", 0x800000, 0x1000, Permission::READ_WRITE)
        .expect("create map");
    let map_end = 0x800FFF;
    let ret_len = 0x101100;
    setup_qsi(&mut emu, 0x03, map_end - 0x2F, 0x30, ret_len);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(ret_len).unwrap_or(0), 0x30);
}

#[test]
fn qsi_oversized_buffer_only_requires_native_response_range() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.maps
        .create_map("qsi_native_edge", 0x800000, 0x1000, Permission::READ_WRITE)
        .expect("create output map");
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0x00; // SystemBasicInformation
    emu.regs_mut().rdx = 0x800FC0; // native 0x40-byte response ends at map boundary
    emu.regs_mut().r8 = u64::from(NATIVE_SYSTEM_BASIC_INFO_SIZE + 0x10);
    emu.regs_mut().r9 = 0x101100;
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(0x101100).unwrap_or(0),
        NATIVE_SYSTEM_BASIC_INFO_SIZE
    );
}

#[test]
fn qsi_range_near_u64_max_returns_access_violation() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    // Place a small map very close to u64::MAX so address+len would overflow.
    // No map is created at this address; the validation must still report
    // STATUS_ACCESS_VIOLATION rather than panic on overflow.
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0x03;
    emu.regs_mut().rdx = u64::MAX - 0x10;
    emu.regs_mut().r8 = 0x30;
    emu.regs_mut().r9 = 0;
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_ACCESS_VIOLATION);
}

#[test]
fn qsi_invalid_return_length_pointer_returns_access_violation_on_success_path() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    // Output buffer is fully mapped, but the ReturnLength pointer is
    // deliberately unmapped. We must set registers directly because the
    // `setup_qsi` helper would auto-map the ReturnLength pointer.
    ensure_qsi_map(&mut emu, 0x800100);
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0x03;
    emu.regs_mut().rdx = 0x800100;
    emu.regs_mut().r8 = 0x30;
    emu.regs_mut().r9 = 0x900000; // unmapped
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_ACCESS_VIOLATION);
}

#[test]
fn qsi_read_only_return_length_returns_access_violation() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.maps
        .create_map("qsi_output", 0x800000, 0x1000, Permission::READ_WRITE)
        .expect("create output map");
    emu.maps
        .create_map("qsi_return_ro", 0x900000, 0x1000, Permission::READ_EXECUTE)
        .expect("create read-only return-length map");
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0x03;
    emu.regs_mut().rdx = 0x800100;
    emu.regs_mut().r8 = u64::from(NATIVE_SYSTEM_TIME_OF_DAY_INFO_SIZE);
    emu.regs_mut().r9 = 0x900100;
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_ACCESS_VIOLATION);
}

#[test]
fn qsi_return_length_crossing_adjacent_writable_maps_succeeds() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.maps
        .create_map("qsi_return_a", 0x800000, 0x1000, Permission::READ_WRITE)
        .expect("create first return-length map");
    emu.maps
        .create_map("qsi_return_b", 0x801000, 0x1000, Permission::READ_WRITE)
        .expect("create second return-length map");
    emu.maps
        .create_map("qsi_output", 0x900000, 0x1000, Permission::READ_WRITE)
        .expect("create output map");
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0x03;
    emu.regs_mut().rdx = 0x900100;
    emu.regs_mut().r8 = u64::from(NATIVE_SYSTEM_TIME_OF_DAY_INFO_SIZE);
    emu.regs_mut().r9 = 0x800FFE; // four-byte ULONG crosses the map boundary
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_byte(0x800FFE).unwrap_or(0), 0x30);
    assert_eq!(emu.maps.read_byte(0x800FFF).unwrap_or(0), 0x00);
    assert_eq!(emu.maps.read_byte(0x801000).unwrap_or(0), 0x00);
    assert_eq!(emu.maps.read_byte(0x801001).unwrap_or(0), 0x00);
}

#[test]
fn qsi_return_length_crossing_unmapped_hole_returns_access_violation() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.maps
        .create_map("qsi_return", 0x800000, 0x1000, Permission::READ_WRITE)
        .expect("create return-length map");
    emu.maps
        .create_map("qsi_output", 0x900000, 0x1000, Permission::READ_WRITE)
        .expect("create output map");
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0x03;
    emu.regs_mut().rdx = 0x900100;
    emu.regs_mut().r8 = u64::from(NATIVE_SYSTEM_TIME_OF_DAY_INFO_SIZE);
    emu.regs_mut().r9 = 0x800FFE; // final two ULONG bytes are unmapped
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_ACCESS_VIOLATION);
}

#[test]
fn qsi_overflowing_return_length_range_returns_access_violation() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    ensure_qsi_map(&mut emu, 0x800100);
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0x03;
    emu.regs_mut().rdx = 0x800100;
    emu.regs_mut().r8 = u64::from(NATIVE_SYSTEM_TIME_OF_DAY_INFO_SIZE);
    emu.regs_mut().r9 = u64::MAX - 1;
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_ACCESS_VIOLATION);
}

#[test]
fn qsi_short_buffer_invalid_return_length_returns_access_violation() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    ensure_qsi_map(&mut emu, 0x800100);
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0x03; // SystemTimeOfDayInformation
    emu.regs_mut().rdx = 0x800100;
    emu.regs_mut().r8 = u64::from(NATIVE_SYSTEM_TIME_OF_DAY_INFO_SIZE - 1);
    emu.regs_mut().r9 = 0x900000; // non-null and unmapped
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_ACCESS_VIOLATION);
}

#[test]
fn qsi_unsupported_invalid_return_length_returns_access_violation() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    ensure_qsi_map(&mut emu, 0x800100);
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0x5D; // SystemTimeZoneInformation
    emu.regs_mut().rdx = 0x800100;
    emu.regs_mut().r8 = 0x100;
    emu.regs_mut().r9 = 0x900000; // non-null and unmapped
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_ACCESS_VIOLATION);
}

#[test]
fn qsi_unknown_invalid_return_length_returns_access_violation() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    ensure_qsi_map(&mut emu, 0x800100);
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0xFFF;
    emu.regs_mut().rdx = 0x800100;
    emu.regs_mut().r8 = 0x40;
    emu.regs_mut().r9 = 0x900000; // non-null and unmapped
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_ACCESS_VIOLATION);
}

#[test]
fn qsi_performance_invalid_return_length_returns_access_violation() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    ensure_qsi_map(&mut emu, 0x800100);
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0x02; // SystemPerformanceInformation
    emu.regs_mut().rdx = 0x800100;
    emu.regs_mut().r8 = 0x138;
    emu.regs_mut().r9 = 0x900000; // non-null and unmapped
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_ACCESS_VIOLATION);
}

#[test]
fn qsi_null_zero_buffer_unsupported_class_returns_not_supported() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0x5D; // SystemTimeZoneInformation
    emu.regs_mut().rdx = 0;
    emu.regs_mut().r8 = 0;
    emu.regs_mut().r9 = 0;
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_NOT_SUPPORTED);
}

#[test]
fn qsi_unmapped_buffer_unsupported_class_returns_not_supported() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0x5D; // SystemTimeZoneInformation
    emu.regs_mut().rdx = 0x900000;
    emu.regs_mut().r8 = 0x100;
    emu.regs_mut().r9 = 0x101100; // valid ReturnLength on the stack map
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_NOT_SUPPORTED);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(1), 0);
}

#[test]
fn qsi_read_only_buffer_unsupported_class_returns_not_supported() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.maps
        .create_map(
            "qsi_unsupported_ro",
            0x800000,
            0x1000,
            Permission::READ_EXECUTE,
        )
        .expect("create read-only map");
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0x5D; // SystemTimeZoneInformation
    emu.regs_mut().rdx = 0x800100;
    emu.regs_mut().r8 = 0x100;
    emu.regs_mut().r9 = 0x101100;
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_NOT_SUPPORTED);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(1), 0);
}

#[test]
fn qsi_unmapped_buffer_unknown_class_returns_invalid_info_class() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0xFFF;
    emu.regs_mut().rdx = 0x900000;
    emu.regs_mut().r8 = 0x40;
    emu.regs_mut().r9 = 0x101100;
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INVALID_INFO_CLASS);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(1), 0);
}

#[test]
fn qsi_read_only_buffer_unknown_class_returns_invalid_info_class() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.maps
        .create_map("qsi_unknown_ro", 0x800000, 0x1000, Permission::READ_EXECUTE)
        .expect("create read-only map");
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = 0xFFF;
    emu.regs_mut().rdx = 0x800100;
    emu.regs_mut().r8 = 0x40;
    emu.regs_mut().r9 = 0x101100;
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INVALID_INFO_CLASS);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(1), 0);
}

#[test]
fn qsi_basic_information_one_byte_short_reports_native_size() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(
        &mut emu,
        0x00, // SystemBasicInformation
        0x800100,
        NATIVE_SYSTEM_BASIC_INFO_SIZE - 1,
        0x101100,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INFO_LENGTH_MISMATCH);
    assert_eq!(
        emu.maps.read_dword(0x101100).unwrap_or(0),
        NATIVE_SYSTEM_BASIC_INFO_SIZE
    );
}

#[test]
fn qsi_basic_information_exact_size_writes_native_x64_fields() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(
        &mut emu,
        0x00, // SystemBasicInformation
        0x800100,
        NATIVE_SYSTEM_BASIC_INFO_SIZE,
        ret_len,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    // ABI offsets from the x64 SYSTEM_BASIC_INFORMATION definition.
    assert_eq!(emu.maps.read_dword(ret_len).unwrap_or(0), 0x40);
    assert_eq!(emu.maps.read_dword(0x800108).unwrap_or(0), 0x1000); // PageSize
    assert_eq!(emu.maps.read_dword(0x80010C).unwrap_or(0), 0x0010_0000); // NumberOfPhysicalPages
    assert_eq!(emu.maps.read_dword(0x800118).unwrap_or(0), 0x0001_0000); // AllocationGranularity
    assert_eq!(
        emu.maps.read_qword(0x800120).unwrap_or(0),
        0x0000_0000_0001_0000
    ); // MinimumUserModeAddress
    assert_eq!(
        emu.maps.read_qword(0x800128).unwrap_or(0),
        0x0000_7fff_fffe_ffff
    ); // MaximumUserModeAddress
    assert_eq!(emu.maps.read_qword(0x800130).unwrap_or(0), 1); // ActiveProcessorsAffinityMask
    assert_eq!(emu.maps.read_byte(0x800138).unwrap_or(0), 1); // NumberOfProcessors
}

#[test]
fn qsi_basic_information_oversized_buffer_preserves_trailing_sentinel() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let total = NATIVE_SYSTEM_BASIC_INFO_SIZE + 0x10;
    setup_qsi(&mut emu, 0x00, 0x800100, total, 0x101100);
    fill_sentinel(
        &mut emu,
        0x800100,
        usize::try_from(total).expect("test length"),
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(0), 0x40);
    for offset in NATIVE_SYSTEM_BASIC_INFO_SIZE..total {
        let byte = emu
            .maps
            .read_byte(0x800100 + u64::from(offset))
            .unwrap_or(0);
        let expected = sentinel_byte(usize::try_from(offset).expect("test offset fits usize"));
        assert_eq!(
            byte, expected,
            "byte at +0x{:x} should remain sentinel",
            offset
        );
    }
}

#[test]
fn qsi_modeled_oversized_buffers_preserve_trailing_sentinels() {
    // PHNT/ReactOS x64 native response sizes, kept as independent ABI values
    // rather than aliases to the dispatcher's implementation constants.
    let cases: &[(u64, u32)] = &[
        (0x00, 0x40),  // SYSTEM_BASIC_INFORMATION
        (0x3E, 0x40),  // SYSTEM_EMULATION_BASIC_INFORMATION
        (0x01, 0x0C),  // SYSTEM_PROCESSOR_INFORMATION
        (0x03, 0x30),  // SYSTEM_TIMEOFDAY_INFORMATION
        (0x05, 0x150), // SYSTEM_PROCESS_INFORMATION with one thread
        (0x08, 0x30),  // SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION
        (0x07, 0x18),  // SYSTEM_DEVICE_INFORMATION
        (0x21, 0x10),  // SYSTEM_EXCEPTION_INFORMATION
        (0x15, 0x3C),  // SYSTEM_FILECACHE_INFORMATION
        (0x51, 0x3C),  // SYSTEM_FILECACHE_INFORMATION_EX
        (0x50, 0xB0),  // SYSTEM_MEMORY_LIST_INFORMATION
        (0x0B, 0x130), // SYSTEM_MODULE_INFORMATION
        (0x4D, 0x140), // RTL_PROCESS_MODULE_INFORMATION_EX
        (0x40, 0x10),  // SYSTEM_HANDLE_INFORMATION_EX header
        (0x73, 0x08),  // SYSTEM_ERROR_PORT_TIMEOUTS
        (0x3A, 0x04),  // SYSTEM_RECOMMENDED_SHARED_DATA_ALIGNMENT
        (0x23, 0x02),  // SYSTEM_KERNEL_DEBUGGER_INFORMATION
        (0x67, 0x08),  // SYSTEM_CODEINTEGRITY_INFORMATION
        (0xA4, 0x20),  // SYSTEM_CODEINTEGRITYPOLICY_INFORMATION
        (0x95, 0x03),  // SYSTEM_KERNEL_DEBUGGER_INFORMATION_EX
    ];

    for &(class, native_size) in cases {
        helpers::setup();
        let mut emu = setup_emu64_syscall();
        let total = native_size.checked_add(8).expect("test size overflow");
        setup_qsi(&mut emu, class, 0x800100, total, 0x101100);
        fill_sentinel(
            &mut emu,
            0x800100,
            usize::try_from(total).expect("test size fits usize"),
        );
        syscall64::gateway(&mut emu);
        assert_eq!(emu.regs().rax, STATUS_SUCCESS, "class 0x{:x}", class);
        assert_eq!(
            emu.maps.read_dword(0x101100).unwrap_or(0),
            native_size,
            "ReturnLength for class 0x{:x}",
            class
        );
        for offset in native_size..total {
            let byte = emu
                .maps
                .read_byte(0x800100 + u64::from(offset))
                .unwrap_or(0);
            let expected = sentinel_byte(usize::try_from(offset).expect("test offset fits usize"));
            assert_eq!(
                byte, expected,
                "class 0x{:x} wrote past native size at +0x{:x}",
                class, offset
            );
        }
    }
}

#[test]
fn qsi_unknown_class_returns_invalid_info_class() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(&mut emu, 0xFFF, 0x800100, 0x40, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INVALID_INFO_CLASS);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(1), 0);
}

// ---------------------------------------------------------------------------
// Class IDs (corrected per PHNT ntexapi.h)
// ---------------------------------------------------------------------------

#[test]
fn qsi_class_0xa4_code_integrity_policy_dispatch() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(
        &mut emu,
        0xA4, // SystemCodeIntegrityPolicyInformation
        0x800100,
        NATIVE_SYSTEM_CODE_INTEGRITY_POLICY_INFO_SIZE,
        ret_len,
    );
    fill_sentinel(
        &mut emu,
        0x800100,
        usize::try_from(NATIVE_SYSTEM_CODE_INTEGRITY_POLICY_INFO_SIZE)
            .expect("test size fits usize"),
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_CODE_INTEGRITY_POLICY_INFO_SIZE
    );
    // Options (+0x00) and HVCIOptions (+0x04) populated zero.
    assert_eq!(emu.maps.read_dword(0x800100).unwrap_or(1), 0);
    assert_eq!(emu.maps.read_dword(0x800104).unwrap_or(1), 0);
}

#[test]
fn qsi_class_0x5d_time_zone_returns_not_supported() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(&mut emu, 0x5D, 0x800100, 0x100, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_NOT_SUPPORTED);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(1), 0);
}

#[test]
fn qsi_class_0x2c_current_time_zone_is_not_the_0x2b_legacy_driver_class() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    // PHNT: 0x2B is SystemLegacyDriverInformation; 0x2C is
    // SystemCurrentTimeZoneInformation and remains recognized but unsupported.
    setup_qsi(&mut emu, 0x2C, 0x800100, 0x100, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_NOT_SUPPORTED);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(1), 0);
}

#[test]
fn qsi_class_0xb5_supported_processor_architectures_returns_not_supported() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(&mut emu, 0xB5, 0x800100, 0x04, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_NOT_SUPPORTED);
}

#[test]
fn qsi_old_class_0xb4_does_not_alias_supported_processor_architectures() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    // PHNT names 0xB4 SystemInterruptSteering; 0xB5 is the supported-
    // processor-architectures class. The old 0xB4 alias must not be modeled.
    setup_qsi(&mut emu, 0xB4, 0x800100, 0x04, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INVALID_INFO_CLASS);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(1), 0);
}

#[test]
fn qsi_old_class_0x5c_does_not_alias_time_zone_information() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    // PHNT names 0x5C SystemVerifierInformationEx; 0x5D is SystemTimeZoneInformation.
    setup_qsi(&mut emu, 0x5C, 0x800100, 0x100, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INVALID_INFO_CLASS);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(1), 0);
}

#[test]
fn qsi_old_class_0xdd_is_not_silently_modeled() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    // PHNT names 0xDD SystemShadowStackInformation; it is not a modeled class.
    setup_qsi(&mut emu, 0xDD, 0x800100, 0x100, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INVALID_INFO_CLASS);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(1), 0);
}

#[test]
fn qsi_class_0xb6_memory_usage_returns_not_supported() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(&mut emu, 0xB6, 0x800100, 0x10, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_NOT_SUPPORTED);
}

#[test]
fn qsi_class_0xc0_flush_information_returns_not_supported() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(&mut emu, 0xC0, 0x800100, 0x04, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_NOT_SUPPORTED);
}

#[test]
fn qsi_class_0xc5_hypervisor_shared_page_rejects_short_buffer() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    // 4 bytes is short of the pointer-sized payload.
    setup_qsi(&mut emu, 0xC5, 0x800100, 0x04, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INFO_LENGTH_MISMATCH);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(0), 0x08);
}

/// `LdrInitializeThunk` probes this during init and treats any failure status as
/// fatal, so it must succeed rather than report STATUS_NOT_SUPPORTED.
#[test]
fn qsi_class_0xc5_hypervisor_shared_page_succeeds() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(&mut emu, 0xC5, 0x800100, 0x08, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_qword(0x800100).unwrap_or(1), 0);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(0), 0x08);
}

/// Same fatal-on-failure path as 0xC5: NUMA reports a single node.
#[test]
fn qsi_class_0x37_numa_processor_map_succeeds() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(&mut emu, 0x37, 0x800100, 0x408, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    // HighestNodeNumber == 0 -> one NUMA node.
    assert_eq!(emu.maps.read_dword(0x800100).unwrap_or(1), 0);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(0), 0x408);
}

#[test]
fn qsi_class_0x73_error_port_timeouts_single_definition() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    // Class 0x73 is SystemErrorPortTimeouts; the dispatcher must succeed with
    // an 8-byte buffer and return STATUS_INFO_LENGTH_MISMATCH for a 4-byte one.
    let ret_len = 0x101100;
    setup_qsi(&mut emu, 0x73, 0x800100, 4, ret_len);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INFO_LENGTH_MISMATCH);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_ERROR_PORT_TIMEOUTS_SIZE
    );

    setup_qsi(
        &mut emu,
        0x73,
        0x800100,
        NATIVE_SYSTEM_ERROR_PORT_TIMEOUTS_SIZE,
        ret_len,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_ERROR_PORT_TIMEOUTS_SIZE
    );
}

#[test]
fn qsi_class_0x02_performance_information_returns_not_supported() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    // SystemPerformanceInformation is intentionally unsupported: a valid
    // ReturnLength receives zero and the output buffer is never written.
    setup_qsi(&mut emu, 0x02, 0x800100, 0x138, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_NOT_SUPPORTED);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(1), 0);
}

#[test]
fn qsi_class_0x02_performance_information_does_not_overwrite_buffer() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(&mut emu, 0x02, 0x800100, 0x138, 0x101100);
    fill_sentinel(&mut emu, 0x800100, 0x138);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_NOT_SUPPORTED);
    // Sentinel bytes preserved — we did not write a single byte.
    let expected: Vec<u8> = (0..0x138).map(sentinel_byte).collect();
    assert_eq!(emu.maps.read_bytes(0x800100, 0x138), expected);
}

// ---------------------------------------------------------------------------
// Exact-length and one-byte-short buffers
// ---------------------------------------------------------------------------

#[test]
fn qsi_processor_information_short_buffer_reports_required_length() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(
        &mut emu,
        0x01,
        0x800100,
        NATIVE_SYSTEM_PROCESSOR_INFO_SIZE - 1,
        0x101100,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INFO_LENGTH_MISMATCH);
    assert_eq!(
        emu.maps.read_dword(0x101100).unwrap_or(0),
        NATIVE_SYSTEM_PROCESSOR_INFO_SIZE
    );
}

#[test]
fn qsi_processor_information_writes_amd64_fields() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(
        &mut emu,
        0x01,
        0x800100,
        NATIVE_SYSTEM_PROCESSOR_INFO_SIZE,
        ret_len,
    );
    fill_sentinel(
        &mut emu,
        0x800100,
        usize::try_from(NATIVE_SYSTEM_PROCESSOR_INFO_SIZE).expect("test size fits usize"),
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_PROCESSOR_INFO_SIZE
    );
    // +0x00 ProcessorArchitecture = PROCESSOR_ARCHITECTURE_AMD64 (0x0009)
    assert_eq!(emu.maps.read_word(0x800100).unwrap_or(0), 0x0009);
    // +0x06 MaximumProcessors = 1
    assert_eq!(emu.maps.read_word(0x800106).unwrap_or(0), 1);
    // +0x08 ProcessorFeatureBits = 0
    assert_eq!(emu.maps.read_dword(0x800108).unwrap_or(1), 0);
}

#[test]
fn qsi_processor_information_extra_buffer_bytes_unchanged() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    let total_len = NATIVE_SYSTEM_PROCESSOR_INFO_SIZE + 0x10;
    setup_qsi(&mut emu, 0x01, 0x800100, total_len, ret_len);
    fill_sentinel(
        &mut emu,
        0x800100,
        usize::try_from(total_len).expect("test size fits usize"),
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_PROCESSOR_INFO_SIZE
    );
    // Bytes past the native structure keep the sentinel.
    for off in NATIVE_SYSTEM_PROCESSOR_INFO_SIZE..total_len {
        let b = emu.maps.read_byte(0x800100 + u64::from(off)).unwrap_or(0);
        let expected = sentinel_byte(usize::try_from(off).expect("test offset fits usize"));
        assert_eq!(b, expected, "byte at +0x{:x} should remain sentinel", off);
    }
}

#[test]
fn qsi_timeofday_short_buffer_reports_required_length() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(
        &mut emu,
        0x03,
        0x800100,
        NATIVE_SYSTEM_TIME_OF_DAY_INFO_SIZE - 1,
        0x101100,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INFO_LENGTH_MISMATCH);
    assert_eq!(
        emu.maps.read_dword(0x101100).unwrap_or(0),
        NATIVE_SYSTEM_TIME_OF_DAY_INFO_SIZE
    );
}

#[test]
fn qsi_timeofday_writes_native_size_and_timezone_id() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(
        &mut emu,
        0x03,
        0x800100,
        NATIVE_SYSTEM_TIME_OF_DAY_INFO_SIZE,
        ret_len,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_TIME_OF_DAY_INFO_SIZE
    );
    assert_eq!(emu.maps.read_qword(0x800108).unwrap_or(0), 1);
    assert_eq!(emu.maps.read_dword(0x800118).unwrap_or(0), 0x2);
}

#[test]
fn qsi_file_cache_information_short_buffer_reports_required_length() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(
        &mut emu,
        0x15,
        0x800100,
        NATIVE_SYSTEM_FILE_CACHE_INFO_SIZE - 1,
        0x101100,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INFO_LENGTH_MISMATCH);
    assert_eq!(
        emu.maps.read_dword(0x101100).unwrap_or(0),
        NATIVE_SYSTEM_FILE_CACHE_INFO_SIZE
    );
}

#[test]
fn qsi_file_cache_information_writes_native_size() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(
        &mut emu,
        0x15,
        0x800100,
        NATIVE_SYSTEM_FILE_CACHE_INFO_SIZE,
        ret_len,
    );
    fill_sentinel(
        &mut emu,
        0x800100,
        usize::try_from(NATIVE_SYSTEM_FILE_CACHE_INFO_SIZE).expect("test size fits usize"),
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_FILE_CACHE_INFO_SIZE
    );
    // First 4 bytes zeroed.
    assert_eq!(emu.maps.read_dword(0x800100).unwrap_or(0xDEAD), 0);
}

#[test]
fn qsi_file_cache_information_extra_bytes_unchanged() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    let total = NATIVE_SYSTEM_FILE_CACHE_INFO_SIZE + 0x08;
    setup_qsi(&mut emu, 0x15, 0x800100, total, ret_len);
    fill_sentinel(
        &mut emu,
        0x800100,
        usize::try_from(total).expect("test size fits usize"),
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_FILE_CACHE_INFO_SIZE
    );
    let sentinel_value = sentinel_byte(
        usize::try_from(NATIVE_SYSTEM_FILE_CACHE_INFO_SIZE).expect("test size fits usize"),
    );
    assert_eq!(
        emu.maps
            .read_byte(0x800100 + u64::from(NATIVE_SYSTEM_FILE_CACHE_INFO_SIZE))
            .unwrap_or(0),
        sentinel_value
    );
}

#[test]
fn qsi_file_cache_information_ex_uses_same_size() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(
        &mut emu,
        0x51, // SystemFileCacheInformationEx
        0x800100,
        NATIVE_SYSTEM_FILE_CACHE_INFO_SIZE,
        ret_len,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_FILE_CACHE_INFO_SIZE
    );
}

#[test]
fn qsi_device_information_writes_native_size() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(
        &mut emu,
        0x07,
        0x800100,
        NATIVE_SYSTEM_DEVICE_INFO_SIZE,
        ret_len,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_DEVICE_INFO_SIZE
    );
    assert_eq!(emu.maps.read_dword(0x800100).unwrap_or(0), 1);
}

#[test]
fn qsi_exception_information_short_buffer_reports_required_length() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(
        &mut emu,
        0x21,
        0x800100,
        NATIVE_SYSTEM_EXCEPTION_INFO_SIZE - 1,
        0x101100,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INFO_LENGTH_MISMATCH);
    assert_eq!(
        emu.maps.read_dword(0x101100).unwrap_or(0),
        NATIVE_SYSTEM_EXCEPTION_INFO_SIZE
    );
}

#[test]
fn qsi_memory_list_information_writes_native_size() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(
        &mut emu,
        0x50,
        0x800100,
        NATIVE_SYSTEM_MEMORY_LIST_INFO_SIZE,
        ret_len,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_MEMORY_LIST_INFO_SIZE
    );
}

#[test]
fn qsi_recommended_shared_data_alignment_returns_64() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(
        &mut emu,
        0x3A,
        0x800100,
        NATIVE_SYSTEM_RECOMMENDED_SHARED_DATA_ALIGNMENT_SIZE,
        ret_len,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_RECOMMENDED_SHARED_DATA_ALIGNMENT_SIZE
    );
    assert_eq!(emu.maps.read_dword(0x800100).unwrap_or(0), 64);
}

#[test]
fn qsi_error_port_timeouts_short_buffer_reports_required_length() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(
        &mut emu,
        0x73,
        0x800100,
        NATIVE_SYSTEM_ERROR_PORT_TIMEOUTS_SIZE - 1,
        0x101100,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INFO_LENGTH_MISMATCH);
    assert_eq!(
        emu.maps.read_dword(0x101100).unwrap_or(0),
        NATIVE_SYSTEM_ERROR_PORT_TIMEOUTS_SIZE
    );
}

#[test]
fn qsi_code_integrity_information_reports_enabled() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(
        &mut emu,
        0x67,
        0x800100,
        NATIVE_SYSTEM_CODE_INTEGRITY_INFO_SIZE,
        ret_len,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_CODE_INTEGRITY_INFO_SIZE
    );
    assert_eq!(
        emu.maps.read_dword(0x800100).unwrap_or(0),
        NATIVE_SYSTEM_CODE_INTEGRITY_INFO_SIZE
    );
    assert_eq!(emu.maps.read_dword(0x800104).unwrap_or(0), 0x1);
}

#[test]
fn qsi_code_integrity_policy_short_buffer_reports_required_length() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(
        &mut emu,
        0xA4,
        0x800100,
        NATIVE_SYSTEM_CODE_INTEGRITY_POLICY_INFO_SIZE - 1,
        0x101100,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INFO_LENGTH_MISMATCH);
    assert_eq!(
        emu.maps.read_dword(0x101100).unwrap_or(0),
        NATIVE_SYSTEM_CODE_INTEGRITY_POLICY_INFO_SIZE
    );
}

#[test]
fn qsi_code_integrity_policy_writes_native_size_and_zero_fields() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(
        &mut emu,
        0xA4,
        0x800100,
        NATIVE_SYSTEM_CODE_INTEGRITY_POLICY_INFO_SIZE,
        ret_len,
    );
    fill_sentinel(
        &mut emu,
        0x800100,
        usize::try_from(NATIVE_SYSTEM_CODE_INTEGRITY_POLICY_INFO_SIZE)
            .expect("test size fits usize"),
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_CODE_INTEGRITY_POLICY_INFO_SIZE
    );
    // The native structure is filled (Options, HVCIOptions, Version, PolicyGuid = 0).
    assert_eq!(emu.maps.read_dword(0x800100).unwrap_or(1), 0);
    assert_eq!(emu.maps.read_dword(0x800104).unwrap_or(1), 0);
    assert_eq!(emu.maps.read_qword(0x800108).unwrap_or(1), 0);
    assert_eq!(emu.maps.read_qword(0x800110).unwrap_or(1), 0);
    assert_eq!(emu.maps.read_qword(0x800118).unwrap_or(1), 0);
}

#[test]
fn qsi_code_integrity_policy_extra_bytes_unchanged() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    let total = NATIVE_SYSTEM_CODE_INTEGRITY_POLICY_INFO_SIZE + 0x10;
    setup_qsi(&mut emu, 0xA4, 0x800100, total, ret_len);
    fill_sentinel(
        &mut emu,
        0x800100,
        usize::try_from(total).expect("test size fits usize"),
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_CODE_INTEGRITY_POLICY_INFO_SIZE
    );
    for off in NATIVE_SYSTEM_CODE_INTEGRITY_POLICY_INFO_SIZE..total {
        let b = emu.maps.read_byte(0x800100 + u64::from(off)).unwrap_or(0);
        let expected = sentinel_byte(usize::try_from(off).expect("test offset fits usize"));
        assert_eq!(b, expected, "byte at +0x{:x} should remain sentinel", off);
    }
}

#[test]
fn qsi_extended_handle_information_writes_header_size() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(
        &mut emu,
        0x40,
        0x800100,
        NATIVE_SYSTEM_EXTENDED_HANDLE_HEADER_SIZE,
        ret_len,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_EXTENDED_HANDLE_HEADER_SIZE
    );
}

#[test]
fn qsi_kernel_debugger_information_ex_writes_3_bytes() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(
        &mut emu,
        0x95,
        0x800100,
        NATIVE_SYSTEM_KERNEL_DEBUGGER_INFO_EX_SIZE,
        ret_len,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_KERNEL_DEBUGGER_INFO_EX_SIZE
    );
}

#[test]
fn qsi_processor_performance_information_writes_tick() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.pos = 12345;
    let ret_len = 0x101100;
    setup_qsi(
        &mut emu,
        0x08,
        0x800100,
        NATIVE_SYSTEM_PROCESSOR_PERFORMANCE_INFO_SIZE,
        ret_len,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_PROCESSOR_PERFORMANCE_INFO_SIZE
    );
    // IdleTime == emu.pos at +0x00.
    assert_eq!(emu.maps.read_qword(0x800100).unwrap_or(0), 12345);
    // KernelTime == emu.pos at +0x08.
    assert_eq!(emu.maps.read_qword(0x800108).unwrap_or(0), 12345);
    // UserTime at +0x10 stays zero.
    assert_eq!(emu.maps.read_qword(0x800110).unwrap_or(1), 0);
}

// ---------------------------------------------------------------------------
// ModuleInformation / ModuleInformationEx
// ---------------------------------------------------------------------------

#[test]
fn qsi_module_information_short_buffer_reports_required_length() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(
        &mut emu,
        0x0B,
        0x800100,
        NATIVE_SYSTEM_MODULE_INFO_REQUIRED - 1,
        0x101100,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INFO_LENGTH_MISMATCH);
    assert_eq!(
        emu.maps.read_dword(0x101100).unwrap_or(0),
        NATIVE_SYSTEM_MODULE_INFO_REQUIRED
    );
}

#[test]
fn qsi_module_information_writes_ntoskrnl_payload() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(
        &mut emu,
        0x0B,
        0x800100,
        NATIVE_SYSTEM_MODULE_INFO_REQUIRED,
        ret_len,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_MODULE_INFO_REQUIRED
    );
    // NumberOfModules = 1 at +0x00.
    assert_eq!(emu.maps.read_dword(0x800100).unwrap_or(0), 1);
    // Module array starts at +0x08; PHNT x64 RTL_PROCESS_MODULE_INFORMATION:
    //   +0x00  Section         HANDLE   (info+0x08)
    //   +0x08  MappedBase      PVOID    (info+0x10)
    //   +0x10  ImageBase       PVOID    (info+0x18)
    //   +0x18  ImageSize       ULONG    (info+0x20)
    //   +0x1C  Flags           ULONG    (info+0x24)
    //   +0x20  LoadCount       USHORT   (info+0x28)
    //   +0x22  OffsetToFileName USHORT  (info+0x2A)
    //   +0x24  FullPathName[256]        (info+0x2C)
    assert_eq!(
        emu.maps.read_qword(0x800110).unwrap_or(0),
        0xFFFF_F800_0000_0000
    );
    assert_eq!(
        emu.maps.read_qword(0x800118).unwrap_or(0),
        0xFFFF_F800_0000_0000
    );
    assert_eq!(emu.maps.read_dword(0x800120).unwrap_or(0), 0x00A0_0000);
    assert_eq!(emu.maps.read_word(0x800128).unwrap_or(0), 1);
    let name_off = emu.maps.read_word(0x80012A).unwrap_or(0);
    assert!(name_off > 0);
    let path_bytes = emu.maps.read_bytes(0x80012C + u64::from(name_off), 13);
    assert_eq!(path_bytes, b"ntoskrnl.exe\0");
}

#[test]
fn qsi_module_information_ex_short_buffer_reports_required_length() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(
        &mut emu,
        0x4D,
        0x800100,
        NATIVE_SYSTEM_MODULE_INFO_EX_REQUIRED - 1,
        0x101100,
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INFO_LENGTH_MISMATCH);
    assert_eq!(
        emu.maps.read_dword(0x101100).unwrap_or(0),
        NATIVE_SYSTEM_MODULE_INFO_EX_REQUIRED
    );
}

#[test]
fn qsi_module_information_ex_writes_native_layout() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(
        &mut emu,
        0x4D,
        0x800100,
        NATIVE_SYSTEM_MODULE_INFO_EX_REQUIRED,
        ret_len,
    );
    fill_sentinel(
        &mut emu,
        0x800100,
        usize::try_from(NATIVE_SYSTEM_MODULE_INFO_EX_REQUIRED).expect("test size fits usize"),
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_MODULE_INFO_EX_REQUIRED
    );
    // NextOffset at +0x00 is zero (terminator).
    assert_eq!(emu.maps.read_word(0x800100).unwrap_or(0xFFFF), 0);
    // BaseInfo.ImageBase at +0x10+0x08 == +0x18.
    assert_eq!(
        emu.maps.read_qword(0x800118).unwrap_or(0),
        0xFFFF_F800_0000_0000
    );
    // ImageChecksum at +0x130.
    assert_eq!(emu.maps.read_dword(0x800230).unwrap_or(1), 0);
    // TimeDateStamp at +0x134.
    assert_eq!(emu.maps.read_dword(0x800234).unwrap_or(1), 0);
    // DefaultBase at +0x138.
    assert_eq!(
        emu.maps.read_qword(0x800238).unwrap_or(0),
        0xFFFF_F800_0000_0000
    );
}

#[test]
fn qsi_module_information_ex_extra_bytes_unchanged() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    let total = NATIVE_SYSTEM_MODULE_INFO_EX_REQUIRED + 0x10;
    setup_qsi(&mut emu, 0x4D, 0x800100, total, ret_len);
    fill_sentinel(
        &mut emu,
        0x800100,
        usize::try_from(total).expect("test size fits usize"),
    );
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_dword(ret_len).unwrap_or(0),
        NATIVE_SYSTEM_MODULE_INFO_EX_REQUIRED
    );
    for off in NATIVE_SYSTEM_MODULE_INFO_EX_REQUIRED..total {
        let b = emu.maps.read_byte(0x800100 + u64::from(off)).unwrap_or(0);
        let expected = sentinel_byte(usize::try_from(off).expect("test offset fits usize"));
        assert_eq!(b, expected, "byte at +0x{:x} should remain sentinel", off);
    }
}

// ---------------------------------------------------------------------------
// Process information / thread state
// ---------------------------------------------------------------------------

#[test]
fn qsi_process_information_short_buffer_reports_required_length() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(&mut emu, 0x05, 0x800100, 0x80, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INFO_LENGTH_MISMATCH);
    // Default thread (1 entry): 0x100 + 0x50 * 1 = 0x150.
    let expected = NATIVE_SYSTEM_PROCESS_INFO_PREFIX_SIZE + NATIVE_SYSTEM_THREAD_INFO_SIZE;
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(0), expected);
}

#[test]
fn qsi_process_information_writes_one_thread_by_default() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    let total = NATIVE_SYSTEM_PROCESS_INFO_PREFIX_SIZE + NATIVE_SYSTEM_THREAD_INFO_SIZE;
    setup_qsi(&mut emu, 0x05, 0x800100, total, ret_len);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(ret_len).unwrap_or(0), total);
    // NextEntryOffset at +0x00 is zero (terminator).
    assert_eq!(emu.maps.read_dword(0x800100).unwrap_or(0xFF), 0);
    // NumberOfThreads at +0x04 == 1.
    assert_eq!(emu.maps.read_dword(0x800104).unwrap_or(0), 1);
    // SYSTEM_PROCESS_INFORMATION x64 ABI: BasePriority at +0x48 and
    // UniqueProcessId at +0x50.
    assert_eq!(emu.maps.read_dword(0x800148).unwrap_or(0), 8);
    assert_eq!(emu.maps.read_qword(0x800150).unwrap_or(0), 1);
    // InheritedFromUniqueProcessId at +0x58, HandleCount at +0x60,
    // SessionId at +0x64. Non-zero sentinels make offset regressions visible.
    assert_eq!(emu.maps.read_qword(0x800158).unwrap_or(0), 4);
    assert_eq!(emu.maps.read_dword(0x800160).unwrap_or(0), 3);
    assert_eq!(emu.maps.read_dword(0x800164).unwrap_or(0), 1);
    // The first SYSTEM_THREAD_INFORMATION begins at the native prefix +0x100.
    assert_eq!(
        emu.maps.read_qword(0x800100 + 0x100 + 0x30).unwrap_or(0),
        0x1000
    );
    // Thread at +0x100: ClientId.UniqueProcess at +0x28 == 0x228.
    assert_eq!(emu.maps.read_qword(0x800228).unwrap_or(0), 1);
    // ClientId.UniqueThread at +0x100 + 0x30 == 0x230 == emu.threads[0].id (0x1000).
    assert_eq!(emu.maps.read_qword(0x800230).unwrap_or(0), 0x1000);
    // ThreadState at +0x100 + 0x44 == 0x244 == Running (2).
    assert_eq!(emu.maps.read_dword(0x800244).unwrap_or(0), 2);
}

#[test]
fn qsi_process_information_enumerates_live_threads() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    // Append three more threads with distinct scheduler states.
    let mut suspended = crate::threading::context::ThreadContext::new(0x1001, emu.cfg.arch);
    suspended.suspended = true;
    emu.threads.push(suspended);
    let mut blocked = crate::threading::context::ThreadContext::new(0x1002, emu.cfg.arch);
    blocked.blocked_on_cs = Some(0xDEAD_BEEF);
    emu.threads.push(blocked);
    let mut sleeping = crate::threading::context::ThreadContext::new(0x1003, emu.cfg.arch);
    sleeping.wake_tick = emu.tick + 10;
    emu.threads.push(sleeping);

    let thread_count = u32::try_from(emu.threads.len()).expect("test thread count fits u32");
    let total =
        NATIVE_SYSTEM_PROCESS_INFO_PREFIX_SIZE + thread_count * NATIVE_SYSTEM_THREAD_INFO_SIZE;
    let ret_len = 0x101100;
    setup_qsi(&mut emu, 0x05, 0x800100, total, ret_len);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(ret_len).unwrap_or(0), total);
    assert_eq!(emu.maps.read_dword(0x800104).unwrap_or(0), thread_count);

    let base_thread = 0x800100 + u64::from(NATIVE_SYSTEM_PROCESS_INFO_PREFIX_SIZE);
    let t1 = base_thread + u64::from(NATIVE_SYSTEM_THREAD_INFO_SIZE); // first appended thread
    let t2 = t1 + u64::from(NATIVE_SYSTEM_THREAD_INFO_SIZE);
    let t3 = t2 + u64::from(NATIVE_SYSTEM_THREAD_INFO_SIZE);
    // Suspended -> ThreadState=Waiting (5), WaitReason=Suspended (5).
    assert_eq!(emu.maps.read_dword(t1 + 0x44).unwrap_or(0), 5);
    assert_eq!(emu.maps.read_dword(t1 + 0x48).unwrap_or(0), 5);
    // Blocked on cs -> ThreadState=Waiting (5), WaitReason=WrExecutive (7).
    assert_eq!(emu.maps.read_dword(t2 + 0x44).unwrap_or(0), 5);
    assert_eq!(emu.maps.read_dword(t2 + 0x48).unwrap_or(0), 7);
    // Sleeping -> ThreadState=Waiting (5), WaitReason=DelayExecution (4).
    assert_eq!(emu.maps.read_dword(t3 + 0x44).unwrap_or(0), 5);
    assert_eq!(emu.maps.read_dword(t3 + 0x48).unwrap_or(0), 4);
}
