use crate::arch::Arch;
use crate::maps::mem64::Permission;
use crate::serialization::Serialization;
use crate::tests::helpers;
use minidump::{Minidump, MinidumpModuleList, Module};
use tempdir::TempDir;

const ELF64_AARCH64_ADD: &[u8] = include_bytes!("../fixtures/elf64_aarch64_add.bin");

fn write_tmp(name: &str, bytes: &[u8]) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(name);
    std::fs::write(&path, bytes).unwrap();
    path
}

#[test]
fn test_x86_64_native_serialization_roundtrip() {
    let handle = std::thread::Builder::new()
        .stack_size(1024 * 29055)
        .spawn(|| {
            helpers::setup();

            let mut emu = crate::emu64();
            emu.regs_mut().rax = 0x1122_3344_5566_7788;
            emu.regs_mut().rbx = 0xAABB_CCDD_EEFF_0011;
            emu.regs_mut().rsp = 0x200800;
            emu.regs_mut().rip = 0x401000;
            emu.flags_mut().load(0x246);

            let stack_map = emu
                .maps
                .create_map("stack", 0x200000, 0x2000, Permission::READ_WRITE)
                .unwrap();
            stack_map.memcpy(&vec![0x41; 0x200], 0x200);

            let serialized = Serialization::serialize(&emu);
            let loaded = Serialization::deserialize(&serialized);

            assert!(loaded.cfg.is_x64());
            assert_eq!(loaded.regs().rax, 0x1122_3344_5566_7788);
            assert_eq!(loaded.regs().rbx, 0xAABB_CCDD_EEFF_0011);
            assert_eq!(loaded.regs().rsp, 0x200800);
            assert_eq!(loaded.regs().rip, 0x401000);
            assert_eq!(loaded.flags_snapshot().dump(), 0x246);
            assert_eq!(loaded.maps.read_byte(0x200010), Some(0x41));
        })
        .unwrap();
    handle.join().unwrap();
}

#[test]
fn test_emu_config_preserved() {
    let mut emu = crate::emu64();

    emu.cfg.verbose = 3;
    emu.cfg.arch = Arch::X86_64;

    // Verify config can be set
    assert_eq!(emu.cfg.verbose, 3);
    assert_eq!(emu.cfg.is_x64(), true);
}

#[test]
fn test_emu_registers_basic() {
    let mut emu = crate::emu64();

    // Set up some state
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rbx = 0xFEDCBA9876543210;
    emu.regs_mut().rip = 0x400000;

    // Verify state
    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0);
    assert_eq!(emu.regs().rbx, 0xFEDCBA9876543210);
    assert_eq!(emu.regs().rip, 0x400000);
}

#[test]
fn test_emu_32bit_registers() {
    let mut emu = crate::emu32();

    emu.regs_mut().set_eax(0x12345678);
    emu.regs_mut().set_ebx(0xABCDEF00);

    assert_eq!(emu.regs().get_eax(), 0x12345678);
    assert_eq!(emu.regs().get_ebx(), 0xABCDEF00);
}

fn build_test_pe_header(machine: u16) -> Vec<u8> {
    let mut raw = vec![0u8; 0x200];
    raw[0] = b'M';
    raw[1] = b'Z';
    raw[0x3c..0x40].copy_from_slice(&0x80u32.to_le_bytes());
    raw[0x80..0x84].copy_from_slice(b"PE\0\0");
    raw[0x84..0x86].copy_from_slice(&machine.to_le_bytes());
    raw
}

#[test]
fn test_x64_minidump_roundtrip() {
    let temp_dir = TempDir::new("mwemu_minidump_x64").unwrap();
    let dump_path = temp_dir.path().join("sample64.dmp");

    let mut emu = crate::emu64();
    emu.filename = "sample.exe".to_string();
    emu.cfg.exe_name = "sample.exe".to_string();

    let header = build_test_pe_header(0x8664);
    let pe_map = emu
        .maps
        .create_map("sample.pe", 0x400000, 0x1000, Permission::READ_WRITE)
        .unwrap();
    pe_map.memcpy(&header, header.len());

    let text_map = emu
        .maps
        .create_map("sample.text", 0x401000, 0x1000, Permission::READ_EXECUTE)
        .unwrap();
    text_map.memcpy(&vec![0x90; 0x100], 0x100);

    let stack_map = emu
        .maps
        .create_map("stack", 0x200000, 0x2000, Permission::READ_WRITE)
        .unwrap();
    stack_map.memcpy(&vec![0x41; 0x200], 0x200);

    emu.regs_mut().rax = 0x1122_3344_5566_7788;
    emu.regs_mut().rbp = 0x200900;
    emu.regs_mut().rsp = 0x200800;
    emu.regs_mut().rip = 0x401020;
    emu.flags_mut().load(0x246);

    Serialization::dump_to_minidump(&emu, dump_path.to_str().unwrap()).unwrap();

    let dump = Minidump::read_path(&dump_path).unwrap();
    let modules = dump.get_stream::<MinidumpModuleList>().unwrap();
    assert_eq!(modules.iter().count(), 1);
    let module = modules.iter().next().unwrap();
    assert_eq!(module.base_address(), 0x400000);
    assert_eq!(module.name.to_string(), "sample.exe");

    let loaded = Serialization::load_from_minidump(dump_path.to_str().unwrap());
    assert!(loaded.cfg.is_x64());
    assert_eq!(loaded.regs().rax, 0x1122_3344_5566_7788);
    assert_eq!(loaded.regs().rbp, 0x200900);
    assert_eq!(loaded.regs().rsp, 0x200800);
    assert_eq!(loaded.regs().rip, 0x401020);
    assert_eq!(loaded.flags_snapshot().dump(), 0x246);
    assert_eq!(loaded.maps.read_byte(0x200010), Some(0x41));
    assert!(loaded.pe64.is_some());
}

#[test]
fn test_x86_minidump_roundtrip() {
    let temp_dir = TempDir::new("mwemu_minidump_x86").unwrap();
    let dump_path = temp_dir.path().join("sample32.dmp");

    let mut emu = crate::emu32();
    emu.filename = "sample32.exe".to_string();
    emu.cfg.exe_name = "sample32.exe".to_string();

    let header = build_test_pe_header(0x014c);
    let pe_map = emu
        .maps
        .create_map("sample32.pe", 0x0040_0000, 0x1000, Permission::READ_WRITE)
        .unwrap();
    pe_map.memcpy(&header, header.len());

    let stack_map = emu
        .maps
        .create_map("stack", 0x0010_0000, 0x2000, Permission::READ_WRITE)
        .unwrap();
    stack_map.memcpy(&vec![0x24; 0x200], 0x200);

    emu.regs_mut().set_eax(0x1234_5678);
    emu.regs_mut().set_ebp(0x0010_0900);
    emu.regs_mut().set_esp(0x0010_0800);
    emu.regs_mut().set_eip(0x0040_1020);
    emu.flags_mut().load(0x202);

    Serialization::dump_to_minidump(&emu, dump_path.to_str().unwrap()).unwrap();

    let loaded = Serialization::load_from_minidump(dump_path.to_str().unwrap());
    assert!(!loaded.cfg.is_x64());
    assert_eq!(loaded.regs().get_eax(), 0x1234_5678);
    assert_eq!(loaded.regs().get_ebp(), 0x0010_0900);
    assert_eq!(loaded.regs().get_esp(), 0x0010_0800);
    assert_eq!(loaded.regs().get_eip(), 0x0040_1020);
    assert_eq!(loaded.flags_snapshot().dump(), 0x202);
    assert_eq!(loaded.maps.read_byte(0x0010_0010), Some(0x24));
    assert!(loaded.pe32.is_some());
}

#[test]
fn test_aarch64_minidump_roundtrip() {
    let temp_dir = TempDir::new("mwemu_minidump_aarch64_rt").unwrap();
    let dump_path = temp_dir.path().join("sample_aarch64.dmp");

    let mut emu = crate::emu_aarch64();
    emu.init_cpu();
    emu.filename = "sample_aarch64.exe".to_string();

    emu.regs_aarch64_mut().x[0] = 0x1122_3344_5566_7788;
    emu.regs_aarch64_mut().x[1] = 0xAABB_CCDD_EEFF_0011;
    emu.regs_aarch64_mut().x[29] = 0x200900; // fp
    emu.regs_aarch64_mut().x[30] = 0x401004; // lr
    emu.regs_aarch64_mut().sp = 0x200800;
    emu.regs_aarch64_mut().pc = 0x401000;
    emu.regs_aarch64_mut().nzcv.n = true;
    emu.regs_aarch64_mut().nzcv.z = false;
    emu.regs_aarch64_mut().nzcv.c = true;
    emu.regs_aarch64_mut().nzcv.v = false;
    emu.regs_aarch64_mut().v[0] = 0x0011_2233_4455_6677_8899_AABB_CCDD_EEFF;

    let stack_map = emu
        .maps
        .create_map("test_stack", 0x200000, 0x2000, Permission::READ_WRITE)
        .unwrap();
    stack_map.memcpy(&vec![0x55; 0x200], 0x200);

    Serialization::dump_to_minidump(&emu, dump_path.to_str().unwrap()).unwrap();

    // Validate dump is parseable by the minidump crate
    let dump = Minidump::read_path(&dump_path).unwrap();
    let sys_info = dump.get_stream::<minidump::MinidumpSystemInfo>().unwrap();
    assert!(matches!(sys_info.cpu, minidump::system_info::Cpu::Arm64));

    // Import back and verify register state
    let loaded = Serialization::load_from_minidump(dump_path.to_str().unwrap());
    assert!(loaded.cfg.arch.is_aarch64());
    assert_eq!(loaded.regs_aarch64().x[0], 0x1122_3344_5566_7788);
    assert_eq!(loaded.regs_aarch64().x[1], 0xAABB_CCDD_EEFF_0011);
    assert_eq!(loaded.regs_aarch64().x[29], 0x200900);
    assert_eq!(loaded.regs_aarch64().x[30], 0x401004);
    assert_eq!(loaded.regs_aarch64().sp, 0x200800);
    assert_eq!(loaded.regs_aarch64().pc, 0x401000);
    assert!(loaded.regs_aarch64().nzcv.n);
    assert!(!loaded.regs_aarch64().nzcv.z);
    assert!(loaded.regs_aarch64().nzcv.c);
    assert!(!loaded.regs_aarch64().nzcv.v);
    assert_eq!(
        loaded.regs_aarch64().v[0],
        0x0011_2233_4455_6677_8899_AABB_CCDD_EEFF
    );
    assert_eq!(loaded.maps.read_byte(0x200010), Some(0x55));
}

#[test]
fn test_aarch64_native_serialization_fixture_roundtrip() {
    let handle = std::thread::Builder::new()
        .stack_size(1024 * 29055)
        .spawn(|| {
            helpers::setup();

            let path = write_tmp(
                "mwemu_test_serialize_elf64_aarch64_add.bin",
                ELF64_AARCH64_ADD,
            );

            let mut emu = crate::emu_aarch64();
            emu.load_code(path.to_str().unwrap());

            emu.step();
            emu.step();
            emu.step();
            assert_eq!(emu.regs_aarch64().x[2], 2);

            let serialized = Serialization::serialize(&emu);
            let loaded = Serialization::deserialize(&serialized);

            assert!(loaded.cfg.arch.is_aarch64());
            assert_eq!(loaded.regs_aarch64().x[2], 2);
            assert_eq!(loaded.regs_aarch64().pc, emu.regs_aarch64().pc);
        })
        .unwrap();
    handle.join().unwrap();
}

#[test]
fn test_aarch64_minidump_fixture_roundtrip() {
    let handle = std::thread::Builder::new()
        .stack_size(1024 * 29055)
        .spawn(|| {
            helpers::setup();

            let sample_path = write_tmp(
                "mwemu_test_minidump_elf64_aarch64_add.bin",
                ELF64_AARCH64_ADD,
            );
            let temp_dir = TempDir::new("mwemu_minidump_aarch64_future").unwrap();
            let dump_path = temp_dir.path().join("elf64_aarch64_add.dmp");

            let mut emu = crate::emu_aarch64();
            emu.load_code(sample_path.to_str().unwrap());

            emu.step();
            emu.step();
            emu.step();
            assert_eq!(emu.regs_aarch64().x[2], 2);

            Serialization::dump_to_minidump(&emu, dump_path.to_str().unwrap()).unwrap();
            let loaded = Serialization::load_from_minidump(dump_path.to_str().unwrap());

            assert!(loaded.cfg.arch.is_aarch64());
            assert_eq!(loaded.regs_aarch64().x[2], 2);
            assert_eq!(loaded.regs_aarch64().pc, emu.regs_aarch64().pc);
        })
        .unwrap();
    handle.join().unwrap();
}
