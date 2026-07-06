//! ELF parse + generic-load tests. The fixtures are tiny, hand-built ELF
//! images (header + one PT_LOAD program header, no sections) so the tests stay
//! self-contained and exercise the borrow-free `ElfLoader` path without an
//! emulator.

use std::collections::HashMap;

use rs_header::elf::elf32::Elf32;
use rs_header::elf::elf64::Elf64;
use rs_header::elf::{ElfLoader, Perm};

/// In-memory `ElfLoader`: records each map and stores written bytes, so tests
/// can assert what the loader mapped and what landed in "guest" memory.
#[derive(Default)]
struct Mock {
    mem: HashMap<u64, u8>,
    maps: Vec<(String, u64, u64)>,
}

impl Mock {
    fn mapped(&self, name: &str) -> Option<(u64, u64)> {
        self.maps
            .iter()
            .find(|(n, _, _)| n == name)
            .map(|(_, a, s)| (*a, *s))
    }
    fn byte(&self, addr: u64) -> u8 {
        self.mem.get(&addr).copied().unwrap_or(0)
    }
}

impl ElfLoader for Mock {
    fn map(&mut self, name: &str, addr: u64, size: u64, _perm: Perm) -> Option<u64> {
        self.maps.push((name.to_string(), addr, size));
        Some(addr)
    }
    fn lib_alloc(&mut self, _size: u64) -> Option<u64> {
        Some(0x7f00_0000_0000)
    }
    fn write_bytes(&mut self, addr: u64, data: &[u8]) -> bool {
        for (i, b) in data.iter().enumerate() {
            self.mem.insert(addr + i as u64, *b);
        }
        true
    }
    fn read_qword(&self, addr: u64) -> Option<u64> {
        let mut v = 0u64;
        for i in 0..8 {
            v |= (*self.mem.get(&(addr + i))? as u64) << (8 * i);
        }
        Some(v)
    }
    fn write_qword(&mut self, addr: u64, val: u64) -> bool {
        for i in 0..8 {
            self.mem.insert(addr + i, (val >> (8 * i)) as u8);
        }
        true
    }
}

fn put_u16(b: &mut [u8], off: usize, v: u16) {
    b[off..off + 2].copy_from_slice(&v.to_le_bytes());
}
fn put_u32(b: &mut [u8], off: usize, v: u32) {
    b[off..off + 4].copy_from_slice(&v.to_le_bytes());
}
fn put_u64(b: &mut [u8], off: usize, v: u64) {
    b[off..off + 8].copy_from_slice(&v.to_le_bytes());
}

/// 64-byte ELF64 ehdr + one 56-byte PT_LOAD phdr, no sections. `e_machine`
/// 0x3E = x86_64.
fn build_elf64() -> Vec<u8> {
    let mut b = vec![0u8; 64 + 56];
    b[0..4].copy_from_slice(&[0x7f, b'E', b'L', b'F']);
    b[4] = 2; // ELFCLASS64
    b[5] = 1; // little-endian
    b[6] = 1; // EV_CURRENT
    put_u16(&mut b, 16, 2); // e_type = ET_EXEC
    put_u16(&mut b, 18, 0x3E); // e_machine = EM_X86_64
    put_u32(&mut b, 20, 1); // e_version
    put_u64(&mut b, 24, 0x401000); // e_entry
    put_u64(&mut b, 32, 64); // e_phoff
    put_u64(&mut b, 40, 0); // e_shoff
    put_u16(&mut b, 52, 64); // e_ehsize
    put_u16(&mut b, 54, 56); // e_phentsize
    put_u16(&mut b, 56, 1); // e_phnum
    put_u16(&mut b, 58, 64); // e_shentsize
    put_u16(&mut b, 60, 0); // e_shnum
    put_u16(&mut b, 62, 0); // e_shstrndx

    let total = b.len() as u64;
    let p = 64; // program header
    put_u32(&mut b, p, 1); // p_type = PT_LOAD
    put_u32(&mut b, p + 4, 5); // p_flags = R+X
    put_u64(&mut b, p + 8, 0); // p_offset
    put_u64(&mut b, p + 16, 0x400000); // p_vaddr
    put_u64(&mut b, p + 24, 0x400000); // p_paddr
    put_u64(&mut b, p + 32, total); // p_filesz
    put_u64(&mut b, p + 40, 0x1000); // p_memsz
    put_u64(&mut b, p + 48, 0x1000); // p_align
    b
}

/// 52-byte ELF32 ehdr + one 32-byte PT_LOAD phdr, no sections.
fn build_elf32() -> Vec<u8> {
    let mut b = vec![0u8; 52 + 32];
    b[0..4].copy_from_slice(&[0x7f, b'E', b'L', b'F']);
    b[4] = 1; // ELFCLASS32
    b[5] = 1;
    b[6] = 1;
    put_u16(&mut b, 16, 2); // e_type
    put_u16(&mut b, 18, 0x03); // e_machine = EM_386
    put_u32(&mut b, 20, 1);
    put_u32(&mut b, 24, 0x8048000); // e_entry
    put_u32(&mut b, 28, 52); // e_phoff
    put_u32(&mut b, 32, 0); // e_shoff
    put_u16(&mut b, 40, 52); // e_ehsize
    put_u16(&mut b, 42, 32); // e_phentsize
    put_u16(&mut b, 44, 1); // e_phnum
    put_u16(&mut b, 46, 40); // e_shentsize
    put_u16(&mut b, 48, 0); // e_shnum
    put_u16(&mut b, 50, 0); // e_shstrndx

    let total = b.len() as u32;
    let p = 52;
    put_u32(&mut b, p, 1); // p_type = PT_LOAD
    put_u32(&mut b, p + 4, 0); // p_offset
    put_u32(&mut b, p + 8, 0x8048000); // p_vaddr
    put_u32(&mut b, p + 12, 0x8048000); // p_paddr
    put_u32(&mut b, p + 16, total); // p_filesz
    put_u32(&mut b, p + 20, 0x1000); // p_memsz
    put_u32(&mut b, p + 24, 5); // p_flags = R+X
    put_u32(&mut b, p + 28, 0x1000); // p_align
    b
}

#[test]
fn parse_elf64() {
    let raw = build_elf64();
    let elf = Elf64::parse(&raw).expect("parse elf64");

    assert_eq!(&elf.elf_hdr.e_ident[..4], &[0x7f, b'E', b'L', b'F']);
    assert_eq!(elf.elf_hdr.e_machine, 0x3E);
    assert_eq!(elf.elf_hdr.e_entry, 0x401000);
    assert_eq!(elf.elf_phdr.len(), 1);
    assert_eq!(elf.elf_phdr[0].p_type, 1);
    assert_eq!(elf.elf_phdr[0].p_vaddr, 0x400000);
    assert!(elf.is_static(), "no PT_DYNAMIC => static");
}

#[test]
fn detect_elf64_machine() {
    let raw = build_elf64();
    assert!(Elf64::is_elf64_x64(&raw));
    assert!(!Elf64::is_elf64_aarch64(&raw));
    assert!(!Elf64::is_elf64_x64(&raw[..10])); // too short to identify
}

#[test]
fn load_elf64_static_maps_header() {
    let raw = build_elf64();
    let mut elf = Elf64::parse(&raw).expect("parse");
    let mut mock = Mock::default();

    // force_base != CFG_DEFAULT_BASE => the loader maps at exactly force_base.
    elf.load(&mut mock, "test", false, false, 0x400000);

    assert_eq!(elf.base, 0x400000);
    let (addr, size) = mock.mapped("elf64.hdr").expect("header map created");
    assert_eq!(addr, 0x400000);
    assert_eq!(size, 512);
    // ELF magic landed in guest memory at the base.
    assert_eq!(mock.byte(0x400000), 0x7f);
    assert_eq!(mock.byte(0x400003), b'F');
}

#[test]
fn parse_and_load_elf32() {
    let raw = build_elf32();
    assert!(Elf32::is_elf32(&raw));

    let mut elf = Elf32::parse(&raw).expect("parse elf32");
    let mut mock = Mock::default();
    elf.load(&mut mock);

    assert_eq!(elf.elf_phdr.len(), 1);
    assert_eq!(elf.base(), 0, "non-dynamic elf32 loads at vaddr as-is");
    let (addr, _size) = mock.mapped("elf32_seg0").expect("segment mapped");
    assert_eq!(addr, 0x8048000);
    // Segment contents (the ELF header bytes) were written at the vaddr.
    assert_eq!(mock.byte(0x8048000), 0x7f);
}
