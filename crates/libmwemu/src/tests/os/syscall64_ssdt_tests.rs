use goblin::pe::PE;
use goblin::pe::characteristic::IMAGE_FILE_DLL;
use goblin::pe::header::{CoffHeader, SIZEOF_PE_MAGIC};
use std::collections::HashSet;
use std::fs;
use std::mem::offset_of;
use std::sync::atomic::Ordering;

use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::{
    ALLOCATION_GRANULARITY, Allocation, MemoryKind, PAGE_SIZE, PageState,
};
use crate::syscall::windows::syscall64;
use crate::tests::helpers;
use crate::threading::context::ThreadContext;
use crate::windows::constants::errors::*;
use crate::windows::constants::memory::*;
use crate::windows::constants::process::*;
use crate::windows::constants::security::{SYNCHRONIZE, TOKEN_QUERY};
use crate::windows::constants::syscalls::*;
use crate::windows::constants::system::{
    ETW_ADD_NOTIFICATION_EVENT, ETW_NT_FLAGS_TRACE_EVENT_NOREG, ETW_REGISTER_GUIDS,
    ETW_SET_PROVIDER_TRAITS, MANAGE_HOT_PATCH_CHECK_ENABLED, MANAGE_HOT_PATCH_QUERY_SINGLE_PATCH,
    TIMER2_ATTRIBUTE_NO_WAKE, TIMER2_ATTRIBUTE_NOTIFICATION, WIN11_ETW_WNF_EVENT_MASK,
    WIN11_ETW_WNF_STATE,
};
use crate::windows::peb::peb64;
use crate::windows::structures::{MemoryBasicInformation64, TEB64, UnicodeString64};
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
fn canonical_syscall_names_distinguish_alpc_from_file_queries() {
    let mut unique = HashSet::new();
    for number in 0..=WIN64_NTWAITLOWEVENTPAIR {
        let name = syscall64::what_syscall(number);
        assert!(
            !name.starts_with("NtSyscall_unknown"),
            "missing {number:#x}"
        );
        assert!(unique.insert(name), "duplicate name at {number:#x}");
    }
    assert_eq!(
        syscall64::what_syscall(WIN64_NTALPCSENDWAITRECEIVEPORT),
        "NtAlpcSendWaitReceivePort"
    );
    assert_eq!(
        syscall64::what_syscall(WIN64_NTAREMAPPEDFILESTHESAME),
        "NtAreMappedFilesTheSame"
    );
    assert_eq!(
        syscall64::what_syscall(u64::MAX),
        format!("NtSyscall_unknown(0x{:x})", u64::MAX)
    );
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
    emu.maps
        .write_qword(emu.regs().rsp + 0x28, MemoryBasicInformation64::SIZE); // out length
    emu.maps.write_qword(emu.regs().rsp + 0x30, 0x500080); // return length ptr

    syscall64::gateway(&mut emu);

    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    assert_eq!(
        emu.maps.read_qword(0x500080).unwrap_or(0),
        MemoryBasicInformation64::SIZE
    );
    assert_eq!(emu.maps.read_qword(0x500100).unwrap_or(0), 0x400000);
}

#[test]
fn nt_map_view_of_section_preserves_page_permissions() {
    helpers::setup();
    let cases = [
        (PAGE_NOACCESS, Permission::NONE),
        (PAGE_READONLY, Permission::READ),
        (PAGE_READWRITE, Permission::READ_WRITE),
        (PAGE_WRITECOPY, Permission::READ_WRITE),
        (PAGE_EXECUTE, Permission::EXECUTE),
        (PAGE_EXECUTE_READ, Permission::READ_EXECUTE),
        (PAGE_EXECUTE_READWRITE, Permission::READ_WRITE_EXECUTE),
        (PAGE_EXECUTE_WRITECOPY, Permission::READ_WRITE_EXECUTE),
        (PAGE_EXECUTE_READ | PAGE_NOCACHE, Permission::READ_EXECUTE),
    ];
    for (protection, expected) in cases {
        let mut emu = setup_emu64_syscall();
        emu.syscall_name_by_real
            .insert(WIN64_NTMAPVIEWOFSECTION, "NtMapViewOfSection".into());
        let rsp = emu.regs().rsp;
        let base_ptr = rsp + 0x100;
        let size_ptr = base_ptr + 8;

        let maximum = base_ptr + 16;
        let section_out = maximum + 8;
        assert!(emu.maps.write_qword(maximum, PAGE_SIZE));
        assert_eq!(
            native_test_call(
                &mut emu,
                WIN64_NTCREATESECTION,
                &[
                    section_out,
                    u64::from(SECTION_ALL_ACCESS),
                    0,
                    maximum,
                    u64::from(PAGE_EXECUTE_READWRITE),
                    u64::from(SEC_COMMIT),
                    0
                ]
            ),
            STATUS_SUCCESS
        );
        let section_handle = emu.maps.read_qword(section_out).unwrap();
        emu.maps.write_qword(rsp + 0x28, 0);
        emu.maps.write_qword(rsp + 0x30, 0);
        assert!(emu.maps.write_qword(base_ptr, 0));
        assert!(emu.maps.write_qword(size_ptr, PAGE_SIZE));
        assert!(emu.maps.write_qword(rsp + 0x38, size_ptr));
        assert!(emu.maps.write_dword(rsp + 0x40, VIEW_SHARE));
        assert!(emu.maps.write_dword(rsp + 0x50, protection));
        emu.regs_mut().rax = WIN64_NTMAPVIEWOFSECTION;
        emu.regs_mut().rcx = section_handle;
        emu.regs_mut().rdx = NT_CURRENT_PROCESS;
        emu.regs_mut().r8 = base_ptr;
        emu.regs_mut().r9 = 0;

        syscall64::gateway(&mut emu);

        if matches!(protection, PAGE_WRITECOPY | PAGE_EXECUTE_WRITECOPY) {
            assert_eq!(emu.regs().rax, STATUS_NOT_IMPLEMENTED);
            assert_eq!(emu.maps.read_qword(base_ptr), Some(0));
            continue;
        }
        assert_eq!(emu.regs().rax, STATUS_SUCCESS, "protection {protection:#x}");
        let base = emu.maps.read_qword(base_ptr).expect("mapped base");
        let map = emu.maps.get_mem_by_addr(base).expect("mapped section view");
        assert_eq!(map.permission(), expected, "protection {protection:#x}");
        assert_eq!(emu.maps.read_qword(size_ptr), Some(PAGE_SIZE));
    }
}

#[test]
fn mapping_a_known_dll_leaves_the_guest_loader_lists_unchanged() {
    let dir = tempfile::tempdir().unwrap();
    let template = PE::parse(helpers::LOADER64).unwrap();
    let import = template
        .imports
        .iter()
        .find(|entry| entry.dll.eq_ignore_ascii_case("kernel32.dll") && entry.rva != 0)
        .unwrap();
    let coff = template.header.dos_header.pe_pointer as usize + SIZEOF_PE_MAGIC;
    let flags = coff + offset_of!(CoffHeader, characteristics);
    let mut dll = helpers::LOADER64.to_vec();
    dll[flags..flags + size_of::<u16>()].copy_from_slice(
        &(template.header.coff_header.characteristics | IMAGE_FILE_DLL).to_le_bytes(),
    );
    fs::write(dir.path().join("probe.dll"), dll).unwrap();

    let mut emu = setup_emu64_syscall();
    emu.cfg.maps_folder = dir.path().display().to_string();
    emu.cfg.winver = None;
    peb64::init_peb(&mut emu);
    let loader_lists = emu.maps.get_mem("ldr").get_bytes().into_owned();
    let section = 0x1234;
    emu.section_handles.insert(section, "probe.dll".into());
    let base_out = emu.sp() + 0x100;
    let size_out = base_out + 8;
    let args = [
        section,
        NT_CURRENT_PROCESS,
        base_out,
        0,
        0,
        0,
        size_out,
        VIEW_SHARE as u64,
        0,
        PAGE_READONLY as u64,
    ];

    for _ in 0..2 {
        emu.maps.write_qword(base_out, 0);
        emu.maps.write_qword(size_out, 0);
        assert_eq!(
            native_test_call(&mut emu, WIN64_NTMAPVIEWOFSECTION, &args),
            STATUS_SUCCESS
        );
        let header = emu.maps.get_mem("probe.pe");
        let image = emu.maps.allocation_at(header.get_base()).unwrap();
        assert_eq!(image.kind, MemoryKind::Image);
        assert_eq!(emu.maps.read_qword(base_out), Some(image.base));
        assert_eq!(emu.maps.read_qword(size_out), Some(image.size));
        assert_eq!(emu.maps.get_mem("ldr").get_bytes().as_ref(), loader_lists);
        assert_eq!(
            emu.maps.read_qword(image.base + import.offset as u64),
            Some(import.rva as u64)
        );
        assert!(emu.maps.get_map_by_name("kernel32.pe").is_none());
    }
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
    assert_eq!(
        emu.maps.read_bytes(0x550100, payload.len()).as_ref(),
        payload
    );
}

#[test]
fn invalid_native_open_preserves_the_output_and_current_process_termination_stops_execution() {
    helpers::setup();
    let mut emu = setup_emu64_syscall();
    emu.maps
        .create_map("io", 0x570000, 0x1000, Permission::READ_WRITE)
        .expect("create io");

    let output = 0x570080;
    let sentinel = 0xfa11;
    emu.maps.write_qword(output, sentinel);
    emu.regs_mut().rax = WIN64_NTOPENPROCESS;
    emu.regs_mut().rcx = output;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().r8 = 0;
    emu.regs_mut().r9 = 0;
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_ACCESS_VIOLATION);
    assert_eq!(emu.maps.read_qword(output), Some(sentinel));

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
    assert_eq!(path_bytes.as_ref(), b"ntoskrnl.exe\0");
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
    let mut suspended = ThreadContext::new(0x1001, emu.cfg.arch);
    suspended.suspended = true;
    emu.threads.push(suspended);
    let mut blocked = ThreadContext::new(0x1002, emu.cfg.arch);
    blocked.blocked_on_cs = Some(0xDEAD_BEEF);
    emu.threads.push(blocked);
    let mut sleeping = ThreadContext::new(0x1003, emu.cfg.arch);
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

#[test]
fn file_section_syscalls_map_content_and_unmap_the_view() {
    let mut emu = setup_emu64_syscall();
    let image = 0x600000;
    let file = 0x123;
    let header_offset = 0x80;
    let marker_offset = 0x300;
    let marker = 0x0123_4567_89ab_cdef;
    let sp = emu.regs().rsp;
    let handle_ptr = sp + 0x100;
    let base_ptr = handle_ptr + 8;
    let size_ptr = base_ptr + 8;
    emu.maps
        .create_map("fixture.pe", image, PAGE_SIZE, Permission::READ_WRITE)
        .unwrap();
    emu.maps.write_dword(image + 0x3c, header_offset);
    emu.maps
        .write_dword(image + header_offset as u64 + 0x50, PAGE_SIZE as u32);
    emu.maps.write_qword(image + marker_offset, marker);
    emu.file_handles.insert(file, "fixture.dll".into());
    emu.regs_mut().rax = WIN64_NTCREATESECTION;
    emu.regs_mut().rcx = handle_ptr;
    emu.maps.write_qword(sp + 0x28, PAGE_READONLY as u64);
    emu.maps.write_qword(sp + 0x30, SEC_IMAGE as u64);
    emu.maps.write_qword(sp + 0x38, file);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    let section = emu.maps.read_qword(handle_ptr).unwrap();
    assert!(emu.file_section_names.contains_key(&section));
    assert!(!emu.section_handles.contains_key(&section));
    emu.maps.write_qword(base_ptr, 0);
    emu.maps.write_qword(size_ptr, 0);
    emu.regs_mut().rax = WIN64_NTMAPVIEWOFSECTION;
    emu.regs_mut().rcx = section;
    emu.regs_mut().rdx = NT_CURRENT_PROCESS;
    emu.regs_mut().r8 = base_ptr;
    emu.regs_mut().r9 = 0;
    emu.maps.write_qword(sp + 0x28, 0);
    emu.maps.write_qword(sp + 0x30, 0);
    emu.maps.write_qword(sp + 0x38, size_ptr);
    emu.maps.write_qword(sp + 0x40, VIEW_SHARE as u64);
    emu.maps.write_qword(sp + 0x48, 0);
    emu.maps.write_qword(sp + 0x50, PAGE_READONLY as u64);
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_SUCCESS);
    let view = emu.maps.read_qword(base_ptr).unwrap();
    assert_ne!(view, image);
    assert_eq!(emu.maps.read_qword(size_ptr), Some(PAGE_SIZE));
    assert_eq!(emu.maps.read_qword(view + marker_offset), Some(marker));
    let name_out = sp + 0x200;
    let returned = sp + 0x180;
    assert_eq!(
        native_test_call(
            &mut emu,
            WIN64_NTQUERYVIRTUALMEMORY,
            &[
                NT_CURRENT_PROCESS,
                view + marker_offset,
                MEMORY_INFORMATION_CLASS_MEMORY_MAPPED_FILENAME_INFORMATION,
                name_out,
                256,
                returned
            ]
        ),
        STATUS_SUCCESS
    );
    let name = UnicodeString64::load(name_out, &emu.maps);
    assert_eq!(name.buffer, name_out + UnicodeString64::size());
    assert_eq!(
        emu.maps.read_wide_string(name.buffer),
        r"\Device\HarddiskVolume2\Windows\System32\fixture.dll"
    );
    assert_eq!(
        emu.maps.read_qword(returned),
        Some(UnicodeString64::size() + name.maximum_length as u64)
    );
    assert_eq!(
        emu.section_view_dll.get(&view).map(String::as_str),
        Some("fixture.dll")
    );
    assert_eq!(
        native_test_call(
            &mut emu,
            WIN64_NTUNMAPVIEWOFSECTION,
            &[NT_CURRENT_PROCESS, view]
        ),
        STATUS_SUCCESS
    );
    assert!(!emu.maps.is_mapped(view));
    assert_eq!(emu.maps.read_qword(image + marker_offset), Some(marker));
}

#[test]
fn file_section_mapping_rejects_invalid_outputs_ranges_and_missing_images() {
    let mut emu = setup_emu64_syscall();
    let image = 0x600000;
    let section = 0x123;
    let header_offset = 0x80;
    let sp = emu.regs().rsp;
    let base_ptr = sp + 0x100;
    let size_ptr = base_ptr + 8;
    emu.maps
        .create_map("fixture.pe", image, PAGE_SIZE, Permission::READ_WRITE)
        .unwrap();
    emu.maps.write_dword(image + 0x3c, header_offset);
    emu.maps
        .write_dword(image + header_offset as u64 + 0x50, PAGE_SIZE as u32);
    emu.file_section_names.insert(section, "fixture.dll".into());
    for (output, requested_base, requested_size, expected) in [
        (1, 0, 0, STATUS_ACCESS_VIOLATION),
        (base_ptr, image + 1, 0, STATUS_MAPPED_ALIGNMENT),
        (base_ptr, image, 0, STATUS_CONFLICTING_ADDRESSES),
        (base_ptr, 0, u64::MAX, STATUS_NO_MEMORY),
    ] {
        emu.maps.write_qword(base_ptr, requested_base);
        emu.maps.write_qword(size_ptr, requested_size);
        emu.maps.write_qword(sp + 0x38, size_ptr);
        emu.maps.write_qword(sp + 0x50, PAGE_READONLY as u64);
        emu.regs_mut().rax = WIN64_NTMAPVIEWOFSECTION;
        emu.regs_mut().rcx = section;
        emu.regs_mut().rdx = NT_CURRENT_PROCESS;
        emu.regs_mut().r8 = output;
        syscall64::gateway(&mut emu);
        assert_eq!(emu.regs().rax, expected);
        assert_eq!(emu.maps.read_qword(base_ptr), Some(requested_base));
        assert_eq!(emu.maps.read_qword(size_ptr), Some(requested_size));
    }
    emu.file_section_names.insert(section, "absent.dll".into());
    emu.maps.write_qword(base_ptr, 0);
    emu.maps.write_qword(size_ptr, 0);
    emu.regs_mut().rax = WIN64_NTMAPVIEWOFSECTION;
    emu.regs_mut().rcx = section;
    emu.regs_mut().rdx = NT_CURRENT_PROCESS;
    emu.regs_mut().r8 = base_ptr;
    syscall64::gateway(&mut emu);
    assert_eq!(emu.regs().rax, STATUS_INVALID_IMAGE_FORMAT);
    assert_eq!(emu.maps.read_qword(base_ptr), Some(0));
}

#[test]
fn native_object_syscalls_and_windows_apis_share_handles_and_error_contracts() {
    fn syscall(emu: &mut emu::Emu, number: u64, arguments: &[u64]) -> u64 {
        let sp = emu.sp();
        let regs = emu.regs_mut();
        regs.rax = number;
        for (index, value) in arguments.iter().copied().enumerate() {
            match index {
                0 => emu.regs_mut().rcx = value,
                1 => emu.regs_mut().rdx = value,
                2 => emu.regs_mut().r8 = value,
                3 => emu.regs_mut().r9 = value,
                _ => {
                    assert!(emu.maps.write_qword(sp + (index as u64 + 1) * 8, value));
                }
            }
        }
        syscall64::gateway(emu);
        assert_eq!(emu.sp(), sp);
        assert!(!emu.api_policy.stopped());
        emu.regs().rax
    }
    let (mut emu, out) = helpers::prepare_winapi(true);
    emu.set_sp(emu.sp() - 0x100);
    for (number, name) in [
        (WIN64_NTOPENPROCESS, "NtOpenProcess"),
        (WIN64_NTOPENPROCESSTOKEN, "NtOpenProcessToken"),
        (WIN64_NTDUPLICATEOBJECT, "NtDuplicateObject"),
        (WIN64_NTCLOSE, "NtClose"),
    ] {
        emu.syscall_name_by_real.insert(number, name.into());
    }
    let pid = helpers::call_winapi_export(&mut emu, "kernel32.dll", "GetCurrentProcessId", &[]);
    let attributes = out + 16;
    let client = attributes + 48;
    emu.maps.write_dword(attributes, 48);
    emu.maps.write_qword(client, pid);
    let rights = u64::from(PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_DUP_HANDLE);
    assert_eq!(
        syscall(
            &mut emu,
            WIN64_NTOPENPROCESS,
            &[out, rights, attributes, client]
        ),
        STATUS_SUCCESS
    );
    let process = emu.maps.read_qword(out).unwrap();
    assert_eq!(
        helpers::call_winapi_export(&mut emu, "kernel32.dll", "GetProcessId", &[process]),
        pid
    );
    assert_eq!(
        syscall(
            &mut emu,
            WIN64_NTOPENPROCESSTOKEN,
            &[process, u64::from(TOKEN_QUERY), out]
        ),
        STATUS_SUCCESS
    );
    let token = emu.maps.read_qword(out).unwrap();
    assert_eq!(
        emu.handle_management.process_token_rights(token),
        Some(TOKEN_QUERY)
    );
    let arguments = [
        NT_CURRENT_PROCESS,
        process,
        NT_CURRENT_PROCESS,
        out,
        0,
        0,
        u64::from(DUPLICATE_SAME_ACCESS),
    ];
    assert_eq!(
        syscall(&mut emu, WIN64_NTDUPLICATEOBJECT, &arguments),
        STATUS_SUCCESS
    );
    let copy = emu.maps.read_qword(out).unwrap();
    assert_ne!(copy, process);
    assert_eq!(syscall(&mut emu, WIN64_NTCLOSE, &[process]), STATUS_SUCCESS);
    assert_eq!(
        helpers::call_winapi_export(&mut emu, "kernel32.dll", "GetProcessId", &[copy]),
        pid
    );
    assert_eq!(
        helpers::call_winapi_export(&mut emu, "ntdll.dll", "ZwClose", &[token]),
        STATUS_SUCCESS
    );
    assert_eq!(
        syscall(&mut emu, WIN64_NTCLOSE, &[token]),
        STATUS_INVALID_HANDLE
    );
    let invalid = 0x7fff0000;
    let mut invalid_dup = arguments;
    invalid_dup[1] = invalid;
    assert_eq!(
        syscall(&mut emu, WIN64_NTDUPLICATEOBJECT, &invalid_dup),
        STATUS_INVALID_HANDLE
    );
    assert_eq!(emu.maps.read_qword(out), Some(0));
    emu.maps.write_qword(out, copy);
    assert_eq!(
        syscall(&mut emu, WIN64_NTOPENPROCESSTOKEN, &[invalid, 0, out]),
        STATUS_INVALID_HANDLE
    );
    assert_eq!(emu.maps.read_qword(out), Some(copy));
    assert_eq!(
        helpers::call_winapi_export(&mut emu, "kernel32.dll", "CloseHandle", &[copy]),
        1
    );
    assert_eq!(
        syscall(&mut emu, WIN64_NTCLOSE, &[copy]),
        STATUS_INVALID_HANDLE
    );
}

#[test]
fn native_handle_calls_respect_x86_guest_pointer_width() {
    let (mut emu, out) = helpers::prepare_winapi(false);
    let current = u64::from(NT_CURRENT_PROCESS as u32);
    let guard = 0x11223344;
    emu.maps.write_dword(out + 4, guard);
    assert_eq!(
        helpers::call_winapi_export(
            &mut emu,
            "ntdll.dll",
            "ZwOpenProcessToken",
            &[current, u64::from(TOKEN_QUERY), out]
        ),
        STATUS_SUCCESS
    );
    let token = u64::from(emu.maps.read_dword(out).unwrap());
    assert_eq!(emu.maps.read_dword(out + 4), Some(guard));
    assert_eq!(
        helpers::call_winapi_export(&mut emu, "kernel32.dll", "CloseHandle", &[token]),
        1
    );
    assert_eq!(
        helpers::call_winapi_export(
            &mut emu,
            "ntdll.dll",
            "NtDuplicateObject",
            &[
                current,
                current,
                current,
                out,
                0,
                0,
                u64::from(DUPLICATE_SAME_ACCESS)
            ]
        ),
        STATUS_SUCCESS
    );
    assert_eq!(emu.maps.read_dword(out + 4), Some(guard));
    let copy = u64::from(emu.maps.read_dword(out).unwrap());
    assert_eq!(
        helpers::call_winapi_export(&mut emu, "ntdll.dll", "NtClose", &[copy]),
        STATUS_SUCCESS
    );
}

fn native_test_call(emu: &mut emu::Emu, number: u64, args: &[u64]) -> u64 {
    emu.syscall_name_by_real
        .insert(number, syscall64::what_syscall(number));
    emu.regs_mut().rax = number;
    for (index, value) in args.iter().copied().enumerate() {
        match index {
            0 => emu.regs_mut().rcx = value,
            1 => emu.regs_mut().rdx = value,
            2 => emu.regs_mut().r8 = value,
            3 => emu.regs_mut().r9 = value,
            _ => assert!(
                emu.maps
                    .write_qword(emu.sp() + (index as u64 + 1) * 8, value)
            ),
        }
    }
    syscall64::gateway(emu);
    emu.regs().rax
}

fn native_name(emu: &mut emu::Emu, descriptor: u64, text: &str) {
    let bytes = (text.encode_utf16().count() * 2) as u16;
    emu.maps.write_word(descriptor, bytes);
    emu.maps.write_word(descriptor + 2, bytes + 2);
    emu.maps.write_qword(descriptor + 8, descriptor + 16);
    emu.maps.write_wide_string(descriptor + 16, text);
}

#[test]
fn native_unsupported_services_preserve_outputs_and_follow_the_requested_mode() {
    use crate::winapi::policy::ApiMode;

    let out = 0x101200;
    for mode in [
        ApiMode::Strict,
        ApiMode::SimulateFail,
        ApiMode::SimulateSuccess,
    ] {
        for number in [
            WIN64_NTCREATETHREADEX,
            WIN64_NTRAISEHARDERROR,
            WIN64_NTSETINFORMATIONPROCESS,
            WIN64_NTSETINFORMATIONTHREAD,
            WIN64_NTWAITFORALERTBYTHREADID,
            WIN64_NTALPCCREATEPORTSECTION,
            WIN64_NTALPCQUERYINFORMATION,
            WIN64_NTOPENKEYTRANSACTED,
            WIN64_NTQUERYMULTIPLEVALUEKEY,
            WIN64_NTQUERYIOCOMPLETION,
            WIN64_NTSUBSCRIBEWNFSTATECHANGE,
        ] {
            let mut emu = setup_emu64_syscall();
            emu.set_api_mode(mode);
            emu.maps.write_qword(out, u64::MAX);
            let status = native_test_call(
                &mut emu,
                number,
                &[out, out, out, out, 8, out, 0, 0, 0, 0, 0],
            );
            assert_eq!(
                emu.api_policy.stopped(),
                mode == ApiMode::Strict,
                "{number:#x}"
            );
            if mode == ApiMode::SimulateFail {
                assert_eq!(status, STATUS_NOT_IMPLEMENTED, "{number:#x}");
            }
            assert_eq!(emu.maps.read_qword(out), Some(u64::MAX), "{number:#x}");
        }
    }
}

#[test]
fn native_knowndll_links_validate_root_access_output_and_lifetime() {
    let mut emu = setup_emu64_syscall();
    let out = 0x101200;
    let object = out + 16;
    let descriptor = object + 48;
    let returned = out + 256;
    emu.maps.write_dword(object, 48);
    emu.maps.write_qword(object + 16, descriptor);
    native_name(&mut emu, descriptor, r"\KnownDlls32");
    let open = |emu: &mut emu::Emu, nr, access| native_test_call(emu, nr, &[out, access, object]);
    assert_eq!(
        open(&mut emu, WIN64_NTOPENDIRECTORYOBJECT, 0),
        STATUS_SUCCESS
    );
    let no_traverse = emu.maps.read_qword(out).unwrap();
    assert_eq!(
        open(&mut emu, WIN64_NTOPENDIRECTORYOBJECT, 2),
        STATUS_SUCCESS
    );
    let directory = emu.maps.read_qword(out).unwrap();
    native_name(&mut emu, descriptor, "KnownDllPath");
    emu.maps.write_qword(object + 8, no_traverse);
    assert_eq!(
        open(&mut emu, WIN64_NTOPENSYMBOLICLINKOBJECT, 1),
        STATUS_ACCESS_DENIED
    );
    emu.maps.write_qword(object + 8, directory);
    assert_eq!(
        open(&mut emu, WIN64_NTOPENSYMBOLICLINKOBJECT, 1),
        STATUS_SUCCESS
    );
    let link = emu.maps.read_qword(out).unwrap();
    let query = |emu: &mut emu::Emu| {
        native_test_call(
            emu,
            WIN64_NTQUERYSYMBOLICLINKOBJECT,
            &[link, descriptor, returned],
        )
    };
    emu.maps.write_word(descriptor + 2, 0);
    assert_eq!(query(&mut emu), STATUS_BUFFER_TOO_SMALL);
    let target = r"C:\Windows\SysWOW64";
    assert_eq!(
        emu.maps.read_dword(returned),
        Some((target.len() * 2) as u32)
    );
    emu.maps.write_word(descriptor + 2, 128);
    emu.maps.write_qword(descriptor + 8, 0x11ffff);
    assert_eq!(query(&mut emu), STATUS_ACCESS_VIOLATION);
    emu.maps.write_qword(descriptor + 8, descriptor + 16);
    assert_eq!(query(&mut emu), STATUS_SUCCESS);
    assert_eq!(emu.maps.read_wide_string(descriptor + 16), target);
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTCLOSE, &[link]),
        STATUS_SUCCESS
    );
    assert_eq!(query(&mut emu), STATUS_INVALID_HANDLE);
}

#[test]
fn native_csr_port_checks_message_capacity_and_service_before_writing() {
    use crate::winapi::policy::ApiMode;

    let mut emu = setup_emu64_syscall();
    emu.set_api_mode(ApiMode::SimulateFail);
    let out = 0x102000;
    let descriptor = out + 16;
    let send = out + 128;
    let receive = out + 256;
    let length = out + 512;
    native_name(&mut emu, descriptor, r"\Sessions\1\Windows\ApiPort");
    assert_eq!(
        native_test_call(
            &mut emu,
            WIN64_NTALPCCONNECTPORT,
            &[out, descriptor, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ),
        STATUS_SUCCESS
    );
    let port = emu.maps.read_qword(out).unwrap();
    let quality = out + 640;
    emu.maps.write_dword(quality, 12);
    let client_view = quality + 16;
    emu.maps.write_dword(client_view, 48);
    emu.maps.write_qword(client_view + 8, u64::MAX);
    assert_eq!(
        native_test_call(
            &mut emu,
            WIN64_NTCONNECTPORT,
            &[out, descriptor, quality, client_view, 0, length, 0, 0]
        ),
        STATUS_INVALID_HANDLE
    );
    assert_eq!(emu.maps.read_qword(out), Some(port));
    assert_eq!(
        native_test_call(
            &mut emu,
            WIN64_NTCONNECTPORT,
            &[out, descriptor, quality, 0, 0, length, 0, 0]
        ),
        STATUS_SUCCESS
    );
    assert_eq!(emu.maps.read_dword(length), Some(512));
    let message = [port, 0x20000, send, 0, receive, length, 0, 0];
    const TOTAL: usize = 0x58;
    emu.maps.write_word(send, (TOTAL - 0x28) as u16);
    emu.maps.write_word(send + 2, TOTAL as u16);
    emu.maps.write_dword(send + 0x34, u32::MAX);
    emu.maps.write_qword(receive, u64::MAX);
    emu.maps.write_qword(length, TOTAL as u64 - 1);
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTALPCSENDWAITRECEIVEPORT, &message),
        STATUS_BUFFER_TOO_SMALL
    );
    assert_eq!(emu.maps.read_qword(receive), Some(u64::MAX));
    assert_eq!(emu.maps.read_qword(length), Some(TOTAL as u64));
    emu.maps.write_byte(receive + TOTAL as u64, 0xa5);
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTALPCSENDWAITRECEIVEPORT, &message),
        STATUS_SUCCESS
    );
    assert_eq!(emu.maps.read_word(receive + 4), Some(2));
    assert_eq!(emu.maps.read_dword(receive + 0x34), Some(0));
    assert_eq!(emu.maps.read_byte(receive + TOTAL as u64), Some(0xa5));
    emu.maps.write_dword(send + 0x30, 17);
    emu.maps.write_qword(receive, u64::MAX);
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTALPCSENDWAITRECEIVEPORT, &message),
        STATUS_NOT_IMPLEMENTED
    );
    assert_eq!(emu.maps.read_qword(receive), Some(u64::MAX));
}

#[test]
fn native_nls_and_empty_wnf_data_validate_each_output() {
    let mut emu = setup_emu64_syscall();
    let address = 0x102000;
    let length = address + 8;
    let end = 0x120000;
    let profile = tempfile::tempdir().unwrap();
    std::fs::write(profile.path().join("C_1252.NLS"), [7; 512]).unwrap();
    emu.cfg.maps_folder = profile.path().to_str().unwrap().into();
    emu.maps.write_qword(address, u64::MAX);
    let nls = |emu: &mut emu::Emu, size| {
        native_test_call(emu, WIN64_NTGETNLSSECTIONPTR, &[11, 1252, 0, address, size])
    };
    assert_eq!(nls(&mut emu, end - 4), STATUS_ACCESS_VIOLATION);
    assert_eq!(emu.maps.read_qword(address), Some(u64::MAX));
    emu.maps.write_qword(length, u64::MAX);
    assert_eq!(nls(&mut emu, length), STATUS_SUCCESS);
    assert_eq!(emu.maps.read_qword(length), Some(PAGE_SIZE));
    assert_eq!(
        emu.maps.read_byte(emu.maps.read_qword(address).unwrap()),
        Some(7)
    );
    let state = address + 32;
    emu.maps.write_qword(state, 0x418a_073a_a3bc_0c75);
    emu.maps.write_qword(address, u64::MAX);
    let wnf = |emu: &mut emu::Emu, size| {
        native_test_call(
            emu,
            WIN64_NTQUERYWNFSTATEDATA,
            &[state, 0, 0, address, 0, size],
        )
    };
    assert_eq!(wnf(&mut emu, end - 2), STATUS_ACCESS_VIOLATION);
    assert_eq!(emu.maps.read_qword(address), Some(u64::MAX));
    assert_eq!(wnf(&mut emu, length), STATUS_SUCCESS);
    assert_eq!(emu.maps.read_qword(address), Some((u32::MAX as u64) << 32));
    assert_eq!(emu.maps.read_dword(length), Some(0));
}

#[test]
fn native_allocation_honors_extended_address_requirements() {
    const BASE: u64 = 0x101200;
    const SIZE: u64 = BASE + 8;
    const PARAMETER: u64 = SIZE + 8;
    const REQUIREMENTS: u64 = PARAMETER + 16;
    const ALIGNMENT: u64 = 2 * ALLOCATION_GRANULARITY;
    const LOW: u64 = 0x1000_0000;
    const HIGH: u64 = LOW + 4 * ALIGNMENT - 1;
    let mut emu = setup_emu64_syscall();
    let args = [
        NT_CURRENT_PROCESS,
        BASE,
        SIZE,
        (MEM_RESERVE | MEM_TOP_DOWN) as u64,
        PAGE_READWRITE as u64,
        PARAMETER,
        1,
    ];
    emu.maps
        .write_qword(PARAMETER, MEM_EXTENDED_PARAMETER_ADDRESS_REQUIREMENTS);
    emu.maps.write_qword(PARAMETER + 8, REQUIREMENTS);
    for (low, high, alignment, expected) in [
        (0, 0, 0, STATUS_SUCCESS),
        (LOW, HIGH, ALIGNMENT, STATUS_SUCCESS),
        (LOW, HIGH, PAGE_SIZE, STATUS_INVALID_PARAMETER),
    ] {
        emu.maps.write_qword(BASE, 0);
        emu.maps.write_qword(SIZE, PAGE_SIZE);
        emu.maps.write_qword(REQUIREMENTS, low);
        emu.maps.write_qword(REQUIREMENTS + 8, high);
        emu.maps.write_qword(REQUIREMENTS + 16, alignment);
        assert_eq!(
            native_test_call(&mut emu, WIN64_NTALLOCATEVIRTUALMEMORYEX, &args),
            expected
        );
        let result = emu.maps.read_qword(BASE).unwrap();
        if expected == STATUS_SUCCESS {
            assert_ne!(result, 0);
            assert!(result.is_multiple_of(alignment.max(ALLOCATION_GRANULARITY)));
            if high != 0 {
                assert_eq!(result, (high + 1 - PAGE_SIZE) & !(alignment - 1));
            }
        } else {
            assert_eq!(result, 0);
        }
    }
    emu.maps.write_qword(PARAMETER + 8, u64::MAX);
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTALLOCATEVIRTUALMEMORYEX, &args),
        STATUS_ACCESS_VIOLATION
    );
}

#[test]
fn completion_bootstrap_reports_target_state_on_both_windows_abis() {
    const OUTPUT: u64 = 0x101200;
    const FLAG: u64 = OUTPUT + 8;
    for mut emu in [emu32(), emu64()] {
        emu.maps
            .create_map("outputs", OUTPUT, PAGE_SIZE, Permission::READ_WRITE)
            .unwrap();
        let call = |emu: &mut emu::Emu, name, args: &[u64]| {
            helpers::call_winapi_export(emu, "ntdll.dll", name, args)
        };
        let mut create = |name, args: &[u64]| {
            assert_eq!(call(&mut emu, name, args), STATUS_SUCCESS);
            emu.maps.read_qword(OUTPUT).unwrap()
        };
        let port = create("NtCreateIoCompletion", &[OUTPUT, 2, 0, 1]);
        let packet = create("NtCreateWaitCompletionPacket", &[OUTPUT, 1, 0]);
        let event = create("NtCreateEvent", &[OUTPUT, SYNCHRONIZE as u64, 0, 0, 0]);
        let associate = |emu: &mut emu::Emu, packet, flag| {
            call(
                emu,
                "NtAssociateWaitCompletionPacket",
                &[packet, port, event, 0, 0, 0, 0, flag],
            )
        };
        emu.maps.write_dword(FLAG, u32::MAX);
        assert_eq!(associate(&mut emu, packet, FLAG), STATUS_SUCCESS);
        assert_eq!(emu.maps.read_dword(FLAG), Some(0xffff_ff00));
        assert_eq!(call(&mut emu, "NtSetEvent", &[event, 0]), STATUS_SUCCESS);
        assert_eq!(associate(&mut emu, packet, FLAG), STATUS_SUCCESS);
        assert_eq!(emu.maps.read_dword(FLAG), Some(0xffff_ff01));
        assert_eq!(associate(&mut emu, packet, 0), STATUS_SUCCESS);
        assert_eq!(associate(&mut emu, packet, 1), STATUS_ACCESS_VIOLATION);
        assert_eq!(call(&mut emu, "NtClose", &[packet]), STATUS_SUCCESS);
        assert_eq!(associate(&mut emu, packet, FLAG), STATUS_INVALID_HANDLE);
    }
}

#[test]
fn loader_notification_stubs_initialize_only_their_outputs() {
    const STATE: u64 = 0x101200;
    const OUTPUT: u64 = STATE + 16;
    const EVENT: u64 = OUTPUT + 16;
    let mut emu = setup_emu64_syscall();
    emu.maps.write_qword(STATE, WIN11_ETW_WNF_STATE);
    let subscribe = [STATE, 0, WIN11_ETW_WNF_EVENT_MASK as u64, OUTPUT];
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTSUBSCRIBEWNFSTATECHANGE, &subscribe),
        STATUS_SUCCESS
    );
    let id = emu.maps.read_qword(OUTPUT).unwrap();
    assert_ne!(id, 0);
    emu.maps.write_qword(OUTPUT, 0);
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTSUBSCRIBEWNFSTATECHANGE, &subscribe),
        STATUS_SUCCESS
    );
    assert_eq!(emu.maps.read_qword(OUTPUT), Some(id));
    assert_eq!(
        native_test_call(
            &mut emu,
            WIN64_NTCREATEEVENT,
            &[EVENT, SYNCHRONIZE as u64, 0, 0, 0]
        ),
        STATUS_SUCCESS
    );
    let event = emu.maps.read_qword(EVENT).unwrap();
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTSETWNFPROCESSNOTIFICATIONEVENT, &[event]),
        STATUS_SUCCESS
    );
    emu.maps.write_qword(OUTPUT, u64::MAX);
    assert_eq!(
        native_test_call(
            &mut emu,
            WIN64_NTTRACECONTROL,
            &[ETW_ADD_NOTIFICATION_EVENT as u64, EVENT, 4, 0, 0, OUTPUT]
        ),
        STATUS_SUCCESS
    );
    assert_eq!(emu.maps.read_qword(OUTPUT), Some(0xffff_ffff_0000_0000));
    assert!(!emu.api_policy.stopped());
}

#[test]
fn native_timer2_shares_wait_and_close_semantics() {
    const HANDLE: u64 = 0x101200;
    const TIMEOUT: u64 = HANDLE + 8;
    const TIMER_MODIFY_STATE: u32 = 2;
    const ACCESS: u64 = (SYNCHRONIZE | TIMER_MODIFY_STATE) as u64;
    let mut emu = setup_emu64_syscall();
    for flags in [TIMER2_ATTRIBUTE_NO_WAKE, TIMER2_ATTRIBUTE_NOTIFICATION] {
        assert_eq!(
            native_test_call(
                &mut emu,
                WIN64_NTCREATETIMER2,
                &[HANDLE, 0, 0, flags as u64, ACCESS]
            ),
            STATUS_SUCCESS
        );
        let timer = emu.maps.read_qword(HANDLE).unwrap();
        let poll = [timer, 0, TIMEOUT];
        assert_eq!(
            native_test_call(&mut emu, WIN64_NTWAITFORSINGLEOBJECT, &poll),
            STATUS_TIMEOUT
        );
        assert_eq!(
            helpers::call_winapi_export(
                &mut emu,
                "kernel32.dll",
                "SetWaitableTimer",
                &[timer, TIMEOUT, 0, 0, 0, 0]
            ),
            1
        );
        assert_eq!(
            native_test_call(&mut emu, WIN64_NTWAITFORSINGLEOBJECT, &poll),
            STATUS_SUCCESS
        );
        let next = if flags == TIMER2_ATTRIBUTE_NOTIFICATION {
            STATUS_SUCCESS
        } else {
            STATUS_TIMEOUT
        };
        assert_eq!(
            native_test_call(&mut emu, WIN64_NTWAITFORSINGLEOBJECT, &poll),
            next
        );
        assert_eq!(
            native_test_call(&mut emu, WIN64_NTCLOSE, &[timer]),
            STATUS_SUCCESS
        );
    }
    let previous = emu.maps.read_qword(HANDLE);
    for (id, flags, expected) in [
        (0, 1, STATUS_INVALID_PARAMETER_4),
        (TIMEOUT, 0, STATUS_INVALID_PARAMETER_2),
    ] {
        assert_eq!(
            native_test_call(
                &mut emu,
                WIN64_NTCREATETIMER2,
                &[HANDLE, id, 0, flags, ACCESS]
            ),
            expected
        );
        assert_eq!(emu.maps.read_qword(HANDLE), previous);
    }
}

#[test]
fn hot_patch_capability_query_preserves_the_input_and_returns_zero_bytes() {
    const BUFFER: u64 = 0x100200;
    const RETURN_LENGTH: u64 = BUFFER + 32;
    let mut emu = setup_emu64_syscall();
    for (address, length, version, flags, expected) in [
        (BUFFER, 8, 1, 0, STATUS_SUCCESS),
        (BUFFER, 16, 1, 0, STATUS_SUCCESS),
        (BUFFER, 4, 1, 0, STATUS_INFO_LENGTH_MISMATCH),
        (BUFFER, 8, 2, 0, STATUS_INVALID_PARAMETER),
        (BUFFER, 8, 1, 1, STATUS_INVALID_PARAMETER),
        (BUFFER + 1, 8, 1, 0, STATUS_DATATYPE_MISALIGNMENT),
        (0, 8, 1, 0, STATUS_ACCESS_VIOLATION),
    ] {
        emu.maps.write_dword(BUFFER, version);
        emu.maps.write_dword(BUFFER + 4, flags);
        emu.maps.write_qword(RETURN_LENGTH, u64::MAX);
        let status = native_test_call(
            &mut emu,
            WIN64_NTMANAGEHOTPATCH,
            &[
                MANAGE_HOT_PATCH_CHECK_ENABLED as u64,
                address,
                length,
                RETURN_LENGTH,
            ],
        );
        assert_eq!(status, expected);
        assert_eq!(emu.maps.read_dword(BUFFER), Some(version));
        assert_eq!(emu.maps.read_dword(BUFFER + 4), Some(flags));
        assert_eq!(
            emu.maps.read_qword(RETURN_LENGTH),
            Some((u32::MAX as u64) << 32)
        );
    }
    let status = native_test_call(
        &mut emu,
        WIN64_NTMANAGEHOTPATCH,
        &[MANAGE_HOT_PATCH_CHECK_ENABLED as u64, BUFFER, 8, 0],
    );
    assert_eq!(status, STATUS_ACCESS_VIOLATION);

    emu.maps.write_dword(RETURN_LENGTH, u32::MAX);
    native_test_call(
        &mut emu,
        WIN64_NTMANAGEHOTPATCH,
        &[u32::MAX as u64, BUFFER, 8, RETURN_LENGTH],
    );
    assert!(emu.api_policy.stopped());
    assert_eq!(emu.maps.read_dword(RETURN_LENGTH), Some(u32::MAX));
}

#[test]
fn hot_patch_lookup_reports_an_unpatched_image_on_both_windows_abis() {
    const BUFFER: u64 = 0x100000;
    const IMAGE: u64 = 0x400000;
    const RETURN_LENGTH: u64 = BUFFER + 64;
    const CANARY: u32 = 0x11223344;
    for (mut emu, width, size) in [(emu32(), 4, 24), (emu64(), 8, 48)] {
        emu.maps
            .create_map("query", BUFFER, PAGE_SIZE, Permission::READ_WRITE)
            .unwrap();
        emu.maps.register_allocation(Allocation::new(
            IMAGE,
            PAGE_SIZE,
            PAGE_READONLY,
            MemoryKind::Image,
            PageState::committed(PAGE_READONLY, Permission::READ),
        ));
        for export in ["NtManageHotPatch", "ZwManageHotPatch"] {
            for (base, expected) in [
                (IMAGE + 16, STATUS_SUCCESS),
                (BUFFER, STATUS_CONFLICTING_ADDRESSES),
                (0x700000, STATUS_MEMORY_NOT_ALLOCATED),
                (0, STATUS_INVALID_PARAMETER),
            ] {
                emu.maps.write_dword(BUFFER, 1);
                if width == 8 {
                    emu.maps.write_qword(BUFFER + width, NT_CURRENT_PROCESS);
                    emu.maps.write_qword(BUFFER + 2 * width, base);
                } else {
                    emu.maps
                        .write_dword(BUFFER + width, NT_CURRENT_PROCESS as u32);
                    emu.maps.write_dword(BUFFER + 2 * width, base as u32);
                }
                emu.maps.write_dword(BUFFER + 4 * width, CANARY);
                emu.maps.write_dword(BUFFER + 5 * width, CANARY);
                emu.maps.write_dword(BUFFER + size, CANARY);
                emu.maps.write_dword(RETURN_LENGTH, CANARY);
                let status = helpers::call_winapi_export(
                    &mut emu,
                    "ntdll.dll",
                    export,
                    &[
                        MANAGE_HOT_PATCH_QUERY_SINGLE_PATCH as u64,
                        BUFFER,
                        size,
                        RETURN_LENGTH,
                    ],
                );
                assert_eq!(status, expected, "{export} width={width} image={base:#x}");
                let ok = status == STATUS_SUCCESS;
                assert_eq!(
                    emu.maps.read_dword(RETURN_LENGTH),
                    Some(if ok { 48 } else { 0 })
                );
                assert_eq!(
                    emu.maps.read_dword(BUFFER + 4 * width),
                    Some(if ok { 0 } else { CANARY })
                );
                assert_eq!(
                    emu.maps.read_dword(BUFFER + 5 * width),
                    Some(if ok && width == 8 { 0 } else { CANARY })
                );
                assert_eq!(emu.maps.read_dword(BUFFER + size), Some(CANARY));
            }
        }
    }
}

#[test]
fn strict_loader_registers_a_disabled_provider_and_accepts_unregistered_events() {
    const BUFFER: u64 = 0x101200;
    const WRITTEN: u64 = BUFFER + 0x100;
    let mut emu = setup_emu64_syscall();
    emu.set_api_mode(ApiMode::Strict);
    let register = [ETW_REGISTER_GUIDS as u64, BUFFER, 160, BUFFER, 160, WRITTEN];
    emu.maps.write_bytes(BUFFER, &[0xaa; 161]);
    emu.maps.write_qword(WRITTEN, u64::MAX);
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTTRACECONTROL, &register),
        STATUS_SUCCESS
    );
    let handle = emu.maps.read_qword(BUFFER + 24).unwrap();
    assert_ne!(handle, 0);
    assert_eq!(emu.maps.read_qword(BUFFER + 112), Some(0));
    assert_eq!(emu.maps.read_byte(BUFFER + 160), Some(0xaa));
    assert_eq!(emu.maps.read_qword(WRITTEN), Some(0xffff_ffff_0000_00a0));

    emu.maps.write_qword(BUFFER, handle);
    let traits = [
        ETW_SET_PROVIDER_TRAITS as u64,
        BUFFER,
        24,
        BUFFER,
        120,
        WRITTEN,
    ];
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTTRACECONTROL, &traits),
        STATUS_SUCCESS
    );
    assert_eq!(emu.maps.read_dword(WRITTEN), Some(120));
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTCLOSE, &[handle]),
        STATUS_SUCCESS
    );
    emu.maps.write_qword(BUFFER, handle);
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTTRACECONTROL, &traits),
        STATUS_INVALID_HANDLE
    );
    for (pointer, status) in [(BUFFER, STATUS_SUCCESS), (0, STATUS_ACCESS_VIOLATION)] {
        assert_eq!(
            native_test_call(
                &mut emu,
                WIN64_NTTRACEEVENT,
                &[0, ETW_NT_FLAGS_TRACE_EVENT_NOREG as u64, 120, pointer]
            ),
            status
        );
    }
    assert_eq!(
        native_test_call(
            &mut emu,
            WIN64_NTSETINFORMATIONPROCESS,
            &[
                NT_CURRENT_PROCESS,
                PROCESS_SCHEDULER_SHARED_DATA as u64,
                BUFFER,
                8
            ]
        ),
        STATUS_NOT_SUPPORTED
    );
    assert!(!emu.api_policy.stopped());
}

#[test]
fn process_tls_swaps_the_vector_and_index_in_guest_memory() {
    const REQUEST: u64 = 0x101200;
    const OLD: u64 = 0x101400;
    const NEW: u64 = 0x101500;
    const ENTRY_FLAGS: u64 = REQUEST + 16;
    const DATA: u64 = REQUEST + 24;
    const TID: u64 = REQUEST + 32;
    const REPLACE_VECTOR: u32 = 1;
    const REPLACE_INDEX: u32 = 0;
    let mut emu = setup_emu64_syscall();
    emu.set_api_mode(ApiMode::Strict);
    let teb = emu.maps.map("teb", 0x1000, Permission::READ_WRITE);
    let slot = teb + TEB64::THREAD_LOCAL_STORAGE_POINTER_OFFSET;
    let call = |emu: &mut emu::Emu, length| {
        native_test_call(
            emu,
            WIN64_NTSETINFORMATIONPROCESS,
            &[
                NT_CURRENT_PROCESS,
                PROCESS_TLS_INFORMATION as u64,
                REQUEST,
                length,
            ],
        )
    };
    emu.maps.write_qword(slot, OLD);
    emu.maps.write_qword(OLD, OLD + 0x80);
    emu.maps.write_qword(NEW + 8, NEW + 0x80);
    emu.maps.write_dword(REQUEST + 4, REPLACE_VECTOR);
    emu.maps.write_dword(REQUEST + 8, 1);
    emu.maps.write_dword(REQUEST + 12, 1);
    emu.maps.write_qword(DATA, NEW);
    emu.maps.write_qword(REQUEST + 40, u64::MAX);
    assert_eq!(call(&mut emu, 39), STATUS_INFO_LENGTH_MISMATCH);
    assert_eq!(emu.maps.read_qword(slot), Some(OLD));
    assert_eq!(call(&mut emu, 40), STATUS_SUCCESS);
    assert_eq!(emu.maps.read_qword(slot), Some(NEW));
    assert_eq!(emu.maps.read_qword(DATA), Some(OLD));
    assert_eq!(emu.maps.read_dword(ENTRY_FLAGS), Some(2));
    assert_eq!(emu.maps.read_qword(TID), Some(emu.current_thread().id));
    assert_eq!(emu.maps.read_qword(NEW), Some(OLD + 0x80));
    assert_eq!(emu.maps.read_qword(NEW + 8), Some(NEW + 0x80));

    emu.maps.write_dword(REQUEST + 4, REPLACE_INDEX);
    emu.maps.write_dword(ENTRY_FLAGS, 0);
    emu.maps.write_qword(DATA, NEW + 0x90);
    assert_eq!(call(&mut emu, 40), STATUS_SUCCESS);
    assert_eq!(emu.maps.read_qword(DATA), Some(NEW + 0x80));
    assert_eq!(emu.maps.read_qword(NEW + 8), Some(NEW + 0x90));
    assert_eq!(emu.maps.read_qword(REQUEST + 40), Some(u64::MAX));
    emu.maps.write_dword(ENTRY_FLAGS, 0);
    emu.maps.write_dword(REQUEST + 12, u32::MAX);
    assert_eq!(call(&mut emu, 40), STATUS_ACCESS_VIOLATION);
    assert_eq!(emu.maps.read_dword(ENTRY_FLAGS), Some(0));
    assert_eq!(emu.maps.read_qword(NEW), Some(OLD + 0x80));
    assert!(!emu.api_policy.stopped());
}

#[test]
fn startup_token_queries_share_identity_and_preserve_short_outputs() {
    use crate::windows::constants::security::{
        CURRENT_THREAD_EFFECTIVE_TOKEN, TOKEN_STATISTICS, TOKEN_USER,
    };
    let mut emu = setup_emu64_syscall();
    let output = 0x101200;
    let returned = output + 0x100;
    let token = CURRENT_THREAD_EFFECTIVE_TOKEN as i64 as u64;
    let query = |emu: &mut emu::Emu, class, size| {
        native_test_call(
            emu,
            WIN64_NTQUERYINFORMATIONTOKEN,
            &[token, class, output, size, returned],
        )
    };
    emu.maps.write_qword(output, u64::MAX);
    assert_eq!(
        query(&mut emu, TOKEN_USER as u64, 0),
        STATUS_BUFFER_TOO_SMALL
    );
    assert_eq!(emu.maps.read_qword(output), Some(u64::MAX));
    let required = emu.maps.read_dword(returned).unwrap();
    assert_eq!(
        query(&mut emu, TOKEN_USER as u64, required as u64),
        STATUS_SUCCESS
    );
    assert_eq!(emu.maps.read_qword(output), Some(output + 16));
    emu.maps.write_byte(output + 56, 0xa5);
    assert_eq!(query(&mut emu, TOKEN_STATISTICS as u64, 56), STATUS_SUCCESS);
    assert_eq!(emu.maps.read_dword(returned), Some(56));
    assert_eq!(emu.maps.read_dword(output + 24), Some(1));
    assert_eq!(emu.maps.read_byte(output + 56), Some(0xa5));
}

#[test]
fn native_dll_identity_comparison_preserves_loader_entries() {
    use crate::windows::structures::{LdrDataTableEntry64, Peb64, PebLdrData64};

    let mut emu = setup_emu64_syscall();
    let peb = emu.maps.map("peb", PAGE_SIZE, Permission::READ_WRITE);
    let ldr = peb + 0x300;
    let head = ldr + PebLdrData64::IN_LOAD_ORDER_MODULE_LIST_OFFSET;
    let entry = peb + 0x500;
    emu.maps.write_qword(peb + Peb64::LDR_OFFSET, ldr);
    emu.maps.write_qword(head, entry);
    emu.maps.write_qword(entry, head);
    let saved = emu
        .maps
        .read_bytes(entry, LdrDataTableEntry64::size() as usize)
        .to_vec();
    let dll = emu
        .maps
        .map("example.tools.pe", PAGE_SIZE, Permission::READ);
    let view = emu.maps.map("file_view", PAGE_SIZE, Permission::READ);
    emu.section_view_dll
        .insert(view, "EXAMPLE.TOOLS.DLL".into());
    let unrelated = emu
        .maps
        .map("example.other.pe", PAGE_SIZE, Permission::READ);
    for (address, expected) in [
        (dll + 4, STATUS_SUCCESS),
        (peb, STATUS_NOT_SAME_DEVICE),
        (unrelated, STATUS_NOT_SAME_DEVICE),
        (dll, STATUS_SUCCESS),
    ] {
        assert_eq!(
            native_test_call(
                &mut emu,
                WIN64_NTAREMAPPEDFILESTHESAME,
                &[view + 16, address]
            ),
            expected
        );
        assert_eq!(
            emu.maps.read_bytes(entry, saved.len()).as_ref(),
            saved.as_slice()
        );
    }
}

#[test]
fn native_csr_session_maps_static_strings_and_a_separate_writable_client_view() {
    use crate::api::windows::common::sections;
    use crate::windows::peb::peb64;
    use crate::windows::structures::Peb64;

    let mut emu = setup_emu64_syscall();
    emu.set_api_mode(ApiMode::Strict);
    let peb = emu.maps.map("peb", PAGE_SIZE, Permission::READ_WRITE);
    const SERVER_BASE: u64 = 0x30000000;
    emu.maps.write_dword(peb + Peb64::SESSION_ID_OFFSET, 3);
    emu.maps.write_qword(
        peb + Peb64::READ_ONLY_SHARED_MEMORY_BASE_OFFSET,
        SERVER_BASE,
    );
    emu.maps.write_qword(
        peb + Peb64::READ_ONLY_STATIC_SERVER_DATA_OFFSET,
        SERVER_BASE + Peb64::CSR_STATIC_TABLE_OFFSET,
    );
    peb64::ensure_peb_nls_tables(&mut emu);

    let out = 0x102000;
    let object = out + 64;
    let name = out + 128;
    let quality = out + 256;
    let client = out + 288;
    let server = out + 352;
    let data = out + 400;
    let length = out + 464;
    let view_base = out + 480;
    let view_size = out + 488;
    emu.maps.write_dword(object, 48);
    emu.maps.write_qword(object + 16, name);
    for (session, status) in [(1, STATUS_OBJECT_NAME_NOT_FOUND), (3, STATUS_SUCCESS)] {
        native_name(
            &mut emu,
            name,
            &format!(r"\Sessions\{session}\Windows\SharedSection"),
        );
        assert_eq!(
            native_test_call(
                &mut emu,
                WIN64_NTOPENSECTION,
                &[out, SECTION_MAP_READ as u64, object]
            ),
            status
        );
    }
    let shared = emu.maps.read_qword(out).unwrap();
    let (client_section, _) =
        sections::create(&mut emu, None, PAGE_SIZE, PAGE_READWRITE, String::new()).unwrap();
    native_name(&mut emu, name, r"\Sessions\3\Windows\ApiPort");
    // The Win11 CSR client leaves the QoS Length field zero.
    emu.maps.write_dword(client, 48);
    emu.maps.write_qword(client + 8, client_section);
    emu.maps.write_dword(length, 48);
    emu.maps.write_dword(data + 32, 1);
    let connect = [out, name, quality, client, server, 0, data, length];
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTCONNECTPORT, &connect),
        STATUS_INVALID_PARAMETER
    );
    assert_eq!(emu.maps.read_qword(client + 32), Some(0));
    assert_eq!(emu.maps.read_qword(out), Some(shared));
    emu.maps.write_dword(server, 24);
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTCONNECTPORT, &connect),
        STATUS_SUCCESS
    );
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTCLOSE, &[client_section]),
        STATUS_SUCCESS
    );
    let heap = emu.maps.read_qword(client + 32).unwrap();
    assert!(emu.maps.write_qword(heap, 17));

    let mut map = [
        shared,
        NT_CURRENT_PROCESS,
        view_base,
        0,
        0,
        0,
        view_size,
        1,
        0,
        PAGE_READWRITE as u64,
    ];
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTMAPVIEWOFSECTION, &map),
        STATUS_SECTION_PROTECTION
    );
    map[8] = (MEM_TOP_DOWN | SEC_NO_CHANGE) as u64;
    map[9] = PAGE_READONLY as u64;
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTMAPVIEWOFSECTION, &map),
        STATUS_SUCCESS
    );
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTCLOSE, &[shared]),
        STATUS_SUCCESS
    );
    let local = emu.maps.read_qword(view_base).unwrap();
    let origin = emu.maps.read_qword(data).unwrap();
    assert_eq!(
        local,
        (ALLOC64_MAX - 2 * PAGE_SIZE) & !(ALLOCATION_GRANULARITY - 1)
    );
    assert!(
        emu.maps
            .protect_memory(local, PAGE_SIZE, PAGE_NOACCESS, Permission::NONE)
            .is_err()
    );
    let old = out + 512;
    emu.maps.write_dword(old, u32::MAX);
    assert_eq!(
        helpers::call_winapi_export(
            &mut emu,
            "kernel32.dll",
            "VirtualProtect",
            &[local, PAGE_SIZE, PAGE_NOACCESS as u64, old]
        ),
        0
    );
    assert_eq!(emu.maps.read_dword(old), Some(u32::MAX));
    assert_ne!(local, origin);
    assert_ne!(local, heap);
    assert!(!emu.maps.validate_write_range(local, 1));
    let table = local + (emu.maps.read_qword(data + 8).unwrap() - origin);
    let base_data = local + (emu.maps.read_qword(table + 8).unwrap() - origin);
    for (offset, expected) in [(0, r"C:\Windows"), (16, r"C:\Windows\System32")] {
        let descriptor = UnicodeString64::load(base_data + offset, &emu.maps);
        assert_eq!(
            emu.maps
                .read_wide_string(local + (descriptor.buffer - origin)),
            expected
        );
    }
    assert_eq!(
        native_test_call(
            &mut emu,
            WIN64_NTUNMAPVIEWOFSECTION,
            &[NT_CURRENT_PROCESS, local]
        ),
        STATUS_SUCCESS
    );
    assert!(!emu.maps.is_mapped(local));
    assert_eq!(emu.maps.read_qword(heap), Some(17));
}

#[test]
fn native_duplicate_console_preserves_access_and_copies_attributes_independently() {
    use crate::emu::object_handle::NativeObject;
    let mut emu = setup_emu64_syscall();
    emu.set_api_mode(ApiMode::Strict);
    let out = 0x102000;
    let source = emu
        .handle_management
        .insert_native_object(NativeObject::ConsoleDevice {
            access: 3,
            attributes: OBJ_INHERIT,
        });
    for (options, requested, expected) in [
        (
            DUPLICATE_SAME_ACCESS | DUPLICATE_SAME_ATTRIBUTES,
            0,
            OBJ_INHERIT,
        ),
        (DUPLICATE_SAME_ACCESS, 0, 0),
        (DUPLICATE_SAME_ACCESS, OBJ_INHERIT, OBJ_INHERIT),
    ] {
        let args = [
            NT_CURRENT_PROCESS,
            source,
            NT_CURRENT_PROCESS,
            out,
            0,
            requested as u64,
            options as u64,
        ];
        assert_eq!(
            native_test_call(&mut emu, WIN64_NTDUPLICATEOBJECT, &args),
            STATUS_SUCCESS
        );
        let copy = emu.maps.read_qword(out).unwrap();
        assert_ne!(source, copy);
        assert_eq!(
            emu.handle_management.native_object(copy),
            Some(NativeObject::ConsoleDevice {
                access: 3,
                attributes: expected
            })
        );
        assert_eq!(
            native_test_call(&mut emu, WIN64_NTCLOSE, &[copy]),
            STATUS_SUCCESS
        );
        assert!(emu.handle_management.is_console_device(source));
    }
}

#[test]
fn console_host_association_and_device_information_are_visible_to_the_guest() {
    use crate::emu::object_handle::NativeObject;
    use crate::windows::constants::files::{
        FILE_DEVICE_ALLOW_APPCONTAINER_TRAVERSAL, FILE_DEVICE_CONSOLE, FILE_FS_DEVICE_INFORMATION,
    };
    let mut emu = setup_emu64_syscall();
    emu.set_api_mode(ApiMode::Strict);
    let out = 0x102000;
    let io = out + 32;
    let set = [NT_CURRENT_PROCESS, PROCESS_CONSOLE_HOST_PROCESS, out, 8];
    emu.maps.write_qword(out, 1);
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTSETINFORMATIONPROCESS, &set),
        STATUS_SUCCESS
    );
    emu.maps.write_qword(out, 0);
    assert_eq!(
        native_test_call(
            &mut emu,
            WIN64_NTQUERYINFORMATIONPROCESS,
            &[NT_CURRENT_PROCESS, PROCESS_CONSOLE_HOST_PROCESS, out, 8, io]
        ),
        STATUS_SUCCESS
    );
    assert_eq!(emu.maps.read_qword(out), Some(1));
    assert_eq!(emu.maps.read_dword(io), Some(8));
    let device = emu
        .handle_management
        .insert_native_object(NativeObject::ConsoleDevice {
            access: 1,
            attributes: 0,
        });
    for (length, status) in [(7, STATUS_INFO_LENGTH_MISMATCH), (8, STATUS_SUCCESS)] {
        assert_eq!(
            native_test_call(
                &mut emu,
                WIN64_NTQUERYVOLUMEINFORMATIONFILE,
                &[device, io, out, length, FILE_FS_DEVICE_INFORMATION as u64]
            ),
            status
        );
    }
    assert_eq!(emu.maps.read_dword(out), Some(FILE_DEVICE_CONSOLE));
    assert_eq!(
        emu.maps.read_dword(out + 4),
        Some(FILE_DEVICE_ALLOW_APPCONTAINER_TRAVERSAL)
    );
    assert_eq!(emu.maps.read_qword(io + 8), Some(8));
}

#[test]
fn csr_user_connect_returns_read_only_server_info_only_to_the_ports_capture_buffer() {
    use crate::api::windows::common::sections;

    let mut emu = setup_emu64_syscall();
    emu.set_api_mode(ApiMode::Strict);
    const SCRATCH: u64 = 0x102000;
    const MESSAGE_SIZE: u64 = 0x58;
    const USERCONNECT_SIZE: u32 = 0x248;
    let name = SCRATCH + 16;
    let quality = SCRATCH + 192;
    let view = SCRATCH + 224;
    let send = SCRATCH + 288;
    let receive = SCRATCH + 400;
    let length = SCRATCH + 512;
    let (section, _) =
        sections::create(&mut emu, None, PAGE_SIZE, PAGE_READWRITE, String::new()).unwrap();
    native_name(&mut emu, name, r"\Windows\ApiPort");
    emu.maps.write_dword(quality, 12);
    emu.maps.write_dword(view, 48);
    emu.maps.write_qword(view + 8, section);
    assert_eq!(
        native_test_call(
            &mut emu,
            WIN64_NTCONNECTPORT,
            &[SCRATCH, name, quality, view, 0, 0, 0, 0]
        ),
        STATUS_SUCCESS
    );
    let port = emu.maps.read_qword(SCRATCH).unwrap();
    let capture = emu.maps.read_qword(view + 32).unwrap();
    let data = capture + 40;
    emu.maps.write_dword(capture, 40 + USERCONNECT_SIZE);
    emu.maps.write_dword(capture + 16, 1);
    emu.maps.write_word(send, (MESSAGE_SIZE - 0x28) as u16);
    emu.maps.write_word(send + 2, MESSAGE_SIZE as u16);
    emu.maps.write_qword(send + 0x28, capture);
    emu.maps.write_dword(send + 0x40, 3);
    emu.maps.write_qword(send + 0x48, data);
    emu.maps.write_dword(send + 0x50, USERCONNECT_SIZE);
    emu.maps.write_qword(length, MESSAGE_SIZE);
    emu.maps.write_qword(data + 8, u64::MAX);
    let args = [port, 0x20000, send, 0, receive, length, 0, 0];
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTALPCSENDWAITRECEIVEPORT, &args),
        STATUS_INVALID_PARAMETER
    );
    assert_eq!(emu.maps.read_qword(data + 8), Some(u64::MAX));
    emu.maps.write_qword(capture + 32, 0x48);
    let (other, _) = sections::map(&mut emu, section, 0, 0, 0, PAGE_READWRITE).unwrap();
    emu.maps.write_qword(send + 0x28, other);
    emu.maps.write_qword(send + 0x48, other + 40);
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTALPCSENDWAITRECEIVEPORT, &args),
        STATUS_INVALID_PARAMETER
    );
    emu.maps.write_qword(send + 0x28, capture);
    emu.maps.write_qword(send + 0x48, SCRATCH);
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTALPCSENDWAITRECEIVEPORT, &args),
        STATUS_INVALID_PARAMETER
    );
    assert_eq!(emu.maps.read_word(receive + 4), Some(0));
    emu.maps.write_qword(send + 0x48, data);
    assert_eq!(
        native_test_call(&mut emu, WIN64_NTALPCSENDWAITRECEIVEPORT, &args),
        STATUS_SUCCESS
    );
    let server = emu.maps.read_qword(data + 8).unwrap();
    assert_eq!(emu.maps.read_dword(server), Some(0));
    assert!(!emu.maps.validate_write_range(server, 1));
    assert_eq!(emu.maps.read_word(receive + 4), Some(2));
}
