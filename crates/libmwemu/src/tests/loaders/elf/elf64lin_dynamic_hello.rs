use std::sync::atomic::Ordering;

use crate::tests::helpers;
use crate::*;

/// Dynamic Linux x86_64 hello world using `puts`, built from
/// `fixtures/elf64_x86_64_linux_hello.c`.
const ELF64_LINUX_X64_HELLO: &[u8] = include_bytes!("../../fixtures/elf64_x86_64_linux_hello.bin");

#[test]
fn elf64lin_dynamic_hello_puts_bridge() {
    helpers::setup();

    let tmp = std::env::temp_dir().join("mwemu_test_elf64_x86_64_linux_hello.bin");
    std::fs::write(&tmp, ELF64_LINUX_X64_HELLO).unwrap();

    let mut emu = emu64();
    // This test validates the in-Rust libc-hook fallback path specifically, so
    // disable the default real-ld.so/libc emulation here.
    emu.cfg.linux_real_libc = false;
    emu.load_code(tmp.to_str().unwrap());

    assert!(emu.cfg.arch.is_x64());
    assert!(emu.os.is_linux(), "expected Linux loader path");
    assert!(emu.elf64.is_some(), "ELF metadata should be loaded");

    let elf = emu.elf64.as_ref().unwrap();
    assert!(
        elf.needed_libs.iter().any(|lib| lib.contains("libc.so.6")),
        "expected libc.so.6 in DT_NEEDED, got {:?}",
        elf.needed_libs
    );

    let start_main_addr = *elf
        .sym_to_addr
        .get("__libc_start_main")
        .expect("expected __libc_start_main to resolve into the linux stub libc");
    let puts_addr = *elf
        .sym_to_addr
        .get("puts")
        .expect("expected puts to resolve into the linux stub libc");

    let start_main_map = emu
        .maps
        .get_addr_name(start_main_addr)
        .unwrap_or("unmapped");
    let puts_map = emu.maps.get_addr_name(puts_addr).unwrap_or("unmapped");
    assert!(
        start_main_map.contains("libc.so.6"),
        "__libc_start_main should map into libc.so.6, got '{}'",
        start_main_map
    );
    assert!(
        puts_map.contains("libc.so.6"),
        "puts should map into libc.so.6, got '{}'",
        puts_map
    );

    let mut saw_unix_api_break = false;
    for i in 0..32 {
        let rip_before = emu.regs().rip;
        let ok = emu.step();
        assert!(ok, "step {} failed at rip=0x{:x}", i, rip_before);
        assert_ne!(emu.regs().rip, 0, "rip should never fall to 0");

        if emu.force_break {
            saw_unix_api_break = true;
            break;
        }
    }

    assert!(
        saw_unix_api_break,
        "expected to intercept the Linux x86_64 startup/libc path"
    );
    assert_eq!(
        emu.is_running.load(Ordering::Relaxed),
        0,
        "the process should be stopped after __libc_start_main drives main -> exit"
    );
    assert_eq!(
        emu.regs().rdi,
        7,
        "main return value should be forwarded into exit(status)"
    );
}

#[test]
fn guest_exec_starts_amd64_interpreter_with_main_entry_and_headers() {
    use std::collections::BTreeMap;

    use crate::syscall::linux::constants::elf::{AT_BASE, AT_ENTRY, AT_NULL, AT_PHDR};
    use crate::syscall::linux::constants::errors::ENOEXEC;
    use goblin::elf::Elf;
    use goblin::elf::header::{ET_DYN, ET_EXEC};
    use goblin::elf::program_header::{PT_INTERP, PT_LOAD, PT_NULL};

    let program = Elf::parse(ELF64_LINUX_X64_HELLO).unwrap();
    let mut linker_bytes = ELF64_LINUX_X64_HELLO.to_vec();
    linker_bytes[16..18].copy_from_slice(&ET_DYN.to_le_bytes());
    // A standalone interpreter has no PT_INTERP of its own.
    let interp_index = program
        .program_headers
        .iter()
        .position(|p| p.p_type == PT_INTERP)
        .unwrap();
    let offset =
        program.header.e_phoff as usize + interp_index * program.header.e_phentsize as usize;
    linker_bytes[offset..offset + 4].copy_from_slice(&PT_NULL.to_le_bytes());
    let linker = Elf::parse(&linker_bytes).unwrap();

    for kind in [ET_EXEC, ET_DYN] {
        let mut main_bytes = ELF64_LINUX_X64_HELLO.to_vec();
        main_bytes[16..18].copy_from_slice(&kind.to_le_bytes());
        let main = Elf::parse(&main_bytes).unwrap();
        let mut emu = emu64();
        emu.cfg.exe_name = "/app".to_owned();
        emu.cfg.linux_real_libc = false;
        emu.set_linux_files(Vec::new(), Vec::new());
        assert_eq!(
            emu.load_guest_elf64("/app", &main_bytes, &main, None),
            Err(ENOEXEC)
        );
        emu.load_guest_elf64("/app", &main_bytes, &main, Some(&linker_bytes))
            .unwrap();

        let words = |slot: u64| emu.maps.read_qword(emu.sp() + slot * 8).unwrap();
        assert_eq!(words(0), 1);
        let auxv: BTreeMap<_, _> = (4..)
            .step_by(2)
            .map(|i| (words(i), words(i + 1)))
            .take_while(|(tag, _)| *tag != AT_NULL)
            .collect();
        let main_bias = emu.elf_image_base().unwrap();
        assert_eq!(main_bias != 0, kind == ET_DYN);
        assert_eq!(auxv[&AT_ENTRY], main_bias + main.entry);
        let first = main
            .program_headers
            .iter()
            .find(|p| p.p_type == PT_LOAD && p.p_offset == 0)
            .unwrap();
        assert_eq!(
            auxv[&AT_PHDR],
            main_bias + first.p_vaddr + main.header.e_phoff
        );
        let ld_entry = auxv[&AT_BASE] + linker.entry;
        assert_eq!(emu.pc(), ld_entry);
        emu.set_pc(auxv[&AT_ENTRY]);
        assert!(emu.set_rip_with_check(ld_entry, true));
        assert_eq!(emu.pc(), ld_entry);
        assert!(!emu.force_break);
        assert_ne!(auxv[&AT_ENTRY], emu.pc());
        assert!(
            emu.maps
                .get_mem_by_addr(emu.pc())
                .unwrap()
                .permission_at(emu.pc())
                .can_execute()
        );
    }
    let mut emu = emu64();
    emu.load_guest_elf64("/loader", &linker_bytes, &linker, None)
        .unwrap();
    assert_eq!(emu.pc(), emu.elf_image_base().unwrap() + linker.entry);
}
