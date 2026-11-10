use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::MemoryKind;
use crate::serialization::Serialization;
use crate::tests::helpers::{call_winapi32, call_winapi64};
use crate::winapi::common::memory::{self, Call};
use crate::winapi::common::{sections, virtual_protect};
use crate::{emu32, emu64};

const DATA: u64 = 0x3000_0000;
const FIRST: u64 = 0x4000_0000;
const SECOND: u64 = 0x5000_0000;
const RW: u32 = 4;
const RO: u32 = 2;

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
            &[u32::MAX as u64, 0, RW as u64, 0, 0x20000, 0],
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
            &mut emu, SECOND, 1, RW, DATA
        ));
        assert_eq!(emu.maps.read_dword(DATA), Some(0xabcdef));
        assert!(memory::allocate(&mut emu, SECOND, 1, 0x1000, RW).is_err());
        assert_eq!(emu.maps.query_memory(SECOND).kind, MemoryKind::Mapped);
        assert!(sections::close(&mut emu, handle));
        assert!(sections::map(&mut emu, handle, 0, 0, 0, RO).is_err());
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
        let args = [DATA, 0xf001f, 0, DATA + 32, RW as u64, 0x08000000, 0];
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
            RW as u64,
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
        let (second, _) = sections::map(&mut restored, handle, 0, 0, 0x20000, RW).unwrap();
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
        let (original, _) =
            sections::create(&mut emu, None, 0x1000, RW, "shared-section".into()).unwrap();
        let (view, _) = sections::map(&mut emu, original, 0, 0, 0, RW).unwrap();
        emu.maps.write_dword(view, 0x12345678);
        assert!(sections::close(&mut emu, original));
        let mut emu = Serialization::deserialize(&Serialization::serialize(&emu));
        let opened = call(
            &mut emu,
            |emu| sections::open_file_mapping(emu, false),
            &[4, 0, DATA],
        );
        assert_ne!(opened, 0);
        assert!(sections::map(&mut emu, opened, 0, 0, 0, RW).is_err());
        let (alias, _) = sections::map(&mut emu, opened, 0, 0, 0, RO).unwrap();
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
