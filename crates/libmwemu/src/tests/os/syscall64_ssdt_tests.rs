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
    emu.regs_mut().r9 = payload.len() as u64;
    emu.maps.write_qword(emu.regs().rsp + 0x28, 0x550080); // bytes written ptr
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_qword(0x550080).unwrap_or(0),
        payload.len() as u64
    );

    emu.regs_mut().rax = WIN64_NTREADVIRTUALMEMORY;
    emu.regs_mut().rcx = !0;
    emu.regs_mut().rdx = 0x540100; // source in target region
    emu.regs_mut().r8 = 0x550100; // destination buffer
    emu.regs_mut().r9 = payload.len() as u64;
    emu.maps.write_qword(emu.regs().rsp + 0x28, 0x550088); // bytes read ptr
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_qword(0x550088).unwrap_or(0),
        payload.len() as u64
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

/// Map an I/O buffer and set the standard x64 NtQuerySystemInformation
/// register layout: RCX=class, RDX=buffer, R8=length, R9=return-length ptr.
fn setup_qsi(emu: &mut emu::Emu, class: u64, buf: u64, len: u32, ret_len: u64) {
    emu.maps
        .create_map("qsi_io", buf & !0xFFF, 0x4000, Permission::READ_WRITE)
        .expect("create qsi io map");
    emu.regs_mut().rax = WIN64_NTQUERYSYSTEMINFORMATION;
    emu.regs_mut().rcx = class;
    emu.regs_mut().rdx = buf;
    emu.regs_mut().r8 = len as u64;
    emu.regs_mut().r9 = ret_len;
}

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
fn qsi_unknown_class_returns_invalid_info_class() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(&mut emu, 0xFFF, 0x800100, 0x40, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INVALID_INFO_CLASS);
}

#[test]
fn qsi_timeofday_short_buffer_reports_required_length() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(&mut emu, 0x03, 0x800100, 0x10, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INFO_LENGTH_MISMATCH);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(0), 0x30);
}

#[test]
fn qsi_timeofday_writes_correct_size_and_timezone_id() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(&mut emu, 0x03, 0x800100, 0x30, ret_len);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(ret_len).unwrap_or(0), 0x30);
    assert_eq!(emu.maps.read_qword(0x800108).unwrap_or(0), 1);
    assert_eq!(emu.maps.read_dword(0x800118).unwrap_or(0), 0x2);
}

#[test]
fn qsi_processor_information_reports_amd64() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(&mut emu, 0x01, 0x800100, 0x18, ret_len);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(ret_len).unwrap_or(0), 0x18);
    // ProcessorArchitecture = 0x0009 (PROCESSOR_ARCHITECTURE_AMD64)
    assert_eq!(emu.maps.read_word(0x800100).unwrap_or(0), 0x0009);
    // MaximumProcessors = 1 at +6
    assert_eq!(emu.maps.read_word(0x800106).unwrap_or(0), 1);
}

#[test]
fn qsi_device_information_writes_number_of_disks() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(&mut emu, 0x07, 0x800100, 0x18, ret_len);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(ret_len).unwrap_or(0), 0x18);
    assert_eq!(emu.maps.read_dword(0x800100).unwrap_or(0), 1);
}

#[test]
fn qsi_exception_information_short_buffer() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(&mut emu, 0x21, 0x800100, 0x04, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INFO_LENGTH_MISMATCH);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(0), 0x10);
}

#[test]
fn qsi_file_cache_information_zeroes_buffer() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(&mut emu, 0x15, 0x800100, 0x40, ret_len);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(ret_len).unwrap_or(0), 0x40);
    // First 4 bytes should be zeroed.
    assert_eq!(emu.maps.read_dword(0x800100).unwrap_or(0xDEAD), 0);
}

#[test]
fn qsi_memory_list_information_writes_full_size() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(&mut emu, 0x50, 0x800100, 0xB0, ret_len);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(ret_len).unwrap_or(0), 0xB0);
}

#[test]
fn qsi_recommended_shared_data_alignment_returns_64() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(&mut emu, 0x3A, 0x800100, 0x04, ret_len);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(ret_len).unwrap_or(0), 0x04);
    assert_eq!(emu.maps.read_dword(0x800100).unwrap_or(0), 64);
}

#[test]
fn qsi_error_port_timeouts_short_buffer() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(&mut emu, 0x73, 0x800100, 0x04, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INFO_LENGTH_MISMATCH);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(0), 0x08);
}

#[test]
fn qsi_code_integrity_information_reports_enabled() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(&mut emu, 0x67, 0x800100, 0x08, ret_len);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(ret_len).unwrap_or(0), 0x08);
    assert_eq!(emu.maps.read_dword(0x800100).unwrap_or(0), 0x08);
    assert_eq!(emu.maps.read_dword(0x800104).unwrap_or(0), 0x1);
}

#[test]
fn qsi_code_integrity_policy_information_succeeds_with_zero_fill() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(&mut emu, 0xC0, 0x800100, 0x20, ret_len);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(ret_len).unwrap_or(0), 0x20);
    assert_eq!(emu.maps.read_dword(0x800100).unwrap_or(0xDEAD), 0);
}

#[test]
fn qsi_extended_handles_now_class_0x40_succeeds() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(&mut emu, 0x40, 0x800100, 0x10, ret_len);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(ret_len).unwrap_or(0), 0x10);
}

#[test]
fn qsi_class_0x37_now_returns_not_supported() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(&mut emu, 0x37, 0x800100, 0x100, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_NOT_SUPPORTED);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(1), 0);
}

#[test]
fn qsi_class_0xc5_hypervisor_returns_not_supported() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(&mut emu, 0xC5, 0x800100, 0x04, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_NOT_SUPPORTED);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(1), 0);
}

#[test]
fn qsi_supported_processor_architectures_class_0xb4_not_supported() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(&mut emu, 0xB4, 0x800100, 0x04, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_NOT_SUPPORTED);
}

#[test]
fn qsi_module_information_short_buffer_reports_required_length() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(&mut emu, 0x0B, 0x800100, 0x80, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INFO_LENGTH_MISMATCH);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(0), 0x130);
}

#[test]
fn qsi_module_information_writes_ntoskrnl_payload() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(&mut emu, 0x0B, 0x800100, 0x130, ret_len);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(ret_len).unwrap_or(0), 0x130);
    // NumberOfModules = 1 at +0x00.
    assert_eq!(emu.maps.read_dword(0x800100).unwrap_or(0), 1);
    // MappedBase at +0x10 (NumberOfModules is at +0x00, 4 pad at +0x04, module starts at +0x08).
    assert_eq!(
        emu.maps.read_qword(0x800110).unwrap_or(0),
        0xFFFF_F800_0000_0000
    );
    // ImageBase at +0x18.
    assert_eq!(
        emu.maps.read_qword(0x800118).unwrap_or(0),
        0xFFFF_F800_0000_0000
    );
    // ImageSize at +0x20.
    assert_eq!(emu.maps.read_dword(0x800120).unwrap_or(0), 0x00A0_0000);
    // LoadCount at +0x2C.
    assert_eq!(emu.maps.read_word(0x80012C).unwrap_or(0), 1);
    // OffsetToFileName at +0x2E.
    let name_off = emu.maps.read_word(0x80012E).unwrap_or(0);
    assert!(name_off > 0);
    // FullPathName at +0x30 contains the NUL-terminated ASCII path; read
    // 13 bytes including the trailing NUL to validate the basename.
    let path_bytes = emu.maps.read_bytes(0x800130 + u64::from(name_off), 13);
    assert_eq!(path_bytes, b"ntoskrnl.exe\0");
}

#[test]
fn qsi_module_information_ex_short_buffer_reports_required_length() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(&mut emu, 0x4D, 0x800100, 0x100, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INFO_LENGTH_MISMATCH);
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(0), 0x148);
}

#[test]
fn qsi_module_information_ex_writes_full_payload() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(&mut emu, 0x4D, 0x800100, 0x148, ret_len);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(ret_len).unwrap_or(0), 0x148);
    // NextOffset at +0x00 is zero (terminator).
    assert_eq!(emu.maps.read_word(0x800100).unwrap_or(0xFFFF), 0);
    // ImageBase (BaseInfo.ImageBase at +0x10+0x08) = +0x18.
    assert_eq!(
        emu.maps.read_qword(0x800118).unwrap_or(0),
        0xFFFF_F800_0000_0000
    );
    // DefaultBase at +0x140.
    assert_eq!(
        emu.maps.read_qword(0x800240).unwrap_or(0),
        0xFFFF_F800_0000_0000
    );
}

#[test]
fn qsi_processor_performance_information_writes_tick() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.pos = 12345;
    let ret_len = 0x101100;
    setup_qsi(&mut emu, 0x08, 0x800100, 0x30, ret_len);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(ret_len).unwrap_or(0), 0x30);
    // IdleTime == emu.pos at +0x00.
    assert_eq!(emu.maps.read_qword(0x800100).unwrap_or(0), 12345);
    // KernelTime == emu.pos at +0x08.
    assert_eq!(emu.maps.read_qword(0x800108).unwrap_or(0), 12345);
    // UserTime at +0x10 stays zero.
    assert_eq!(emu.maps.read_qword(0x800110).unwrap_or(1), 0);
}

#[test]
fn qsi_process_information_short_buffer_reports_required_length() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    setup_qsi(&mut emu, 0x05, 0x800100, 0x80, 0x101100);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INFO_LENGTH_MISMATCH);
    // Default thread (1 entry): 0x100 + 0x50 * 1 = 0x150.
    assert_eq!(emu.maps.read_dword(0x101100).unwrap_or(0), 0x150);
}

#[test]
fn qsi_process_information_writes_one_thread_by_default() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    let ret_len = 0x101100;
    setup_qsi(&mut emu, 0x05, 0x800100, 0x150, ret_len);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(ret_len).unwrap_or(0), 0x150);
    // NextEntryOffset at +0x00 is zero (terminator).
    assert_eq!(emu.maps.read_dword(0x800100).unwrap_or(0xFF), 0);
    // NumberOfThreads at +0x04 == 1.
    assert_eq!(emu.maps.read_dword(0x800104).unwrap_or(0), 1);
    // UniqueProcessId at +0x050 == 1.
    assert_eq!(emu.maps.read_qword(0x800150).unwrap_or(0), 1);
    // Thread at +0x100: ClientId.UniqueProcess at +0x100 + 0x28 == 0x228.
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

    let thread_count = emu.threads.len() as u32;
    let total = 0x100 + thread_count * 0x50;
    let ret_len = 0x101100;
    setup_qsi(&mut emu, 0x05, 0x800100, total, ret_len);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(ret_len).unwrap_or(0), total);
    assert_eq!(emu.maps.read_dword(0x800104).unwrap_or(0), thread_count);

    let t1 = 0x800100 + 0x100 + 0x50; // first appended thread
    let t2 = t1 + 0x50;
    let t3 = t2 + 0x50;
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
