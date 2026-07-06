# rs-header
[![Rust CI](https://github.com/mwemuorg/rs-header/actions/workflows/ci.yaml/badge.svg)](https://github.com/mwemuorg/rs-header/actions/workflows/ci.yaml)

Generic executable-header parsers and loaders, extracted from
[mwemu](https://github.com/mwemuorg/mwemu). Two front-ends share one design:
parsing produces plain data structs, and binding into guest memory happens
through a small backend trait — never a concrete emulator — so any project can
map and relocate a binary into its own address space.

- **`pe`** — PE32/PE64 headers, sections, imports/exports, relocations and
  resources. The loader is generic over [`pe::PeLoader`].
- **`elf`** — ELF32/ELF64 headers, segments, dynamic symbols and relocations
  (x86-64 + AArch64). The loader is generic over [`elf::ElfLoader`].

Philosophy: parsing **borrows** the file bytes and keeps only metadata (no
second copy of the image in RAM); accessors that need the bytes take them as a
`&[u8]` argument. The loaders are generic (`fn …<L: PeLoader>`) so calls are
monomorphized and inlined — no `dyn`, no overhead.

## PE

You provide a `PeLoader` (how to touch your memory and resolve imports); the
crate decides *what* to map and patch.

```rust
use rs_header::pe::PeLoader;
use rs_header::pe::pe64::PE64;

let raw = std::fs::read("sample.exe")?;
let mut pe = PE64::parse("sample.exe", &raw);   // metadata only; bytes borrowed
let base = pe.opt.image_base;

// 1) map the PE headers + each section into *your* memory
backend.write_bytes(base, pe.headers(&raw));
for i in 0..pe.num_of_sections() {
    let sect = pe.get_section(i);
    let dst  = base + sect.virtual_address as u64;
    backend.write_bytes(dst, pe.get_section_ptr(&raw, i));
}

// 2) apply base relocations and bind imports through the trait
pe.apply_relocations(&raw, &mut backend, base);
pe.iat_binding(&raw, &mut backend, base);
pe.delay_load_binding(&raw, &mut backend, base);

// 3) the loaded image now lives only in `backend` — the file bytes can go
drop(raw);

// runtime: name the function behind an IAT address (works for bound *and*
// still-unbound slots — no file bytes needed)
let name = pe.import_addr_to_name(some_call_target);
```

A minimal in-memory backend (this is exactly the shape `libmwemu` implements
over its `Maps`, and what the crate's own tests use):

```rust
use std::collections::HashMap;
use rs_header::pe::PeLoader;

#[derive(Default)]
struct Mem {
    bytes: HashMap<u64, u8>,
    resolved: HashMap<String, u64>,
    next: u64, // bump allocator for fake export addresses
}

impl PeLoader for Mem {
    fn is_mapped(&self, addr: u64) -> bool { self.bytes.contains_key(&addr) }

    fn write_bytes(&mut self, addr: u64, data: &[u8]) -> bool {
        for (i, b) in data.iter().enumerate() { self.bytes.insert(addr + i as u64, *b); }
        true
    }
    fn write_dword(&mut self, addr: u64, v: u32) -> bool {
        self.write_bytes(addr, &v.to_le_bytes())
    }
    fn write_qword(&mut self, addr: u64, v: u64) -> bool {
        self.write_bytes(addr, &v.to_le_bytes())
    }

    fn load_library(&mut self, _lib: &str) -> u64 { 0x1_0000 } // pretend it loaded
    fn resolve_api_name(&mut self, name: &str) -> u64 {
        if let Some(&a) = self.resolved.get(name) { return a; }
        self.next += 0x10;
        let a = 0x7fff_0000_0000 + self.next;
        self.resolved.insert(name.to_string(), a);
        a
    }
    fn resolve_api_name_in_module(&mut self, _m: &str, name: &str) -> u64 {
        self.resolve_api_name(name)
    }
    fn search_api_name(&mut self, name: &str) -> (u64, String, String) {
        (self.resolve_api_name(name), "module.dll".into(), name.into())
    }
}
```

PE32 is identical via `rs_header::pe::pe32::PE32` (32-bit thunks/slots).
Resources: `pe.get_resource(&raw, type_id, name_id, type_name, name)`.

## ELF

ELF exposes a single `load` (segments/sections + symbol table) plus relocation
entry points, generic over `ElfLoader`:

```rust
use rs_header::elf::elf64::Elf64;

let raw = std::fs::read("a.out")?;
let mut elf = Elf64::parse(&raw)?;

// map the image and pull in the dynamic symbol table
elf.load(&mut backend, "a.out", /*is_lib*/ false, /*dynamic*/ true, base);

// apply dynamic relocations against the exports you gathered from the libs
let exports: std::collections::HashMap<String, u64> = /* sym -> addr */ Default::default();
let ifuncs:  std::collections::HashSet<u64>         = Default::default();
let outcome = elf.apply_dynamic_relocations_full(&mut backend, &exports, &ifuncs);
// outcome.unresolved : symbolic imports with no provider
// outcome.irelative  : (slot, resolver) IRELATIVE/ifunc relocs to run & patch

// AArch64 uses the section-based path instead:
// elf.apply_rela_aarch64(&mut backend, &exports);
```

The `ElfLoader` backend is smaller — it maps regions and patches qwords:

```rust
use rs_header::elf::{ElfLoader, Perm};

impl ElfLoader for Mem {
    fn map(&mut self, _name: &str, addr: u64, _size: u64, _perm: Perm) -> Option<u64> {
        Some(addr) // map at the requested address; return where it actually landed
    }
    fn lib_alloc(&mut self, _size: u64) -> Option<u64> { Some(0x7f00_0000_0000) }
    fn write_bytes(&mut self, addr: u64, data: &[u8]) -> bool {
        for (i, b) in data.iter().enumerate() { self.bytes.insert(addr + i as u64, *b); }
        true
    }
    fn read_qword(&self, addr: u64) -> Option<u64> {
        let mut v = 0u64;
        for i in 0..8 { v |= (*self.bytes.get(&(addr + i))? as u64) << (8 * i); }
        Some(v)
    }
    fn write_qword(&mut self, addr: u64, v: u64) -> bool {
        for i in 0..8 { self.bytes.insert(addr + i, (v >> (8 * i)) as u8); }
        true
    }
}
```

ELF32 is the same via `rs_header::elf::elf32::Elf32` (`elf.load(&mut backend)`).

## Format detection

All detectors are **byte-based** and symmetric across PE and ELF: a
`*_machine` function returns the architecture field, plus convenience
predicates.

```rust
use rs_header::pe::{pe_machine_type, IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_I386};
use rs_header::pe::pe64::PE64;
use rs_header::elf::{EM_X86_64};
use rs_header::elf::elf64::Elf64;

let raw = std::fs::read(path)?;

// architecture field (None if not that format):
let pe_arch  = pe_machine_type(&raw);     // Some(IMAGE_FILE_MACHINE_*) | None
let elf_arch = Elf64::elf_machine(&raw);  // Some(EM_*) | None

// convenience predicates:
PE64::is_pe64(&raw);              // 64-bit PE (x86-64 or ARM64)
Elf64::is_elf64(&raw);            // any 64-bit ELF
Elf64::is_elf64_x64(&raw);       // ELF64 + EM_X86_64
if pe_machine_type(&raw) == Some(IMAGE_FILE_MACHINE_AMD64) { /* PE64 x86-64 */ }
```

`libmwemu` implements `PeLoader for Emu` and `ElfLoader for Maps` and drives
both exactly like the snippets above. See [`design/`](../design) for the
extraction architecture.

## License

GPL-3.0-only
