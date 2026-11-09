use std::{env, fs};

use itertools::iproduct;

use crate::arch::OperatingSystem;
use crate::maps::Maps;
use crate::maps::mem64::Permission;
use crate::serialization::Serialization;
use crate::syscall::windows::syscall64::memory::{
    nt_protect_virtual_memory, nt_query_virtual_memory,
};
use crate::winapi::common::virtual_protect::{ProtectError, protect};
use crate::windows::constants::{STATUS_CONFLICTING_ADDRESSES, STATUS_SUCCESS};
use crate::windows::structures::MemoryBasicInformation64;
use crate::{emu32, emu64};

const BASE: u64 = 0x400000;
const PAGE: u64 = 0x1000;
const PAGE_READONLY: u32 = 0x02;
const PAGE_EXECUTE: u32 = 0x10;
const PAGE_EXECUTE_READWRITE: u32 = 0x40;

fn allocation() -> Maps {
    let mut maps = Maps::default();
    maps.create_map("allocation", BASE, PAGE * 4, Permission::READ_WRITE_EXECUTE)
        .unwrap();
    assert!(maps.write_bytes(BASE, &vec![0x41; (PAGE * 4) as usize]));
    maps
}

#[test]
fn protection_changes_preserve_allocation_ownership() {
    for free_by_name in [false, true] {
        let mut maps = allocation();
        let key = maps.maps[&BASE];
        let storage = maps.get_mem("allocation").get_bytes().as_ptr();
        assert_eq!(
            protect(&mut maps, BASE + PAGE + 7, 2, PAGE_READONLY),
            Ok(PAGE_EXECUTE_READWRITE)
        );
        let mem = maps.get_mem("allocation");
        assert_eq!(mem.get_bytes().as_ptr(), storage);
        assert_eq!(mem.size(), (4 * PAGE) as usize);
        assert_eq!(maps.maps[&BASE], key);
        assert_eq!(maps.maps.len(), 1);
        for (offset, writable) in [(0, true), (PAGE, false), (2 * PAGE, true), (3 * PAGE, true)] {
            assert_eq!(maps.write_byte(BASE + offset, 0x42), writable);
        }
        // Restore only the protected page, then release the original allocation.
        assert_eq!(
            protect(&mut maps, BASE + PAGE, PAGE, PAGE_EXECUTE_READWRITE),
            Ok(PAGE_READONLY)
        );
        assert!(maps.write_byte(BASE + PAGE, 0x42));
        if free_by_name {
            maps.free("allocation");
        } else {
            maps.dealloc(BASE);
        }
        assert!(maps.maps.is_empty());
        assert!(!maps.is_mapped(BASE + 3 * PAGE));
    }
}

#[test]
fn protected_access_checks_the_entire_range() {
    let mut maps = allocation();
    // Prime the address lookup cache before changing permissions.
    assert_eq!(maps.read_dword(BASE + PAGE - 2), Some(0x41414141));
    protect(&mut maps, BASE + PAGE, PAGE, PAGE_READONLY).unwrap();
    assert!(!maps.write_dword(BASE + PAGE - 2, 0x42424242));
    assert!(!maps.validate_write_range(BASE + PAGE - 2, 4));
    assert!(maps.validate_write_range(BASE + 2 * PAGE, 4));
    assert!(!maps.write_bytes(BASE + PAGE - 2, &[0x42; 4]));
    assert!(!maps.memcpy(BASE + PAGE - 2, BASE, 4));
    let mem = maps.get_mem_by_addr_mut(BASE).unwrap();
    mem.write_dword(BASE + PAGE - 2, 0x42424242);
    mem.write_string(BASE + PAGE - 2, "BBBB");
    assert_eq!(
        &mem.get_bytes()[PAGE as usize - 2..PAGE as usize + 2],
        &[0x41; 4]
    );
    assert!(mem.set_permission_range(BASE + PAGE, PAGE, Permission::NONE));
    assert_eq!(maps.read_dword(BASE + PAGE - 2), None);
    assert!(maps.try_read_bytes(BASE + PAGE - 2, 4).is_none());
    let mem = maps.get_mem_by_addr(BASE).unwrap();
    assert_eq!(mem.read_bytes(BASE + PAGE - 2, 4).as_ref(), &[0x41; 2]);
    assert_eq!(mem.read_string(BASE + PAGE - 2), "AA");
    assert_eq!(mem.read_wide_string(BASE + PAGE - 2), "\u{4141}");
    assert!(maps.write_byte(BASE + 2 * PAGE, 0x42));
}

#[test]
fn protection_rejects_invalid_requests_without_partial_changes() {
    for (address, size, protection, expected) in [
        (BASE, PAGE, 0, ProtectError::InvalidProtection),
        (BASE, PAGE, 0x60000000, ProtectError::InvalidProtection),
        (BASE, PAGE, 0x40000000, ProtectError::InvalidProtection),
        (BASE, PAGE, 0x06, ProtectError::InvalidProtection),
        (BASE, 0, PAGE_READONLY, ProtectError::InvalidParameter),
        (
            u64::MAX - PAGE,
            PAGE,
            PAGE_READONLY,
            ProtectError::InvalidParameter,
        ),
        (BASE, 5 * PAGE, PAGE_READONLY, ProtectError::Unmapped),
    ] {
        let mut maps = allocation();
        assert_eq!(protect(&mut maps, address, size, protection), Err(expected));
        for offset in [0, PAGE, 2 * PAGE, 3 * PAGE] {
            assert!(maps.write_byte(BASE + offset, 0x42));
        }
    }
}

#[test]
fn protection_survives_trimming_and_unmapping() {
    let mut maps = allocation();
    protect(&mut maps, BASE + PAGE, PAGE, PAGE_READONLY).unwrap();
    protect(&mut maps, BASE + 2 * PAGE, PAGE, PAGE_EXECUTE).unwrap();
    maps.unmap_range(BASE, PAGE);
    assert_eq!(
        maps.get_mem_by_addr(BASE + PAGE)
            .unwrap()
            .permission()
            .bits(),
        Permission::READ.bits()
    );
    maps.unmap_range(BASE + PAGE + 128, 64);
    assert!(!maps.is_mapped(BASE + PAGE + 128));
    assert!(!maps.write_byte(BASE + PAGE + 192, 0));
    assert_eq!(maps.read_byte(BASE + 2 * PAGE), None);
    assert!(
        maps.get_mem_by_addr(BASE + 2 * PAGE)
            .unwrap()
            .can_execute_range(BASE + 2 * PAGE, PAGE as usize)
    );
    assert!(maps.write_byte(BASE + 3 * PAGE, 0x42));
}

#[test]
fn protection_survives_native_and_minidump_snapshots() {
    let mut emu = emu32();
    emu.maps = allocation();
    protect(&mut emu.maps, BASE + PAGE, PAGE, PAGE_READONLY).unwrap();
    protect(&mut emu.maps, BASE + 2 * PAGE, PAGE, PAGE_EXECUTE).unwrap();
    let mut restored = Serialization::deserialize(&Serialization::serialize(&emu));
    assert_eq!(restored.maps.maps.len(), 1);
    assert_eq!(
        restored.maps.get_mem("allocation").size(),
        (4 * PAGE) as usize
    );
    assert!(!restored.maps.write_byte(BASE + PAGE, 0));
    assert_eq!(restored.maps.read_byte(BASE + 2 * PAGE), None);
    assert!(restored.maps.write_byte(BASE + 3 * PAGE, 0x42));

    let dir = env::temp_dir().join("mwemu_page_protection");
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("state.mdmp");
    Serialization::dump_to_minidump(&emu, path.to_str().unwrap()).unwrap();
    let mut restored = Serialization::load_from_minidump(path.to_str().unwrap());
    fs::remove_file(path).unwrap();
    fs::remove_file(dir.join(format!("{BASE:08x}-allocation.bin"))).unwrap();
    fs::remove_dir(dir).unwrap();
    assert!(!restored.maps.write_byte(BASE + PAGE, 0));
    assert_eq!(restored.maps.read_byte(BASE + 2 * PAGE), None);
    assert!(restored.maps.write_byte(BASE + 3 * PAGE, 0x42));
}

#[test]
fn cached_fetch_uses_execute_permission_without_data_read_access() {
    for is64 in [false, true] {
        let mut emu = if is64 { emu64() } else { emu32() };
        emu.os = OperatingSystem::Linux;
        emu.cfg.console_enabled = false;
        emu.cfg.max_instructions = Some(16);
        emu.cfg.timeout_secs = Some(1.0);
        emu.maps = allocation();
        assert!(emu.maps.write_bytes(BASE, &[0x90; 16]));
        emu.set_pc(BASE);
        assert_eq!(emu.run(Some(BASE + 1)).unwrap(), BASE + 1);
        protect(&mut emu.maps, BASE, PAGE, PAGE_EXECUTE).unwrap();
        assert_eq!(emu.maps.read_byte(BASE), None);
        assert_eq!(emu.run(Some(BASE + 2)).unwrap(), BASE + 2);
        protect(&mut emu.maps, BASE, PAGE, PAGE_READONLY).unwrap();
        assert_eq!(emu.run(Some(BASE + 3)).unwrap(), BASE + 3);
        emu.maps
            .get_mem_by_addr_mut(BASE)
            .unwrap()
            .set_permission_range(BASE, PAGE, Permission::NONE);
        assert!(emu.run(Some(BASE + 4)).is_err());
    }
}

#[test]
fn syscall_protection_and_query_use_page_ranges() {
    for (name, (offset, size, protection, status)) in iproduct!(
        ["allocation", "image.didat"],
        [
            (PAGE + 7, 2, PAGE_READONLY, STATUS_SUCCESS),
            (4 * PAGE, PAGE, PAGE_READONLY, STATUS_CONFLICTING_ADDRESSES),
            (PAGE, PAGE, 0x60000000, 0xc0000045),
        ]
    ) {
        let mut emu = emu64();
        emu.init_stack64();
        emu.maps
            .create_map(name, BASE, 4 * PAGE, Permission::READ_WRITE_EXECUTE)
            .unwrap();
        let io = emu.regs().rsp - PAGE;
        let address = BASE + offset;
        assert!(emu.maps.write_qword(io, address));
        assert!(emu.maps.write_qword(io + 8, size));
        assert!(emu.maps.write_dword(io + 16, 0x12345678));
        emu.regs_mut().rcx = u64::MAX;
        emu.regs_mut().rdx = io;
        emu.regs_mut().r8 = io + 8;
        emu.regs_mut().r9 = u64::from(protection);
        assert!(emu.maps.write_qword(emu.regs().rsp + 0x28, io + 16));
        nt_protect_virtual_memory(&mut emu);
        assert_eq!(emu.regs().rax, status);
        let ok = status == STATUS_SUCCESS;
        assert_eq!(
            emu.maps.read_dword(io + 16),
            Some(if ok {
                PAGE_EXECUTE_READWRITE
            } else {
                0x12345678
            })
        );
        assert_eq!(
            emu.maps.read_qword(io),
            Some(if ok { BASE + PAGE } else { address })
        );
        assert_eq!(
            emu.maps.read_qword(io + 8),
            Some(if ok { PAGE } else { size })
        );
        assert!(emu.maps.write_byte(BASE, 0x42));
        assert_eq!(emu.maps.write_byte(BASE + PAGE, 0x42), !ok);
        assert!(emu.maps.write_byte(BASE + 2 * PAGE, 0x42));
        assert_eq!(emu.maps.get_mem(name).size(), (4 * PAGE) as usize);
        if ok {
            emu.regs_mut().rcx = u64::MAX;
            emu.regs_mut().rdx = BASE + PAGE + 8;
            emu.regs_mut().r8 = 0;
            emu.regs_mut().r9 = io + 32;
            assert!(
                emu.maps
                    .write_qword(emu.regs().rsp + 0x28, MemoryBasicInformation64::SIZE)
            );
            assert!(emu.maps.write_qword(emu.regs().rsp + 0x30, 0));
            nt_query_virtual_memory(&mut emu);
            assert_eq!(emu.regs().rax, STATUS_SUCCESS);
            assert_eq!(emu.maps.read_qword(io + 32), Some(BASE + PAGE));
            assert_eq!(emu.maps.read_qword(io + 40), Some(BASE));
            assert_eq!(emu.maps.read_qword(io + 56), Some(PAGE));
            assert_eq!(emu.maps.read_dword(io + 68), Some(PAGE_READONLY));
        }
    }
}

#[test]
fn rep_stores_stop_at_a_protected_page() {
    for (is64, copy) in iproduct!([false, true], [false, true]) {
        let mut emu = if is64 { emu64() } else { emu32() };
        emu.os = OperatingSystem::Linux;
        emu.cfg.console_enabled = false;
        emu.cfg.max_instructions = Some(64);
        emu.cfg.timeout_secs = Some(1.0);
        emu.maps = allocation();
        assert!(
            emu.maps
                .write_bytes(BASE, &[0xf3, if copy { 0xa4 } else { 0xaa }])
        );
        let destination = BASE + PAGE - 8;
        assert!(emu.maps.write_bytes(destination, &[0xcc; 32]));
        assert!(emu.maps.write_bytes(BASE + 2 * PAGE, &[0x42; 32]));
        protect(&mut emu.maps, BASE + PAGE, PAGE, PAGE_READONLY).unwrap();
        emu.regs_mut().rdi = destination;
        emu.regs_mut().rsi = BASE + 2 * PAGE;
        emu.regs_mut().rcx = 32;
        emu.regs_mut().rax = 0x42;
        emu.set_pc(BASE);
        let _ = emu.run(Some(BASE + 2));
        assert_eq!(emu.maps.read_bytes(destination, 8).as_ref(), &[0x42; 8]);
        assert_eq!(emu.maps.read_bytes(BASE + PAGE, 24).as_ref(), &[0xcc; 24]);
        assert_eq!(emu.regs().rdi, BASE + PAGE);
    }
}
