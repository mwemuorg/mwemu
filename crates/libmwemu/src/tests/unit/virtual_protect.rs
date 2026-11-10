use itertools::iproduct;
use rs_header::pe::export_index::{ExportIndexData, ExportTarget, NamedExport};

use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::tests::helpers::{call_winapi32, call_winapi64, register_export_module};
use crate::winapi::{winapi32, winapi64};
use crate::windows::constants::{
    ERROR_INVALID_ADDRESS, ERROR_INVALID_PARAMETER, PAGE_READONLY, STATUS_CONFLICTING_ADDRESSES,
    STATUS_SUCCESS,
};
use crate::{emu32, emu64};

const PAGE_SIZE: u64 = 0x1000;
const PAGE_EXECUTE_READ: u32 = 0x20;
const PAGE_EXECUTE_READWRITE: u32 = 0x40;
const MARKER: u32 = 0x12345678;

fn prepare(is64: bool, permission: Permission) -> (Emu, u64, u64) {
    let mut emu = if is64 { emu64() } else { emu32() };
    emu.cfg.console_enabled = false;
    if is64 {
        emu.init_stack64();
    } else {
        emu.init_stack32();
    }
    let base = if is64 { 0x1_3000_0000 } else { 0x3000_0000 };
    let output = base + 3 * PAGE_SIZE;
    emu.maps
        .create_map("allocation", base, 2 * PAGE_SIZE, permission)
        .unwrap()
        .set_permission_range(base + PAGE_SIZE, PAGE_SIZE, Permission::READ);
    emu.maps
        .create_map("output", output, PAGE_SIZE, Permission::READ_WRITE)
        .unwrap();
    assert!(emu.maps.write_dword(output, MARKER));
    (emu, base, output)
}

fn call(emu: &mut Emu, extended: bool, args: [u64; 4]) -> u64 {
    let mut args = args.to_vec();
    if extended {
        args.insert(0, u64::MAX); // Current-process pseudo-handle.
    }
    let sp = emu.regs().rsp;
    let result = if emu.cfg.arch.is_64bits() {
        let handler = if extended {
            winapi64::kernel32::VirtualProtectEx
        } else {
            winapi64::kernel32::VirtualProtect
        };
        call_winapi64(emu, handler, &args)
    } else {
        let handler = if extended {
            winapi32::kernel32::VirtualProtectEx
        } else {
            winapi32::kernel32::VirtualProtect
        };
        let args: Vec<_> = args.into_iter().map(|arg| arg as u32).collect();
        u64::from(call_winapi32(emu, handler, &args))
    };
    assert_eq!(emu.regs().rsp, sp, "VirtualProtectEx={extended}");
    result
}

#[test]
fn virtual_protect_changes_permissions_and_returns_old_protection() {
    for (is64, extended, (permission, old)) in iproduct!(
        [false, true],
        [false, true],
        [
            (Permission::READ, PAGE_READONLY),
            (Permission::READ_EXECUTE, PAGE_EXECUTE_READ),
        ]
    ) {
        let (mut emu, base, output) = prepare(is64, permission);
        assert!(!emu.maps.write_byte(base, 0xaa));
        // An unaligned two-byte range spans two pages in one reservation.
        assert_eq!(
            call(
                &mut emu,
                extended,
                [
                    base + PAGE_SIZE - 1,
                    2,
                    PAGE_EXECUTE_READWRITE as u64,
                    output
                ],
            ),
            1
        );
        assert_eq!(emu.maps.read_dword(output), Some(old));
        for addr in [base, base + PAGE_SIZE] {
            assert_eq!(
                emu.maps
                    .get_mem_by_addr(addr)
                    .unwrap()
                    .permission_at(addr)
                    .bits(),
                Permission::READ_WRITE_EXECUTE.bits()
            );
        }
        assert!(emu.maps.has_code_writes());
        assert!(emu.maps.write_bytes(base + PAGE_SIZE - 1, &[0xaa, 0xbb]));
        assert_eq!(emu.maps.read_byte(base + PAGE_SIZE - 1), Some(0xaa));
        assert_eq!(emu.maps.read_byte(base + PAGE_SIZE), Some(0xbb));

        assert_eq!(
            call(&mut emu, extended, [base, PAGE_SIZE, old as u64, output]),
            1
        );
        assert_eq!(emu.maps.read_dword(output), Some(PAGE_EXECUTE_READWRITE));
        assert_eq!(
            emu.maps
                .get_mem_by_addr(base)
                .unwrap()
                .permission_at(base)
                .bits(),
            permission.bits()
        );
        assert!(!emu.maps.write_byte(base, 0xcc));
    }
}

#[test]
fn virtual_protect_failure_preserves_permissions_and_output() {
    for (is64, extended, (offset, size, protection, null_output, error)) in iproduct!(
        [false, true],
        [false, true],
        [
            (
                0,
                PAGE_SIZE,
                PAGE_EXECUTE_READWRITE,
                true,
                ERROR_INVALID_PARAMETER
            ),
            (
                2 * PAGE_SIZE,
                PAGE_SIZE,
                PAGE_EXECUTE_READWRITE,
                false,
                ERROR_INVALID_ADDRESS
            ),
            (
                0,
                3 * PAGE_SIZE,
                PAGE_EXECUTE_READWRITE,
                false,
                ERROR_INVALID_ADDRESS
            ),
            (0, 0, PAGE_EXECUTE_READWRITE, false, ERROR_INVALID_PARAMETER),
            (0, PAGE_SIZE, 0x60000000, false, ERROR_INVALID_PARAMETER),
        ]
    ) {
        let (mut emu, base, output) = prepare(is64, Permission::READ);
        emu.set_last_error(0);
        assert_eq!(
            call(
                &mut emu,
                extended,
                [
                    base + offset,
                    size,
                    u64::from(protection),
                    if null_output { 0 } else { output },
                ],
            ),
            0
        );
        assert_eq!(emu.get_last_error(), error);
        assert_eq!(emu.maps.read_dword(output), Some(MARKER));
        for addr in [base, base + PAGE_SIZE] {
            assert_eq!(
                emu.maps
                    .get_mem_by_addr(addr)
                    .unwrap()
                    .permission_at(addr)
                    .bits(),
                Permission::READ.bits()
            );
        }
        assert!(!emu.maps.has_code_writes());
    }
}

fn call_native(emu: &mut Emu, api: &str, args: [u64; 5]) -> u64 {
    let module = 0x7000_0000;
    let exports = ExportIndexData {
        export_base: 1,
        number_of_functions: 1,
        ordinal_targets: vec![Some(ExportTarget::Direct { rva: 0x100 })],
        named_exports: vec![NamedExport {
            name: api.into(),
            ordinal_index: 0,
        }],
    };
    register_export_module(emu, "ntdll.dll", module, &exports);
    let sp = emu.regs().rsp;
    if emu.cfg.arch.is_64bits() {
        call_winapi64(
            emu,
            |emu| {
                winapi64::ntdll::gateway(0x7000_0100, emu);
            },
            &args,
        );
    } else {
        let args: Vec<_> = args.into_iter().map(|arg| arg as u32).collect();
        call_winapi32(
            emu,
            |emu| {
                winapi32::ntdll::gateway(0x7000_0100, emu);
            },
            &args,
        );
    }
    assert_eq!(emu.regs().rsp, sp, "{api}");
    emu.regs().rax
}

#[test]
fn nt_protect_virtual_memory_updates_range_and_handles_unmapped_pages() {
    for (is64, api, mapped) in iproduct!(
        [false, true],
        ["NtProtectVirtualMemory", "ZwProtectVirtualMemory"],
        [true, false]
    ) {
        let (mut emu, base, output) = prepare(is64, Permission::READ);
        let base_ptr = output + 8;
        let size_ptr = output + 16;
        let address = base + PAGE_SIZE - 1;
        let size = if mapped { 2 } else { PAGE_SIZE + 2 };
        // Leave sentinels above the x86 pointer-sized output values.
        assert!(emu.maps.write_qword(base_ptr, u64::MAX));
        assert!(emu.maps.write_qword(size_ptr, u64::MAX));
        if is64 {
            assert!(emu.maps.write_qword(base_ptr, address));
            assert!(emu.maps.write_qword(size_ptr, size));
        } else {
            assert!(emu.maps.write_dword(base_ptr, address as u32));
            assert!(emu.maps.write_dword(size_ptr, size as u32));
        }
        emu.set_last_error(MARKER as u64);
        assert_eq!(
            call_native(
                &mut emu,
                api,
                [
                    u64::MAX,
                    base_ptr,
                    size_ptr,
                    PAGE_EXECUTE_READWRITE as u64,
                    output
                ],
            ),
            if mapped {
                STATUS_SUCCESS
            } else {
                STATUS_CONFLICTING_ADDRESSES
            },
            "{api}, x64={is64}"
        );
        assert_eq!(emu.get_last_error(), MARKER as u64);
        assert_eq!(
            emu.maps.read_dword(output),
            Some(if mapped { PAGE_READONLY } else { MARKER })
        );
        let expected_base = if mapped { base } else { address };
        let expected_size = if mapped { 2 * PAGE_SIZE } else { size };
        if is64 {
            assert_eq!(emu.maps.read_qword(base_ptr), Some(expected_base));
            assert_eq!(emu.maps.read_qword(size_ptr), Some(expected_size));
        } else {
            assert_eq!(emu.maps.read_dword(base_ptr), Some(expected_base as u32));
            assert_eq!(emu.maps.read_dword(size_ptr), Some(expected_size as u32));
            assert_eq!(emu.maps.read_dword(base_ptr + 4), Some(u32::MAX));
            assert_eq!(emu.maps.read_dword(size_ptr + 4), Some(u32::MAX));
        }
        for addr in [base, base + PAGE_SIZE] {
            let permission = emu.maps.get_mem_by_addr(addr).unwrap().permission();
            assert_eq!(permission.can_write(), mapped);
            assert_eq!(permission.can_execute(), mapped);
        }
    }
}
