use crate::emu::Emu;
use crate::maps::buffers::BufferKind;
use crate::maps::mem64::Permission;
use crate::serialization::Serialization;
use crate::tests::helpers::{call_winapi32, call_winapi64};
use crate::winapi::common::{bstr, buffers, crt, memory_ops};
use crate::winapi::{winapi32, winapi64};
use crate::{emu32, emu64};
use itertools::iproduct;

const DATA: u64 = 0x3000_0000;

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

fn call(emu: &mut Emu, function: fn(&mut Emu), args: &[u64], cdecl: bool) -> u64 {
    let stack = emu.regs().rsp;
    let value = if emu.cfg.arch.is_64bits() {
        call_winapi64(emu, function, args)
    } else {
        let args: Vec<_> = args.iter().map(|&a| a as u32).collect();
        let value = call_winapi32(emu, function, &args) as u64;
        if cdecl {
            assert_eq!(
                emu.regs().rsp,
                stack - args.len() as u64 * 4,
                "cdecl handler removed arguments"
            );
            for &arg in &args {
                assert_eq!(emu.stack_pop32(false), Some(arg));
            }
        }
        value
    };
    assert_eq!(emu.regs().rsp, stack);
    value
}

#[test]
fn cdecl_memory_exports_keep_arguments_and_copy_real_bytes() {
    for (is64, clear) in iproduct!([false, true], [false, true]) {
        let mut emu = prepare(is64);
        emu.maps.fill_range(DATA, 0xab, 256);
        let function = match (is64, clear) {
            (true, true) => winapi64::msvcrt::memset as fn(&mut Emu),
            (true, false) => winapi64::msvcrt::memmove,
            (false, true) => winapi32::msvcrt::memset,
            (false, false) => winapi32::msvcrt::memmove,
        };
        assert_eq!(
            call(
                &mut emu,
                function,
                &[DATA + 8, if clear { 0 } else { DATA }, 128],
                true
            ),
            DATA + 8
        );
        assert_eq!(
            emu.maps.read_byte(DATA + 8),
            Some(if clear { 0 } else { 0xab })
        );
        assert_eq!(emu.maps.read_byte(DATA + 7), Some(0xab));
        call(
            &mut emu,
            |emu| memory_ops::run(emu, memory_ops::Operation::Zero),
            &[DATA, 8],
            false,
        );
        assert_eq!(emu.maps.read_qword(DATA), Some(0));
    }
}

#[test]
fn crt_reallocation_preserves_old_block_on_failure_and_releases_size_zero() {
    for is64 in [false, true] {
        let mut emu = prepare(is64);
        let address = call(&mut emu, crt::malloc, &[257], true);
        assert_ne!(address, 0);
        emu.maps.write_dword(address, 0x12345678);
        assert_eq!(
            call(&mut emu, crt::realloc, &[address, u32::MAX as u64], true),
            0
        );
        assert_eq!(buffers::size(&emu, address, BufferKind::Crt), Some(257));
        assert_eq!(emu.maps.read_dword(address), Some(0x12345678));
        assert_eq!(call(&mut emu, crt::realloc, &[address, 0], true), 0);
        assert_eq!(buffers::size(&emu, address, BufferKind::Crt), None);
        assert_eq!(
            call(
                &mut emu,
                crt::calloc,
                &[u32::MAX as u64, u32::MAX as u64],
                true
            ),
            0
        );
        let zero = call(&mut emu, crt::calloc, &[17, 13], true);
        assert_ne!(zero, 0);
        assert_eq!(emu.maps.read_byte(zero + 220), Some(0));
    }
}

#[test]
fn aligned_free_releases_the_original_owner_for_small_and_large_blocks() {
    for (is64, size, alignment) in iproduct!([false, true], [257, 0x10000], [16, 0x1000, 0x10000]) {
        let mut emu = prepare(is64);
        let address = call(&mut emu, crt::aligned_malloc, &[size, alignment], true);
        assert_ne!(address, 0);
        assert_eq!(address % alignment, 0);
        let owner = emu.maps.buffers[&address];
        assert!(emu.maps.write_byte(address + size - 1, 0xab));
        call(&mut emu, crt::aligned_free, &[address], true);
        assert!(buffers::size(&emu, address, BufferKind::Aligned).is_none());
        if owner.arena {
            assert!(emu.heap_arenas[0].allocation_size(owner.base).is_none());
        } else {
            assert!(!emu.maps.is_mapped(owner.base));
        }
    }
}

#[test]
fn bstr_uses_four_byte_prefix_and_preserves_embedded_nuls_during_reallocation() {
    for is64 in [false, true] {
        let mut emu = prepare(is64);
        emu.maps.write_bytes(DATA, &[0x16, 0x04, 0, 0, 0x41, 0]);
        let string = call(
            &mut emu,
            |emu| bstr::alloc_api(emu, true, false),
            &[DATA, 3],
            false,
        );
        assert_ne!(string, 0);
        assert_eq!(emu.maps.read_dword(string - 4), Some(6));
        assert_eq!(
            call(
                &mut emu,
                |emu| bstr::length_api(emu, false),
                &[string],
                false
            ),
            3
        );
        assert_eq!(emu.maps.read_word(string + 4), Some(0x41));
        assert_eq!(emu.maps.read_word(string + 6), Some(0));
        if is64 {
            emu.maps.write_qword(DATA + 32, string);
        } else {
            emu.maps.write_dword(DATA + 32, string as u32);
        }
        assert_eq!(
            call(
                &mut emu,
                |emu| bstr::realloc_api(emu, true),
                &[DATA + 32, string, 3],
                false
            ),
            1
        );
        assert!(buffers::size(&emu, string, BufferKind::Bstr).is_none());
        let new = if is64 {
            emu.maps.read_qword(DATA + 32).unwrap()
        } else {
            emu.maps.read_dword(DATA + 32).unwrap() as u64
        };
        assert_eq!(emu.maps.read_word(new), Some(0x416));
        call(&mut emu, bstr::free_api, &[new], false);
        assert!(buffers::size(&emu, new, BufferKind::Bstr).is_none());
    }
}

#[test]
fn restored_crt_buffers_keep_owners_and_do_not_overlap_new_allocations() {
    for (is64, size) in iproduct!([false, true], [257, 0x10000]) {
        let mut emu = prepare(is64);
        let old = call(&mut emu, crt::malloc, &[size], true);
        assert_ne!(old, 0);
        emu.maps.write_dword(old, 0x12345678);
        let mut restored = Serialization::deserialize(&Serialization::serialize(&emu));
        let new = call(&mut restored, crt::malloc, &[size], true);
        assert_ne!(new, 0);
        assert_ne!(new, old);
        assert_eq!(restored.maps.read_dword(old), Some(0x12345678));
        call(&mut restored, crt::free, &[old], true);
        assert_eq!(buffers::size(&restored, old, BufferKind::Crt), None);
    }
}
