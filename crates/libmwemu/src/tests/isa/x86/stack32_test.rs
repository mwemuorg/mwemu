use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::PAGE_SIZE;
use crate::windows::constants::memory::PAGE_GUARD;
use crate::windows::structures::TEB;
use crate::{tests::helpers, *};

#[test]
// stack32 tests
pub fn stack32_test() {
    helpers::setup();

    let mut emu = emu32();
    emu.cfg.maps_folder = win_maps!(32);
    emu.init_win32(false, false).unwrap();

    let stack_check = emu.maps.get_map_by_name("stack");
    assert!(stack_check.is_some());
    let stack = stack_check.unwrap();
    let base = stack.get_base();

    assert!(emu.regs().get_esp() < emu.regs().get_ebp());
    assert!(emu.regs().get_esp() > stack.get_base());
    assert!(emu.regs().get_esp() < stack.get_bottom());
    assert!(emu.regs().get_ebp() > stack.get_base());
    assert!(emu.regs().get_ebp() < stack.get_bottom());
    assert!(stack.inside(emu.regs().get_esp()));
    assert!(stack.inside(emu.regs().get_ebp()));

    for i in 0..5000 {
        emu.stack_push32(i as u32);
    }
    emu.stack_pop32(false);

    assert!(emu.regs().get_esp() > base);
}

#[test]
fn initial_test_stack_alignment_32bit_bare_metal() {
    let mut emu = emu32();
    emu.init_cpu();

    assert_eq!(
        emu.regs().get_esp() % 4,
        0,
        "in 32bits stack has to be aligned to 4 bytes"
    );
}

#[test]
fn pe_stack32_moves_the_guard_within_the_image_reserve() {
    let mut emu = emu32();
    let teb = 0x10000;
    emu.maps
        .create_map("teb", teb, TEB::map_size() as u64, Permission::READ_WRITE)
        .unwrap();
    let reserve = 2 * 0x10000;
    let commit = 2 * PAGE_SIZE;
    emu.maps.max_alloc_size = commit;
    emu.init_pe_stack32(reserve, commit).unwrap();
    let bottom = emu.cfg.stack_addr;
    let top = bottom + reserve;
    let limit = top - commit;
    assert_eq!(
        emu.maps.read_dword(teb + TEB::NT_TIB_STACK_BASE_OFFSET),
        Some(top as u32)
    );
    assert_eq!(
        emu.maps.read_dword(teb + TEB::DEALLOCATION_STACK_OFFSET),
        Some(bottom as u32)
    );
    assert_eq!(
        emu.maps.read_dword(teb + TEB::NT_TIB_STACK_LIMIT_OFFSET),
        Some(limit as u32)
    );
    assert!(emu.maps.read_byte(limit - 1).is_none());
    assert!(!emu.maps.is_mapped(bottom));
    assert!(!emu.maps.is_range_free(bottom, PAGE_SIZE));
    assert!(!emu.try_grow_stack(limit - 2 * PAGE_SIZE));
    emu.regs_mut().set_esp(limit);
    assert!(emu.stack_push32(0x1234));
    assert_eq!(emu.maps.read_dword(limit - 4), Some(0x1234));
    let new_limit = limit - PAGE_SIZE;
    assert_eq!(
        emu.maps.read_dword(teb + TEB::NT_TIB_STACK_LIMIT_OFFSET),
        Some(new_limit as u32)
    );
    assert_eq!(
        emu.maps
            .allocation_at(bottom)
            .unwrap()
            .state(new_limit - 1)
            .protection
            & PAGE_GUARD,
        PAGE_GUARD
    );
    for index in (1..(new_limit - bottom) / PAGE_SIZE).rev() {
        assert!(emu.try_grow_stack(bottom + index * PAGE_SIZE));
    }
    emu.regs_mut().set_esp(bottom + PAGE_SIZE);
    assert!(!emu.stack_push32(123));
    assert_eq!(emu.regs().get_esp(), bottom + PAGE_SIZE);
    assert!(!emu.try_grow_stack(bottom - 1));
}
