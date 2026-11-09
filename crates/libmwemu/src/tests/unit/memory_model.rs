use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::{MemoryError, PAGE_SIZE};
use crate::serialization::Serialization;
use crate::tests::helpers::{call_winapi32, call_winapi64};
use crate::winapi::common::memory::{self, Call};
use crate::{emu32, emu64};
use itertools::iproduct;
use std::{env, fs};

const BASE: u64 = 0x3000_0000;
const OUTPUT: u64 = BASE + 0x10000;
const RW: u32 = 4;
const RO: u32 = 2;
const RESERVE: u32 = 0x2000;
const COMMIT: u32 = 0x1000;
const DECOMMIT: u32 = 0x4000;
const RELEASE: u32 = 0x8000;

fn prepare(is64: bool) -> Emu {
    let mut emu = if is64 { emu64() } else { emu32() };
    emu.cfg.console_enabled = false;
    if is64 {
        emu.init_stack64();
    } else {
        emu.init_stack32();
    }
    emu.maps
        .create_map("output", OUTPUT, PAGE_SIZE, Permission::READ_WRITE)
        .unwrap();
    emu
}

fn call(emu: &mut Emu, function: fn(&mut Emu), args: &[u64]) -> u64 {
    let stack = emu.regs().rsp;
    let result = if emu.cfg.arch.is_64bits() {
        call_winapi64(emu, function, args)
    } else {
        let args: Vec<_> = args.iter().map(|&value| value as u32).collect();
        call_winapi32(emu, function, &args) as u64
    };
    assert_eq!(emu.regs().rsp, stack);
    result
}

fn allocate(emu: &mut Emu) {
    memory::virtual_alloc(emu, false, false);
}

fn free(emu: &mut Emu) {
    memory::virtual_free(emu, false);
}

fn query(emu: &mut Emu) {
    memory::virtual_query(emu, false);
}

fn nt_allocate(emu: &mut Emu) {
    memory::nt_allocate(emu, Call::Api, false);
}

#[test]
fn reserve_commit_decommit_release_preserves_identity_and_resets_bytes() {
    for is64 in [false, true] {
        let mut emu = prepare(is64);
        assert_eq!(
            call(
                &mut emu,
                allocate,
                &[BASE, 3 * PAGE_SIZE, RESERVE as u64, RW as u64]
            ),
            BASE
        );
        assert!(!emu.maps.write_byte(BASE, 1));
        assert!(!emu.maps.query_memory(BASE).state.unwrap().committed);
        assert_eq!(
            call(
                &mut emu,
                allocate,
                &[BASE + PAGE_SIZE + 3, 2, COMMIT as u64, RW as u64]
            ),
            BASE + PAGE_SIZE
        );
        assert_eq!(emu.maps.read_dword(BASE + PAGE_SIZE), Some(0));
        assert!(emu.maps.write_dword(BASE + PAGE_SIZE, 0x12345678));
        assert_eq!(
            call(
                &mut emu,
                allocate,
                &[BASE + PAGE_SIZE, PAGE_SIZE, COMMIT as u64, RO as u64]
            ),
            BASE + PAGE_SIZE
        );
        assert_eq!(emu.maps.read_dword(BASE + PAGE_SIZE), Some(0x12345678));
        assert!(!emu.maps.write_byte(BASE + PAGE_SIZE, 1));
        assert_eq!(
            emu.maps.query_memory(BASE + PAGE_SIZE).allocation_base,
            BASE
        );
        assert_eq!(
            call(
                &mut emu,
                free,
                &[BASE + PAGE_SIZE, PAGE_SIZE, DECOMMIT as u64]
            ),
            1
        );
        assert_eq!(emu.maps.read_byte(BASE + PAGE_SIZE), None);
        assert_eq!(
            call(
                &mut emu,
                allocate,
                &[BASE + PAGE_SIZE, PAGE_SIZE, COMMIT as u64, RW as u64]
            ),
            BASE + PAGE_SIZE
        );
        assert_eq!(emu.maps.read_dword(BASE + PAGE_SIZE), Some(0));
        assert_eq!(
            call(&mut emu, free, &[BASE + PAGE_SIZE, 0, RELEASE as u64]),
            0
        );
        assert_eq!(call(&mut emu, free, &[BASE, PAGE_SIZE, RELEASE as u64]), 0);
        assert_eq!(call(&mut emu, free, &[BASE, 0, RELEASE as u64]), 1);
        assert!(!emu.maps.is_mapped(BASE));
        assert!(emu.maps.query_memory(BASE).state.is_none());
    }
}

#[test]
fn nt_allocate_rounds_pointer_outputs_and_cleans_x86_stack_on_error() {
    for (is64, valid) in iproduct!([false, true], [false, true]) {
        let mut emu = prepare(is64);
        emu.maps.write_qword(OUTPUT, 0xffff_ffff_0000_0000);
        emu.maps.write_qword(OUTPUT + 8, 0xffff_ffff_0000_0000);
        memory::write_pointer(&mut emu, OUTPUT, BASE + 0x123);
        memory::write_pointer(&mut emu, OUTPUT + 8, 0x1000);
        let result = call(
            &mut emu,
            nt_allocate,
            &[
                u64::MAX,
                OUTPUT,
                0,
                OUTPUT + 8,
                if valid { 0x3000 } else { 0 },
                RW as u64,
            ],
        );
        assert_eq!(result, if valid { 0 } else { 0xc000000d });
        assert_eq!(
            memory::read_pointer(&emu, OUTPUT),
            Some(if valid { BASE } else { BASE + 0x123 })
        );
        assert_eq!(
            memory::read_pointer(&emu, OUTPUT + 8),
            Some(if valid { 2 * PAGE_SIZE } else { PAGE_SIZE })
        );
        if !is64 {
            assert_eq!(emu.maps.read_dword(OUTPUT + 4), Some(u32::MAX));
            assert_eq!(emu.maps.read_dword(OUTPUT + 12), Some(u32::MAX));
        }
    }
}

#[test]
fn query_reports_page_state_in_the_guest_layout() {
    for is64 in [false, true] {
        let mut emu = prepare(is64);
        emu.maps
            .reserve_private(BASE, 3 * PAGE_SIZE, RW, Permission::READ_WRITE, false)
            .unwrap();
        emu.maps
            .commit_memory(BASE + PAGE_SIZE, PAGE_SIZE, RO, Permission::READ)
            .unwrap();
        let length = memory::query_size(&emu);
        for (offset, state, protection) in [
            (0, RESERVE, 0),
            (PAGE_SIZE, COMMIT, RO),
            (2 * PAGE_SIZE, RESERVE, 0),
        ] {
            emu.maps.fill_range(OUTPUT, 0xcc, 64);
            assert_eq!(
                call(&mut emu, query, &[BASE + offset + 7, OUTPUT, length]),
                length
            );
            assert_eq!(memory::read_pointer(&emu, OUTPUT), Some(BASE + offset));
            assert_eq!(
                memory::read_pointer(&emu, OUTPUT + memory::pointer_size(&emu)),
                Some(BASE)
            );
            let (size_offset, state_offset) = if is64 { (24, 32) } else { (12, 16) };
            assert_eq!(
                memory::read_pointer(&emu, OUTPUT + size_offset),
                Some(PAGE_SIZE)
            );
            assert_eq!(emu.maps.read_dword(OUTPUT + state_offset), Some(state));
            assert_eq!(
                emu.maps.read_dword(OUTPUT + state_offset + 4),
                Some(protection)
            );
            assert_eq!(emu.maps.read_byte(OUTPUT + length), Some(0xcc));
        }
    }
}

#[test]
fn protection_rejects_reserved_pages_and_cross_allocation_ranges_atomically() {
    let mut emu = prepare(false);
    emu.maps
        .reserve_private(BASE, 2 * PAGE_SIZE, RW, Permission::READ_WRITE, false)
        .unwrap();
    emu.maps
        .commit_memory(BASE, PAGE_SIZE, RW, Permission::READ_WRITE)
        .unwrap();
    emu.maps
        .reserve_private(
            BASE + 2 * PAGE_SIZE,
            PAGE_SIZE,
            RW,
            Permission::READ_WRITE,
            true,
        )
        .unwrap();
    assert_eq!(
        emu.maps
            .protect_memory(BASE, 2 * PAGE_SIZE, RO, Permission::READ),
        Err(MemoryError::NotCommitted)
    );
    assert_eq!(
        emu.maps
            .protect_memory(BASE, 3 * PAGE_SIZE, RO, Permission::READ),
        Err(MemoryError::Conflict)
    );
    assert!(emu.maps.write_byte(BASE, 1));
    assert!(emu.maps.write_byte(BASE + 2 * PAGE_SIZE, 1));
}

#[test]
fn map_overlap_and_oversized_requests_leave_existing_memory_intact() {
    let mut emu = prepare(false);
    emu.maps
        .create_map("existing", BASE, PAGE_SIZE, Permission::READ_WRITE)
        .unwrap();
    for (base, size) in [
        (BASE - PAGE_SIZE, 2 * PAGE_SIZE),
        (BASE - 1, 2),
        (BASE, PAGE_SIZE),
        (u64::MAX - 1, 4),
    ] {
        assert!(
            emu.maps
                .create_map("overlap", base, size, Permission::READ_WRITE)
                .is_err()
        );
    }
    assert!(emu.maps.alloc(emu.maps.max_alloc_size + 1).is_none());
    assert_eq!(emu.maps.get_mem("existing").size(), PAGE_SIZE as usize);
}

#[test]
fn native_snapshot_retains_reserved_pages_and_allocation_identity() {
    let mut emu = prepare(false);
    emu.maps
        .reserve_private(BASE, 3 * PAGE_SIZE, RW, Permission::READ_WRITE, false)
        .unwrap();
    emu.maps
        .commit_memory(BASE + PAGE_SIZE, PAGE_SIZE, RW, Permission::READ_WRITE)
        .unwrap();
    emu.maps.write_dword(BASE + PAGE_SIZE, 0x12345678);
    let mut restored = Serialization::deserialize(&Serialization::serialize(&emu));
    assert!(!restored.maps.query_memory(BASE).state.unwrap().committed);
    assert_eq!(
        restored.maps.query_memory(BASE + PAGE_SIZE).allocation_base,
        BASE
    );
    assert_eq!(restored.maps.read_dword(BASE + PAGE_SIZE), Some(0x12345678));
    assert!(restored.maps.release_memory(BASE).is_ok());
    assert!(!restored.maps.is_mapped(BASE + PAGE_SIZE));
}

#[test]
fn top_down_search_uses_complete_free_ranges() {
    let mut emu = prepare(false);
    emu.maps
        .create_map("last", BASE + 0x3000, 0x1000, Permission::READ_WRITE)
        .unwrap();
    assert_eq!(
        emu.maps
            .find_free_range_top_down(0x2000, 0x1000, BASE, BASE + 0x5000),
        Some(BASE + 0x1000)
    );
    assert_eq!(
        emu.maps
            .find_free_range_top_down(0x4000, 0x1000, BASE, BASE + 0x5000),
        None
    );
    let (high, _) = memory::allocate(&mut emu, 0, 0x2000, RESERVE | COMMIT | 0x100000, RW).unwrap();
    let (low, _) = memory::allocate(&mut emu, 0, 0x2000, RESERVE | COMMIT, RW).unwrap();
    assert!(high > low);
}

#[test]
fn minidump_restores_reservation_boundaries_and_page_state() {
    let mut emu = emu32();
    emu.maps
        .reserve_private(BASE, 3 * PAGE_SIZE, RW, Permission::READ_WRITE, false)
        .unwrap();
    emu.maps
        .commit_memory(BASE + PAGE_SIZE, PAGE_SIZE, RW, Permission::READ_WRITE)
        .unwrap();
    emu.maps.write_dword(BASE + PAGE_SIZE, 0x12345678);
    let dir = env::temp_dir().join("mwemu_reservation_state");
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("state.mdmp");
    Serialization::dump_to_minidump(&emu, path.to_str().unwrap()).unwrap();
    let mut restored = Serialization::load_from_minidump(path.to_str().unwrap());
    fs::remove_file(path).unwrap();
    fs::remove_file(dir.join(format!("{BASE:08x}-alloc_{BASE:x}.bin"))).unwrap();
    fs::remove_dir(dir).unwrap();
    assert_eq!(
        restored.maps.allocation_at(BASE).unwrap().size,
        3 * PAGE_SIZE
    );
    assert!(!restored.maps.query_memory(BASE).state.unwrap().committed);
    assert_eq!(
        restored.maps.query_memory(BASE + PAGE_SIZE).allocation_base,
        BASE
    );
    assert_eq!(restored.maps.read_dword(BASE + PAGE_SIZE), Some(0x12345678));
    assert_eq!(restored.maps.read_byte(BASE + 2 * PAGE_SIZE), None);
    restored
        .maps
        .commit_memory(BASE + 2 * PAGE_SIZE, PAGE_SIZE, RW, Permission::READ_WRITE)
        .unwrap();
    assert_eq!(restored.maps.read_byte(BASE + 2 * PAGE_SIZE), Some(0));
    restored.maps.release_memory(BASE).unwrap();
    assert!(!restored.maps.is_mapped(BASE + PAGE_SIZE));
}
