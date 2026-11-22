//! Linux libc stubs, called directly through the libc gateway.
use rs_header::elf::elf64::Elf64;

use super::unix_api_helpers::{aarch64_env, put_str, ret, scratch, set_args, stdout_text};
use crate::api::abi::ArgumentFault;
use crate::api::linux;
use crate::api::linux::libc::gateway;
use crate::api::unix::{self, ApiStopReason};
use crate::arch::OperatingSystem;
use crate::maps::mem64::Permission;
use crate::serialization::Serialization;
use crate::tests::helpers::setup;
use crate::windows::constants::memory::LIBS64_MIN;
use crate::{emu_aarch64, emu64};

#[test]
fn printf_family_uses_register_varargs() {
    let (mut emu, d) = aarch64_env();
    let fmt = put_str(&mut emu, d, "%d-%s-%x");
    let word = put_str(&mut emu, d + 0x100, "hi");
    set_args(&mut emu, &[fmt, 42, word, 255]);
    gateway("printf", &mut emu);
    assert_eq!(stdout_text(&emu), "42-hi-ff");
    assert_eq!(ret(&emu), 8);

    let buf = d + 0x200;
    let fmt2 = put_str(&mut emu, d + 0x300, "value=%05d");
    set_args(&mut emu, &[buf, 6, fmt2, 7]);
    gateway("snprintf", &mut emu);
    assert_eq!(ret(&emu), 11, "returns the untruncated length");
    assert_eq!(emu.maps.read_string(buf), "value");
}

#[test]
fn printf_spills_ninth_argument_to_stack() {
    let (mut emu, d) = aarch64_env();
    let fmt = put_str(&mut emu, d, "%d %d %d %d %d %d %d %d");
    set_args(&mut emu, &[fmt, 1, 2, 3, 4, 5, 6, 7]);
    let sp = emu.regs_aarch64().sp;
    emu.maps.write_qword(sp, 8);
    gateway("printf", &mut emu);
    assert_eq!(stdout_text(&emu), "1 2 3 4 5 6 7 8");
}

#[test]
fn puts_and_write_reach_stdout() {
    let (mut emu, d) = aarch64_env();
    let s = put_str(&mut emu, d, "line");
    set_args(&mut emu, &[s]);
    gateway("puts", &mut emu);
    set_args(&mut emu, &[1, s, 2]);
    gateway("write", &mut emu);
    assert_eq!(ret(&emu), 2);
    set_args(&mut emu, &[7, s, 4]);
    gateway("write", &mut emu); // not stdout/stderr
    assert_eq!(stdout_text(&emu), "line\nli");
}

#[test]
fn heap_allocation_round_trip() {
    let (mut emu, _) = aarch64_env();
    set_args(&mut emu, &[4, 8]);
    gateway("calloc", &mut emu);
    let p = ret(&emu);
    assert_ne!(p, 0);
    assert_eq!(emu.maps.read_qword(p + 24), Some(0));
    emu.maps.write_qword(p, 0x1122_3344);

    set_args(&mut emu, &[p, 64]);
    gateway("realloc", &mut emu);
    let q = ret(&emu);
    assert_eq!(emu.maps.read_qword(q), Some(0x1122_3344));
    assert!(!emu.maps.is_mapped(p), "old block released");

    set_args(&mut emu, &[q]);
    gateway("free", &mut emu);
    assert!(!emu.maps.is_mapped(q));
}

#[test]
fn mmap_and_munmap() {
    let (mut emu, _) = aarch64_env();
    set_args(&mut emu, &[0, 0x2000, 3, 0x22, u64::MAX, 0]);
    gateway("mmap", &mut emu);
    let p = ret(&emu);
    assert_ne!(p, u64::MAX);
    assert!(emu.maps.write_qword(p + 0x1ff8, 5));

    set_args(&mut emu, &[p, 0x2000]);
    gateway("munmap", &mut emu);
    assert_eq!(ret(&emu), 0);
    assert!(!emu.maps.is_mapped(p));
}

#[test]
fn process_and_terminal_queries() {
    let (mut emu, d) = aarch64_env();
    let name = put_str(&mut emu, d, "HOME");
    set_args(&mut emu, &[name]);
    gateway("getenv", &mut emu);
    assert_eq!(ret(&emu), 0);

    gateway("getpid", &mut emu);
    assert_eq!(ret(&emu), 1234);

    set_args(&mut emu, &[1]);
    gateway("isatty", &mut emu);
    assert_eq!(ret(&emu), 1);

    let ws = d + 0x100;
    set_args(&mut emu, &[1, 0x5413, ws]);
    gateway("ioctl", &mut emu);
    assert_eq!(ret(&emu), 0);
    assert_eq!(emu.maps.read_word(ws), Some(24));
    assert_eq!(emu.maps.read_word(ws + 2), Some(80));

    let tloc = d + 0x200;
    set_args(&mut emu, &[tloc]);
    gateway("time", &mut emu);
    assert!(ret(&emu) > 1_600_000_000);
    assert_eq!(emu.maps.read_qword(tloc), Some(ret(&emu)));
}

#[test]
fn unknown_symbol_returns_zero_without_panicking() {
    let (mut emu, _) = aarch64_env();
    emu.regs_aarch64_mut().x[0] = 0xdead;
    gateway("definitely_not_a_libc_symbol", &mut emu);
    assert_eq!(ret(&emu), 0);
}

#[test]
fn x86_64_sysv_arguments() {
    setup();
    let mut emu = emu64();
    let d = scratch(&mut emu, "api_data", 0x1000);
    let fmt = put_str(&mut emu, d, "%s=%d");
    let key = put_str(&mut emu, d + 0x100, "k");
    emu.regs_mut().rdi = fmt;
    emu.regs_mut().rsi = key;
    emu.regs_mut().rdx = 9;
    gateway("printf", &mut emu);
    assert_eq!(stdout_text(&emu), "k=9");
    assert_eq!(emu.regs().rax, 3);
}

const ELF64_HELLO: &[u8] = include_bytes!("../fixtures/elf64_x86_64_linux_hello.bin");
const CODE: u64 = 0x400000;
const STACK: u64 = 0x500000;
const DATA: u64 = 0x600000;
const RETURN_MARKER: u64 = 0x12345678;
const QWORD_BYTES: u64 = size_of::<u64>() as u64;

#[test]
fn invalid_vararg_stops_execution_without_printing_or_returning() {
    for aarch64 in [false, true] {
        for execution in ["step", "run", "threaded"] {
            let mut emu = if aarch64 { emu_aarch64() } else { emu64() };
            emu.os = OperatingSystem::Linux;
            emu.cfg.linux_real_libc = false;
            emu.cfg.console_enabled = false;
            emu.running_script = true;
            emu.cfg.enable_threading = execution == "threaded";
            let mut elf = Elf64::parse(ELF64_HELLO).unwrap();
            elf.addr_to_symbol.insert(LIBS64_MIN, "printf".into());
            emu.elf64 = Some(elf);
            for (name, base, size) in [
                ("code", CODE, 0x100),
                ("stack", STACK, QWORD_BYTES),
                ("data", DATA, 0x100),
                ("libc.so.6.text", LIBS64_MIN, 0x100),
            ] {
                emu.maps
                    .create_map(name, base, size, Permission::READ_WRITE_EXECUTE)
                    .unwrap();
            }
            emu.set_sp(STACK + QWORD_BYTES);
            emu.set_pc(CODE);
            let end = if aarch64 {
                put_str(&mut emu, DATA, "%u%u%u%u%u%u%u%u");
                emu.regs_aarch64_mut().x[0] = DATA;
                emu.regs_aarch64_mut().x[9] = LIBS64_MIN;
                emu.regs_aarch64_mut().x[19] = RETURN_MARKER;
                // blr x9; mov x19, #0; nop
                assert!(emu.maps.write_bytes(
                    CODE,
                    &[
                        0x20, 0x01, 0x3f, 0xd6, 0x13, 0, 0x80, 0xd2, 0x1f, 0x20, 0x03, 0xd5
                    ]
                ));
                CODE + 8
            } else {
                put_str(&mut emu, DATA, "%u%u%u%u%u%u");
                emu.regs_mut().rdi = DATA;
                emu.regs_mut().r11 = LIBS64_MIN;
                emu.regs_mut().rax = RETURN_MARKER;
                emu.regs_mut().rbx = RETURN_MARKER;
                // call r11; xor ebx, ebx; nop
                assert!(
                    emu.maps
                        .write_bytes(CODE, &[0x41, 0xff, 0xd3, 0x31, 0xdb, 0x90])
                );
                CODE + 5
            };
            gateway("__errno_location", &mut emu);
            let errno = if aarch64 {
                emu.regs_aarch64().x[0]
            } else {
                emu.regs().rax
            };
            assert!(emu.maps.write_dword(errno, RETURN_MARKER as u32));
            if aarch64 {
                emu.regs_aarch64_mut().x[0] = DATA;
            } else {
                emu.regs_mut().rax = RETURN_MARKER;
            }
            emu.set_last_error(RETURN_MARKER);
            if execution == "step" {
                assert!(!emu.step(), "aarch64={aarch64}");
            } else {
                let error = emu.run(Some(end)).unwrap_err();
                assert!(error.to_string().contains("Unix API call stopped"));
            }
            let fault = emu.unix_api.fault.as_ref().unwrap();
            assert_eq!(fault.call.name, "printf");
            let ApiStopReason::InvalidArgument(error) = fault.reason else {
                panic!("{fault:?}");
            };
            assert_eq!(error.fault, ArgumentFault::Unreadable);
            assert_eq!(error.index, if aarch64 { 8 } else { 6 });
            assert_eq!(error.address, Some(STACK + QWORD_BYTES));
            assert_eq!(emu.pc(), fault.call.pc);
            assert_eq!(emu.sp(), fault.call.stack_pointer);
            assert!(emu.emulated_stdout.is_empty());
            assert_eq!(emu.maps.read_dword(errno), Some(RETURN_MARKER as u32));
            assert_eq!(emu.get_last_error(), RETURN_MARKER);
            assert!(emu.api_policy.last_outcome.is_none());
            if aarch64 {
                assert_eq!(emu.regs_aarch64().x[0], DATA);
                assert_eq!(emu.regs_aarch64().x[19], RETURN_MARKER);
            } else {
                assert_eq!(emu.regs().rax, RETURN_MARKER);
                assert_eq!(emu.regs().rbx, RETURN_MARKER);
            }
            assert!(!emu.step());
            assert!(emu.run(None).is_err());
            let mut restored = Serialization::deserialize(&Serialization::serialize(&emu));
            assert_eq!(restored.unix_api.fault, emu.unix_api.fault);
            assert!(restored.run(None).is_err());
        }
    }
}

#[test]
fn invalid_return_address_does_not_dispatch_the_unix_handler() {
    let mut emu = emu64();
    emu.os = OperatingSystem::Linux;
    emu.cfg.linux_real_libc = false;
    emu.set_sp(STACK);
    emu.set_pc(CODE);
    emu.regs_mut().rax = RETURN_MARKER;
    emu.maps
        .create_map(
            "libc.so.6.text",
            LIBS64_MIN,
            0x100,
            Permission::READ_EXECUTE,
        )
        .unwrap();
    let mut elf = Elf64::parse(ELF64_HELLO).unwrap();
    elf.addr_to_symbol.insert(LIBS64_MIN, "getpid".into());
    emu.elf64 = Some(elf);
    assert!(emu.set_rip_with_check(LIBS64_MIN, true));
    assert_eq!(
        emu.unix_api.fault.as_ref().unwrap().reason,
        ApiStopReason::InvalidReturnAddress
    );
    assert_eq!(emu.regs().rax, RETURN_MARKER);
    assert_eq!(emu.pc(), CODE);
    assert_eq!(emu.sp(), STACK);
}

#[test]
fn nested_unix_argument_fault_keeps_the_inner_call_context() {
    let (mut emu, data) = aarch64_env();
    emu.os = OperatingSystem::Linux;
    emu.cfg.linux_real_libc = false;
    let previous = unix::begin_call(&mut emu, "libc.so.6", "outer", CODE);
    let outer = unix::begin_call(&mut emu, "libc.so.6", "printf", CODE + 4);
    put_str(&mut emu, data, "%u%u%u%u%u%u%u%u");
    set_args(&mut emu, &[data]);
    emu.set_sp(u64::MAX);
    linux::gateway(CODE + 4, "libc.so.6", "printf", &mut emu);
    unix::end_call(&mut emu, outer);
    unix::end_call(&mut emu, previous);
    assert_eq!(emu.unix_api.fault.as_ref().unwrap().call.name, "printf");
    assert!(emu.run(None).is_err());
}
