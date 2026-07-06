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
