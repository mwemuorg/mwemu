use crate::api::unix::MemoryAccess;
use rs_header::elf::elf64::Elf64;

use crate::api::{linux, macos};
use crate::arch::OperatingSystem;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::serialization::Serialization;
use crate::tests::helpers;
use crate::windows::constants::memory::LIBS64_MIN;
use crate::{emu_aarch64, emu64};

static ELF64_HELLO: &[u8] = include_bytes!("../fixtures/elf64_x86_64_linux_hello.bin");

const DATA: u64 = 0x30000;
const DESTINATION: u64 = DATA + 0x100;
const STACK: u64 = 0x50000;
const ENTRY: u64 = 0x400000;
const UNMAPPED: u64 = 0xdead0000;
const CANARY: u8 = 0xa5;
const RESULT: u64 = 0x12345678;

fn environment(os: OperatingSystem, aarch64: bool) -> Emu {
    helpers::setup();
    let mut emu = if aarch64 { emu_aarch64() } else { emu64() };
    emu.os = os;
    emu.cfg.linux_real_libc = false;
    emu.cfg.console_enabled = false;
    emu.running_script = true;
    for (name, base) in [("api_data", DATA), ("api_stack", STACK)] {
        emu.maps
            .create_map(name, base, 0x1000, Permission::READ_WRITE)
            .unwrap();
    }
    emu.set_pc(ENTRY);
    emu.set_sp(STACK + 0x800);
    emu
}

fn invoke(emu: &mut Emu, name: &str, arguments: &[u64]) -> u64 {
    if emu.cfg.arch.is_aarch64() {
        emu.regs_aarch64_mut().x[..arguments.len()].copy_from_slice(arguments);
    } else {
        let regs = emu.regs_mut();
        for (destination, value) in [
            &mut regs.rdi,
            &mut regs.rsi,
            &mut regs.rdx,
            &mut regs.rcx,
            &mut regs.r8,
            &mut regs.r9,
        ]
        .into_iter()
        .zip(arguments)
        {
            *destination = *value;
        }
    }
    match emu.os {
        OperatingSystem::MacOS => {
            macos::gateway(ENTRY, "libSystem.B.dylib", &format!("_{name}"), emu)
        }
        _ => linux::gateway(ENTRY, "libc.so.6", name, emu),
    }
    if emu.cfg.arch.is_aarch64() {
        emu.regs_aarch64().x[0]
    } else {
        emu.regs().rax
    }
}

fn bytes(emu: &Emu, address: u64, size: u64) -> Vec<u8> {
    (0..size)
        .map(|index| emu.maps.read_byte(address + index).unwrap())
        .collect()
}

#[test]
fn string_operations_preserve_bytes_and_bounded_reads() {
    for os in [OperatingSystem::Linux, OperatingSystem::MacOS] {
        for arm in [false, true] {
            let mut emu = environment(os, arm);
            let source = [0x80, 0xff, b'A', 0x80, 0];
            assert!(emu.maps.write_bytes(DATA, &source));
            assert_eq!(invoke(&mut emu, "strlen", &[DATA]), 4);
            assert_eq!(invoke(&mut emu, "strnlen", &[DATA, 2]), 2);
            assert_eq!(invoke(&mut emu, "strnlen", &[UNMAPPED, 0]), 0);
            assert!(emu.maps.fill_range(DESTINATION, CANARY, 16));
            assert_eq!(
                invoke(&mut emu, "strcpy", &[DESTINATION, DATA]),
                DESTINATION
            );
            assert_eq!(
                bytes(&emu, DESTINATION, 6),
                [0x80, 0xff, b'A', 0x80, 0, CANARY]
            );
            assert_eq!(invoke(&mut emu, "strcmp", &[DESTINATION, DATA]), 0);
            assert!(emu.maps.write_byte(DESTINATION, 0x7f));
            assert!((invoke(&mut emu, "strcmp", &[DESTINATION, DATA]) as i64) < 0);
            assert_eq!(invoke(&mut emu, "strchr", &[DATA, 0x180]), DATA);
            assert_eq!(invoke(&mut emu, "strrchr", &[DATA, 0x80]), DATA + 3);
            assert_eq!(invoke(&mut emu, "strchr", &[DATA, 0]), DATA + 4);
            assert_eq!(invoke(&mut emu, "strstr", &[DATA, DATA + 1]), DATA + 1);
            assert_eq!(invoke(&mut emu, "strstr", &[UNMAPPED, DATA + 4]), UNMAPPED);
            assert!(
                emu.maps
                    .write_bytes(DESTINATION, &[0x80, 0xff, b'a', 0x80, 0])
            );
            assert_eq!(invoke(&mut emu, "strcasecmp", &[DESTINATION, DATA]), 0);
            assert_eq!(invoke(&mut emu, "strncasecmp", &[UNMAPPED, UNMAPPED, 0]), 0);
            let duplicate = invoke(&mut emu, "strdup", &[DATA]);
            assert_ne!(duplicate, 0);
            assert_eq!(bytes(&emu, duplicate, 5), source);
            invoke(&mut emu, "free", &[duplicate]);

            let edge = DATA + 0x1000 - 2;
            assert!(emu.maps.write_bytes(edge, &[0x81, 0x82]));
            assert_eq!(invoke(&mut emu, "strnlen", &[edge, 2]), 2);
            assert_eq!(invoke(&mut emu, "strncmp", &[edge, edge, 2]), 0);
            assert_eq!(
                invoke(&mut emu, "strncpy", &[DESTINATION, edge, 2]),
                DESTINATION
            );
            assert_eq!(bytes(&emu, DESTINATION, 3), [0x81, 0x82, b'a']);
            assert!(emu.maps.write_bytes(edge, &[0xff, 0]));
            assert_eq!(
                invoke(&mut emu, "strncpy", &[DESTINATION, edge, 5]),
                DESTINATION
            );
            assert_eq!(bytes(&emu, DESTINATION, 5), [0xff, 0, 0, 0, 0]);
            assert_eq!(invoke(&mut emu, "strlcpy", &[DESTINATION, DATA, 3]), 4);
            assert_eq!(bytes(&emu, DESTINATION, 3), [0x80, 0xff, 0]);
            assert_eq!(invoke(&mut emu, "strlcat", &[DESTINATION, DATA, 5]), 6);
            assert_eq!(bytes(&emu, DESTINATION, 5), [0x80, 0xff, 0x80, 0xff, 0]);
            assert!(emu.maps.write_bytes(DESTINATION, b"x\0"));
            assert_eq!(
                invoke(&mut emu, "strncat", &[DESTINATION, DATA, 2]),
                DESTINATION
            );
            assert_eq!(bytes(&emu, DESTINATION, 4), [b'x', 0x80, 0xff, 0]);
            assert_eq!(
                invoke(&mut emu, "strcat", &[DESTINATION, DATA + 4]),
                DESTINATION
            );
            assert!(
                !emu.unix_api.stopped(),
                "{os:?} arm={arm}: {:?}",
                emu.unix_api
            );
        }
    }
}

#[test]
fn memory_operations_handle_overlap_adjacent_maps_and_zero_lengths() {
    const LENGTH: usize = 6000;
    for os in [OperatingSystem::Linux, OperatingSystem::MacOS] {
        for arm in [false, true] {
            let mut emu = environment(os, arm);
            emu.maps
                .create_map(
                    "api_adjacent",
                    DATA + 0x1000,
                    0x1000,
                    Permission::READ_WRITE,
                )
                .unwrap();
            let original: Vec<u8> = (0..LENGTH).map(|index| (index % 251) as u8).collect();
            for backwards in [false, true] {
                assert!(emu.maps.write_bytes(DATA, &original));
                let (source, destination) = if backwards {
                    (DATA, DATA + 31)
                } else {
                    (DATA + 31, DATA)
                };
                let expected = bytes(&emu, source, (LENGTH - 31) as u64);
                assert_eq!(
                    invoke(
                        &mut emu,
                        "memmove",
                        &[destination, source, (LENGTH - 31) as u64]
                    ),
                    destination
                );
                assert_eq!(bytes(&emu, destination, expected.len() as u64), expected);
            }
            assert_eq!(
                invoke(&mut emu, "memcpy", &[UNMAPPED, UNMAPPED, 0]),
                UNMAPPED
            );
            assert_eq!(invoke(&mut emu, "memset", &[UNMAPPED, 42, 0]), UNMAPPED);
            assert_eq!(invoke(&mut emu, "memcmp", &[UNMAPPED, UNMAPPED, 0]), 0);
            assert_eq!(invoke(&mut emu, "memchr", &[UNMAPPED, 42, 0]), 0);
            assert_eq!(invoke(&mut emu, "memset", &[DATA, 0x1ff, 3]), DATA);
            assert_eq!(bytes(&emu, DATA, 3), [0xff; 3]);
            assert_eq!(invoke(&mut emu, "memchr", &[DATA, 0x1ff, 3]), DATA);
            assert!(emu.maps.write_bytes(DATA + 8, &[0x7f; 3]));
            assert!((invoke(&mut emu, "memcmp", &[DATA, DATA + 8, 3]) as i64) > 0);
            invoke(&mut emu, "bzero", &[DATA, 3]);
            assert_eq!(bytes(&emu, DATA, 3), [0; 3]);
            assert!(!emu.unix_api.stopped());
        }
    }
}

#[test]
fn memory_faults_preserve_call_context_outputs_and_snapshots() {
    for os in [OperatingSystem::Linux, OperatingSystem::MacOS] {
        for arm in [false, true] {
            let mut emu = environment(os, arm);
            let edge = DATA + 0x1000 - 2;
            let cases = [
                ("memcpy", vec![DESTINATION, edge, 3], MemoryAccess::Read),
                ("memset", vec![UNMAPPED, 0, 3], MemoryAccess::Write),
                ("memcpy", vec![DESTINATION, u64::MAX, 3], MemoryAccess::Read),
                ("strlen", vec![UNMAPPED], MemoryAccess::Read),
                (
                    "__memcpy_chk",
                    vec![DESTINATION, DATA, 4, 3],
                    MemoryAccess::ObjectBounds,
                ),
                (
                    "__strcpy_chk",
                    vec![DESTINATION, DATA, 3],
                    MemoryAccess::ObjectBounds,
                ),
                (
                    "__strncpy_chk",
                    vec![DESTINATION, DATA, 4, 3],
                    MemoryAccess::ObjectBounds,
                ),
                (
                    "__strlcpy_chk",
                    vec![DESTINATION, DATA, 4, 3],
                    MemoryAccess::ObjectBounds,
                ),
            ];
            for (name, args, access) in cases {
                emu.unix_api.clear_fault();
                emu.force_break = false;
                if !arm {
                    emu.regs_mut().rax = RESULT;
                }
                assert!(emu.maps.write_bytes(DATA, b"abcdef\0"));
                assert!(emu.maps.fill_range(DESTINATION, CANARY, 16));
                let before = (emu.pc(), emu.sp());
                let result = invoke(&mut emu, name, &args);
                let fault = emu.unix_api.memory_fault.clone().expect(name);
                assert_eq!(fault.error.access, access, "{name}");
                assert!(fault.call.name.ends_with(name), "{fault:?}");
                assert_eq!((fault.call.pc, fault.call.stack_pointer), before);
                assert_eq!((emu.pc(), emu.sp()), before);
                assert_eq!(bytes(&emu, DESTINATION, 16), [CANARY; 16]);
                assert_eq!(result, if arm { args[0] } else { RESULT });
                assert!(!emu.step());
                assert!(
                    emu.run(None)
                        .unwrap_err()
                        .to_string()
                        .contains("Unix API memory fault")
                );
                let mut restored = Serialization::deserialize(&Serialization::serialize(&emu));
                assert_eq!(restored.unix_api.memory_fault, Some(fault));
                assert!(!restored.step());
                assert!(restored.run(None).is_err());
            }
        }
    }
}

#[test]
fn intercepted_memory_fault_reaches_run_step_and_threaded_execution() {
    for arm in [false, true] {
        for execution in ["step", "run", "threaded"] {
            let mut emu = environment(OperatingSystem::Linux, arm);
            emu.cfg.enable_threading = execution == "threaded";
            let mut elf = Elf64::parse(ELF64_HELLO).unwrap();
            elf.addr_to_symbol.insert(LIBS64_MIN, "memcpy".into());
            emu.elf64 = Some(elf);
            for (name, base) in [("code", ENTRY), ("libc.so.6.text", LIBS64_MIN)] {
                emu.maps
                    .create_map(name, base, 0x100, Permission::READ_WRITE_EXECUTE)
                    .unwrap();
            }
            emu.maps.fill_range(DESTINATION, CANARY, 16);
            let end = if arm {
                let regs = emu.regs_aarch64_mut();
                regs.x[0] = DESTINATION;
                regs.x[1] = UNMAPPED;
                regs.x[2] = 16;
                regs.x[9] = LIBS64_MIN;
                regs.x[19] = RESULT;
                // blr x9; mov x19, #0; nop
                assert!(emu.maps.write_bytes(
                    ENTRY,
                    &[
                        0x20, 0x01, 0x3f, 0xd6, 0x13, 0, 0x80, 0xd2, 0x1f, 0x20, 0x03, 0xd5
                    ]
                ));
                ENTRY + 8
            } else {
                let regs = emu.regs_mut();
                regs.rdi = DESTINATION;
                regs.rsi = UNMAPPED;
                regs.rdx = 16;
                regs.r11 = LIBS64_MIN;
                regs.rbx = RESULT;
                // call r11; xor ebx, ebx; nop
                assert!(
                    emu.maps
                        .write_bytes(ENTRY, &[0x41, 0xff, 0xd3, 0x31, 0xdb, 0x90])
                );
                ENTRY + 5
            };
            if execution == "step" {
                assert!(!emu.step());
            } else {
                assert!(emu.run(Some(end)).is_err());
            }
            let fault = emu.unix_api.memory_fault.as_ref().unwrap();
            assert_eq!(fault.call.name, "memcpy");
            assert_eq!(fault.error.address, UNMAPPED);
            assert_eq!(fault.error.size, 16);
            assert_eq!(emu.pc(), fault.call.pc);
            assert_eq!(emu.sp(), fault.call.stack_pointer);
            assert_eq!(bytes(&emu, DESTINATION, 16), [CANARY; 16]);
            assert_eq!(
                if arm {
                    emu.regs_aarch64().x[19]
                } else {
                    emu.regs().rbx
                },
                RESULT
            );
        }
    }
}
