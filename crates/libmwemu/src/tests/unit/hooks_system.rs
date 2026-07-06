use std::sync::Arc;
use std::sync::atomic::AtomicI32;

use crate::emu::decoded_instruction::DecodedInstruction;
use crate::exception::types::ExceptionType;
use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::*;

#[test]
// test hooks system basic functionality
pub fn hooks_system() {
    helpers::setup();

    let mut hooks = crate::hooks::Hooks::new();

    // Test initial state - all hooks should be None
    assert!(hooks.hook_on_interrupt.is_none());
    assert!(hooks.hook_on_exception.is_none());
    assert!(hooks.hook_on_memory_read.is_none());
    assert!(hooks.hook_on_memory_write.is_none());
    assert!(hooks.hook_on_pre_instruction.is_none());
    assert!(hooks.hook_on_post_instruction.is_none());
    assert!(hooks.hook_on_winapi_call.is_none());
    assert!(hooks.hook_on_syscall.is_none());

    // Test setting hooks using the setter methods (which accept closures)
    hooks.on_interrupt(|_emu, _addr, _interrupt| true);
    assert!(hooks.hook_on_interrupt.is_some());

    hooks.on_exception(|_emu, _addr, _ex_type| true);
    assert!(hooks.hook_on_exception.is_some());

    hooks.on_memory_read(|_emu, _ip, _addr, _sz| {});
    assert!(hooks.hook_on_memory_read.is_some());

    hooks.on_memory_write(|_emu, _ip, _addr, _sz, value| value);
    assert!(hooks.hook_on_memory_write.is_some());

    hooks.on_pre_instruction(|_emu, _addr, _ins, _sz| true);
    assert!(hooks.hook_on_pre_instruction.is_some());

    hooks.on_post_instruction(|_emu, _addr, _ins, _sz, _ok| {});
    assert!(hooks.hook_on_post_instruction.is_some());

    hooks.on_winapi_call(|_emu, _addr, _called_addr| true);
    assert!(hooks.hook_on_winapi_call.is_some());

    hooks.on_syscall(|_emu, _nr| true);
    assert!(hooks.hook_on_syscall.is_some());

    // Test if all hooks are set
    assert!(!hooks.hook_on_interrupt.is_none());
    assert!(!hooks.hook_on_exception.is_none());
    assert!(!hooks.hook_on_memory_read.is_none());
    assert!(!hooks.hook_on_memory_write.is_none());
    assert!(!hooks.hook_on_pre_instruction.is_none());
    assert!(!hooks.hook_on_post_instruction.is_none());
    assert!(!hooks.hook_on_winapi_call.is_none());
    assert!(!hooks.hook_on_syscall.is_none());
}

#[test]
pub fn test_on_syscall() {
    helpers::setup();

    use std::sync::atomic::{AtomicU64, Ordering::SeqCst};

    let mut emu = emu64();

    let call_counter = Arc::new(AtomicI32::new(0));
    let seen_nr = Arc::new(AtomicU64::new(0));
    let counter = Arc::clone(&call_counter);
    let nr_slot = Arc::clone(&seen_nr);

    // The hook claims to fully handle the syscall: it writes its own return
    // value into rax and returns false to skip the internal dispatch.
    emu.hooks.on_syscall(move |emu: &mut Emu, nr: u64| -> bool {
        counter.fetch_add(1, SeqCst);
        nr_slot.store(nr, SeqCst);
        emu.regs_mut().rax = 0x1337;
        false
    });

    // Drive the Windows x64 syscall gateway directly with an arbitrary number.
    emu.regs_mut().rax = 0x55;
    crate::syscall::windows::syscall64::gateway(&mut emu);

    assert_eq!(call_counter.load(SeqCst), 1, "hook should fire once");
    assert_eq!(
        seen_nr.load(SeqCst),
        0x55,
        "hook should see the raw syscall nr"
    );
    assert_eq!(
        emu.regs().rax,
        0x1337,
        "returning false must skip the default dispatch and keep the hook's rax"
    );
}

#[test]
pub fn test_on_syscall_proceeds() {
    helpers::setup();

    let mut emu = emu64();

    let call_counter = Arc::new(AtomicI32::new(0));
    let counter = Arc::clone(&call_counter);

    // Returning true lets the default dispatch run, which overwrites rax with
    // an NTSTATUS (failure for an arbitrary/unimplemented number).
    emu.hooks
        .on_syscall(move |_emu: &mut Emu, _nr: u64| -> bool {
            counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            true
        });

    emu.regs_mut().rax = 0x55;
    crate::syscall::windows::syscall64::gateway(&mut emu);

    assert_eq!(
        call_counter.load(std::sync::atomic::Ordering::SeqCst),
        1,
        "hook should fire once"
    );
    let rax = emu.regs().rax;
    assert_ne!(
        rax, 0x55,
        "default dispatch should have run and changed rax"
    );
    assert!(
        rax >= 0xC000_0000,
        "default dispatch should leave a failure NTSTATUS, got 0x{:x}",
        rax
    );
}

#[test]
pub fn test_on_interrupt() {
    helpers::setup();

    let mut emu = emu32();
    let code_base = emu.alloc("code", 0x1000, Permission::READ_WRITE_EXECUTE);
    // int 0x80
    emu.maps.write_spaced_bytes(code_base, &"CD 80".to_string());

    let call_counter = Arc::new(AtomicI32::new(0));

    // clone it for the closure
    let counter = Arc::clone(&call_counter);

    emu.hooks
        .on_interrupt(move |emu: &mut Emu, ip: u64, int: u64| -> bool {
            counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            assert_eq!(ip, code_base);
            assert_eq!(int, 0x80);
            true
        });

    emu.set_pc(code_base);
    emu.step();

    // it should have been triggered once
    assert_eq!(call_counter.load(std::sync::atomic::Ordering::SeqCst), 1);
}

#[test]
pub fn test_on_exception() {
    helpers::setup();

    let mut emu = emu32();
    let code_base = emu.alloc("code", 0x1000, Permission::READ_WRITE_EXECUTE);
    // int 3 exception
    emu.maps.write_spaced_bytes(code_base, &"CC".to_string());

    let call_counter = Arc::new(AtomicI32::new(0));

    // clone it for the closure
    let counter = Arc::clone(&call_counter);

    emu.hooks.on_exception(
        move |emu: &mut Emu, ip: u64, ex_type: ExceptionType| -> bool {
            counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            assert_eq!(ip, code_base);
            assert_eq!(ex_type, ExceptionType::Int3);
            true
        },
    );

    emu.set_pc(code_base);
    emu.step();

    // it should have been triggered once
    assert_eq!(call_counter.load(std::sync::atomic::Ordering::SeqCst), 1);
}

#[test]
pub fn test_on_memory_read() {
    helpers::setup();

    let mut emu = emu32();
    let code_base = emu.alloc("code", 0x1000, Permission::READ_WRITE_EXECUTE);
    let data_base = emu.alloc("data", 0x2000, Permission::READ_WRITE);
    // mov eax, [data_base]
    emu.maps.write_spaced_bytes(
        code_base,
        &format!(
            "A1 {:02X} {:02X} {:02X} {:02X}",
            data_base as u8,
            (data_base >> 8) as u8,
            (data_base >> 16) as u8,
            (data_base >> 24) as u8
        ),
    );
    emu.maps.write_dword(data_base, 0x12345678);

    let call_counter = Arc::new(AtomicI32::new(0));

    // clone it for the closure
    let counter = Arc::clone(&call_counter);

    emu.hooks
        .on_memory_read(move |emu: &mut Emu, ip: u64, addr: u64, sz: u32| {
            counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            assert_eq!(ip, code_base);
            assert_eq!(addr, data_base);
            assert_eq!(sz, 32); // size in bits
        });

    emu.set_pc(code_base);
    emu.step();

    // it should have been triggered once
    assert_eq!(call_counter.load(std::sync::atomic::Ordering::SeqCst), 1);
}

#[test]
pub fn test_on_memory_write() {
    helpers::setup();

    let mut emu = emu32();
    let code_base = emu.alloc("code", 0x1000, Permission::READ_WRITE_EXECUTE);
    let data_base = emu.alloc("data", 0x2000, Permission::READ_WRITE);
    // mov [data_base], eax
    emu.maps.write_spaced_bytes(
        code_base,
        &format!(
            "A3 {:02X} {:02X} {:02X} {:02X}",
            data_base as u8,
            (data_base >> 8) as u8,
            (data_base >> 16) as u8,
            (data_base >> 24) as u8
        ),
    );
    emu.regs_mut().set_eax(0x12345678);

    let call_counter = Arc::new(AtomicI32::new(0));
    let counter = Arc::clone(&call_counter);

    emu.hooks.on_memory_write(
        move |emu: &mut Emu, ip: u64, addr: u64, sz: u32, value: u128| -> u128 {
            counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            assert_eq!(ip, code_base);
            assert_eq!(addr, data_base);
            assert_eq!(sz, 32); // size in bits
            assert_eq!(value, 0x12345678);
            value
        },
    );

    emu.set_pc(code_base);
    emu.step();

    // it should have been triggered once
    assert_eq!(call_counter.load(std::sync::atomic::Ordering::SeqCst), 1);
}

#[test]
pub fn test_pre_post_hooks() {
    helpers::setup();

    let mut emu = emu32();
    let code_base = emu.alloc("code", 0x1000, Permission::READ_WRITE_EXECUTE);
    // inc eax , inc eax
    emu.maps.write_spaced_bytes(code_base, &"40 40".to_string());

    let call_counter = Arc::new(AtomicI32::new(0));

    // clone it for the closure
    let counter = Arc::clone(&call_counter);

    emu.hooks.on_pre_instruction(
        move |emu: &mut Emu, ip: u64, ins: &DecodedInstruction, sz: usize| -> bool {
            let ctr_value = counter.load(std::sync::atomic::Ordering::SeqCst);
            if ctr_value == 1 {
                println!("Refused to skip");
                false
            } else {
                counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                true
            }
        },
    );

    emu.set_pc(code_base);
    emu.regs_mut().set_eax(0);
    emu.step();

    // it should have been triggered once
    assert_eq!(call_counter.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert_eq!(emu.regs().get_eax(), 1);

    // should refuse the skip
    emu.step();
    assert_eq!(call_counter.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert_eq!(emu.regs().get_eax(), 1);
}

#[test]
pub fn test_post_instruction_hook() {
    helpers::setup();

    let mut emu = emu32();
    let code_base = emu.alloc("code", 0x1000, Permission::READ_WRITE_EXECUTE);
    // inc eax
    emu.maps.write_spaced_bytes(code_base, &"40".to_string());

    let call_counter = Arc::new(AtomicI32::new(0));
    let counter = Arc::clone(&call_counter);

    emu.hooks.on_post_instruction(
        move |emu: &mut Emu, ip: u64, ins: &DecodedInstruction, sz: usize, ok: bool| {
            counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            assert_eq!(ip, code_base);
            assert_eq!(ok, true);
        },
    );

    emu.set_pc(code_base);
    emu.step();

    // it should have been triggered once
    assert_eq!(call_counter.load(std::sync::atomic::Ordering::SeqCst), 1);
}

#[test]
pub fn test_winapi_hook() {
    // inspired from os/winapi32_tests.rs test_virtual_alloc_32
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();

    let sample = helpers::test_data_path("exe64win_msgbox.bin");
    emu.load_code(&sample);
    emu.run(Some(0x14000123f));

    let message = emu.maps.read_string(emu.regs().rdx);
    let title = emu.maps.read_string(emu.regs().rdi);

    assert_eq!(message, "message");
    assert_eq!(title, "title");

    // we need to set the permission to use it
    let mem = emu
        .maps
        .get_mem_by_addr_mut(emu.regs().rdx)
        .expect("the memory need to be there");
    mem.set_permission(Permission::READ_WRITE);
    emu.maps.write_string(emu.regs().rdx, "inject");

    emu.hooks
        .on_winapi_call(|emu: &mut Emu, ip: u64, _called_addr: u64| -> bool {
            println!("WinAPI call at {:x} {_called_addr:x}", ip);
            // `ip` is the call site inside the test EXE (build-stable). The resolved
            // target lands inside a system DLL whose exact address depends on the
            // DLL build/layout, so just require a non-zero resolution rather than a
            // hardcoded address (which would break under different maps builds).
            assert_eq!(ip, 0x140001241);
            assert!(_called_addr != 0, "winapi call target should resolve");
            true
        });

    // launch the msgbox
    emu.step();
}
