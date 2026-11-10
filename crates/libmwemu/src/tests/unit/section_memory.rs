use crate::winapi::winapi64;
use crate::windows::constants::errors::*;
use crate::windows::constants::memory::*;
use crate::windows::constants::process::*;

use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::MemoryKind;
use crate::serialization::Serialization;
use crate::tests::helpers::{self, call_winapi32, call_winapi64};
use crate::winapi::common::memory::{self, Call};
use crate::winapi::common::{sections, virtual_protect};
use crate::{emu32, emu64};

const DATA: u64 = 0x3000_0000;
const FIRST: u64 = 0x4000_0000;
const SECOND: u64 = 0x5000_0000;

fn prepare(is64: bool) -> Emu {
    let mut emu = if is64 { emu64() } else { emu32() };
    emu.cfg.console_enabled = false;
    if is64 {
        emu.init_stack64();
    } else {
        emu.init_stack32();
    }
    emu.maps
        .create_map("data", DATA, 0x1000, Permission::READ_WRITE)
        .unwrap();
    emu
}

fn call(emu: &mut Emu, function: fn(&mut Emu), args: &[u64]) -> u64 {
    let stack = emu.regs().rsp;
    let result = if emu.cfg.arch.is_64bits() {
        call_winapi64(emu, function, args)
    } else {
        call_winapi32(
            emu,
            function,
            &args.iter().map(|&a| a as u32).collect::<Vec<_>>(),
        ) as u64
    };
    assert_eq!(emu.regs().rsp, stack);
    result
}

#[test]
fn views_share_bytes_but_keep_independent_protection_and_lifetimes() {
    for is64 in [false, true] {
        let mut emu = prepare(is64);
        let handle = call(
            &mut emu,
            |emu| sections::create_file_mapping(emu, false),
            &[u32::MAX as u64, 0, PAGE_READWRITE as u64, 0, 0x20000, 0],
        );
        assert_ne!(handle, 0);
        let first = call(
            &mut emu,
            |emu| sections::map_view(emu, true),
            &[handle, 2, 0, 0, 0x20000, FIRST],
        );
        let second = call(
            &mut emu,
            |emu| sections::map_view(emu, true),
            &[handle, 4, 0, 0x10000, 0x10000, SECOND],
        );
        assert_eq!(first, FIRST);
        assert_eq!(second, SECOND);
        emu.maps.write_dword(FIRST + 0x10000, 0x12345678);
        assert_eq!(emu.maps.read_dword(SECOND), Some(0x12345678));
        assert!(!emu.maps.write_byte(SECOND, 0));
        emu.maps.write_dword(DATA, 0xabcdef);
        assert!(!virtual_protect::protect_winapi(
            &mut emu,
            SECOND,
            1,
            PAGE_READWRITE,
            DATA
        ));
        assert_eq!(emu.maps.read_dword(DATA), Some(0xabcdef));
        assert!(memory::allocate(&mut emu, SECOND, 1, 0x1000, PAGE_READWRITE).is_err());
        assert_eq!(emu.maps.query_memory(SECOND).kind, MemoryKind::Mapped);
        assert!(sections::close(&mut emu, handle));
        assert!(sections::map(&mut emu, handle, 0, 0, 0, PAGE_READONLY).is_err());
        assert!(sections::unmap(&mut emu, FIRST + 32).is_ok());
        assert_eq!(emu.maps.read_dword(SECOND), Some(0x12345678));
        assert_eq!(emu.maps.shared_stores.len(), 1);
        assert!(sections::unmap(&mut emu, SECOND + 32).is_ok());
        assert!(emu.maps.shared_stores.is_empty());
    }
}

#[test]
fn nt_section_calls_validate_outputs_and_preserve_handles_in_snapshots() {
    for is64 in [false, true] {
        let mut emu = prepare(is64);
        emu.maps.write_qword(DATA + 32, 0x20000);
        let create = |emu: &mut Emu| sections::nt_create(emu, Call::Api);
        let args = [
            DATA,
            0xf001f,
            0,
            DATA + 32,
            PAGE_READWRITE as u64,
            SEC_COMMIT as u64,
            0,
        ];
        assert_eq!(call(&mut emu, create, &args), 0);
        let handle = memory::read_pointer(&emu, DATA).unwrap();
        memory::write_pointer(&mut emu, DATA + 8, 0);
        memory::write_pointer(&mut emu, DATA + 16, 0x20000);
        let mut map_args = [
            handle,
            u32::MAX as u64,
            DATA + 8,
            0,
            0,
            0,
            DATA + 16,
            1,
            0,
            PAGE_READWRITE as u64,
        ];
        let map = |emu: &mut Emu| sections::nt_map(emu, Call::Api);
        let count = emu.maps.maps.len();
        map_args[6] = 1;
        assert_ne!(call(&mut emu, map, &map_args), 0);
        assert_eq!(emu.maps.maps.len(), count);
        map_args[6] = DATA + 16;
        assert_eq!(call(&mut emu, map, &map_args), 0);
        let first = memory::read_pointer(&emu, DATA + 8).unwrap();
        emu.maps.write_dword(first, 0x12345678);
        let mut restored = Serialization::deserialize(&Serialization::serialize(&emu));
        let (second, _) =
            sections::map(&mut restored, handle, 0, 0, 0x20000, PAGE_READWRITE).unwrap();
        assert_ne!(first, second);
        restored.maps.write_dword(second, 0x87654321);
        assert_eq!(restored.maps.read_dword(first), Some(0x87654321));
        assert_eq!(emu.maps.read_dword(first), Some(0x12345678));
        assert!(sections::close(&mut restored, handle));
        assert!(sections::unmap(&mut restored, first).is_ok());
        assert!(sections::unmap(&mut restored, second).is_ok());
        assert!(restored.maps.shared_stores.is_empty());
    }
}

#[test]
fn named_sections_remain_openable_while_a_view_is_alive() {
    for is64 in [false, true] {
        let mut emu = prepare(is64);
        emu.maps.write_string(DATA, "shared-section");
        let (original, _) = sections::create(
            &mut emu,
            None,
            0x1000,
            PAGE_READWRITE,
            "shared-section".into(),
        )
        .unwrap();
        let (view, _) = sections::map(&mut emu, original, 0, 0, 0, PAGE_READWRITE).unwrap();
        emu.maps.write_dword(view, 0x12345678);
        assert!(sections::close(&mut emu, original));
        let mut emu = Serialization::deserialize(&Serialization::serialize(&emu));
        let opened = call(
            &mut emu,
            |emu| sections::open_file_mapping(emu, false),
            &[4, 0, DATA],
        );
        assert_ne!(opened, 0);
        assert!(sections::map(&mut emu, opened, 0, 0, 0, PAGE_READWRITE).is_err());
        let (alias, _) = sections::map(&mut emu, opened, 0, 0, 0, PAGE_READONLY).unwrap();
        assert_eq!(emu.maps.read_dword(alias), Some(0x12345678));
        assert!(sections::close(&mut emu, opened));
        assert!(sections::unmap(&mut emu, view).is_ok());
        assert!(sections::unmap(&mut emu, alias).is_ok());
        assert_eq!(
            call(
                &mut emu,
                |emu| sections::open_file_mapping(emu, false),
                &[4, 0, DATA]
            ),
            0
        );
    }
}

#[test]
fn section_lifecycle_64() {
    helpers::setup();
    let mut emu = emu64();

    let handle_ptr = 0x100000u64;
    let base_ptr = 0x100010u64;
    let view_sz_ptr = 0x100020u64;
    let max_sz_ptr = 0x100030u64;
    emu.maps
        .create_map("section_io", 0x100000, 0x1000, Permission::READ_WRITE);

    emu.maps.write_qword(max_sz_ptr, 0x10000);
    emu.maps.write_qword(view_sz_ptr, 0x10000);
    emu.maps.write_qword(base_ptr, 0);

    // NtCreateSection(SectionHandle*, Access, ObjAttr, MaxSize*, Prot, Alloc, File)
    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtCreateSection,
        &[
            handle_ptr,
            SECTION_ALL_ACCESS as u64,
            0,
            max_sz_ptr,
            PAGE_READWRITE as u64,
            SEC_COMMIT as u64,
            0,
        ],
    );
    assert_eq!(
        status, STATUS_SUCCESS,
        "NtCreateSection must return STATUS_SUCCESS"
    );
    let section_handle = emu.maps.read_qword(handle_ptr).unwrap();
    assert_ne!(section_handle, 0, "section handle must be non-zero");

    // NtMapViewOfSection(Section, Process, BaseAddr*, ZeroBits, CommitSz, Offset, ViewSz*, Inherit, AllocType, Protect)
    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtMapViewOfSection,
        &[
            section_handle,
            NT_CURRENT_PROCESS,
            base_ptr,
            0,
            0,
            0,
            view_sz_ptr,
            VIEW_SHARE as u64,
            0,
            PAGE_READWRITE as u64,
        ],
    );
    assert_eq!(
        status, STATUS_SUCCESS,
        "NtMapViewOfSection must return STATUS_SUCCESS"
    );
    let base = emu.maps.read_qword(base_ptr).unwrap();
    assert_ne!(base, 0, "mapped base must be non-zero");
    assert!(emu.maps.is_mapped(base), "mapped memory must be accessible");

    emu.maps.write_dword(base, 0xCAFEBABE);
    assert_eq!(emu.maps.read_dword(base).unwrap(), 0xCAFEBABE);

    // NtUnmapViewOfSection(Process, BaseAddress)
    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtUnmapViewOfSection,
        &[NT_CURRENT_PROCESS, base],
    );
    assert_eq!(
        status, STATUS_SUCCESS,
        "NtUnmapViewOfSection must return STATUS_SUCCESS"
    );
}

#[test]
fn nt_section_lifecycle() {
    helpers::setup();
    let mut emu = emu64();

    let handle_ptr = 0x100000u64;
    let base_ptr = 0x100010u64;
    let max_size_ptr = 0x100020u64;
    let view_size_ptr = 0x100030u64;
    emu.maps
        .create_map("section_ptrs", handle_ptr, 0x1000, Permission::READ_WRITE);

    // Write max size
    emu.maps.write_qword(max_size_ptr, 0x10000);

    // NtCreateSection(handle_ptr, SECTION_ALL_ACCESS, NULL, max_size_ptr, PAGE_READWRITE, SEC_COMMIT, NULL)
    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtCreateSection,
        &[
            handle_ptr,
            SECTION_ALL_ACCESS as u64,
            0,
            max_size_ptr,
            PAGE_READWRITE as u64,
            SEC_COMMIT as u64,
            0,
        ],
    );
    assert_eq!(
        status, STATUS_SUCCESS,
        "NtCreateSection should return STATUS_SUCCESS"
    );

    let section_handle = emu.maps.read_qword(handle_ptr).unwrap();
    assert_ne!(section_handle, 0, "section handle should be non-zero");

    // NtMapViewOfSection(section_handle, process, base_ptr, 0, 0, NULL, view_size_ptr, ...)
    emu.maps.write_qword(view_size_ptr, 0x10000);
    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtMapViewOfSection,
        &[
            section_handle,
            NT_CURRENT_PROCESS,
            base_ptr,
            0,
            0,
            0,
            view_size_ptr,
            VIEW_SHARE as u64,
            0,
            PAGE_READWRITE as u64,
        ],
    );
    assert_eq!(
        status, STATUS_SUCCESS,
        "NtMapViewOfSection should return STATUS_SUCCESS"
    );

    let mapped_base = emu.maps.read_qword(base_ptr).unwrap();
    assert_ne!(mapped_base, 0, "mapped base should be non-zero");
    assert!(
        emu.maps.is_mapped(mapped_base),
        "mapped region should be accessible"
    );

    emu.maps.write_dword(mapped_base, 0xCAFEBABE);
    assert_eq!(emu.maps.read_dword(mapped_base).unwrap(), 0xCAFEBABE);

    // NtUnmapViewOfSection
    let status = helpers::call_winapi64(
        &mut emu,
        winapi64::ntdll::NtUnmapViewOfSection,
        &[NT_CURRENT_PROCESS, mapped_base],
    );
    assert_eq!(
        status, STATUS_SUCCESS,
        "NtUnmapViewOfSection should return STATUS_SUCCESS"
    );
    assert!(!emu.maps.is_mapped(mapped_base));
}
