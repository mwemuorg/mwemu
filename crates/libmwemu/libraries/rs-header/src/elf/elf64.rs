use crate::elf::loader::{ElfLoader, Perm};
use crate::elf::ElfError;
use std::collections::HashMap;

// Load addresses and segment type previously sourced from
// `libmwemu::windows::constants`. Kept here so the parser is self-contained.
const PT_LOAD: u32 = 1;
const CFG_DEFAULT_BASE: u64 = 0x3c0000;
const ELF64_DYN_BASE: u64 = 0x555555554000;
const ELF64_STA_BASE: u64 = 0x400000;

macro_rules! read_u8 {
    ($raw:expr, $off:expr) => {
        $raw[$off]
    };
}

macro_rules! read_u16_le {
    ($raw:expr, $off:expr) => {
        (($raw[$off + 1] as u16) << 8) | ($raw[$off] as u16)
    };
}

macro_rules! read_u32_le {
    ($raw:expr, $off:expr) => {
        (($raw[$off + 3] as u32) << 24)
            | (($raw[$off + 2] as u32) << 16)
            | (($raw[$off + 1] as u32) << 8)
            | ($raw[$off] as u32)
    };
}

macro_rules! read_u64_le {
    ($raw:expr, $off:expr) => {
        (($raw[$off + 7] as u64) << 56)
            | (($raw[$off + 6] as u64) << 48)
            | (($raw[$off + 5] as u64) << 40)
            | (($raw[$off + 4] as u64) << 32)
            | (($raw[$off + 3] as u64) << 24)
            | (($raw[$off + 2] as u64) << 16)
            | (($raw[$off + 1] as u64) << 8)
            | ($raw[$off] as u64)
    };
}

/*
macro_rules! write_u64_le {
    ($raw:expr, $off:expr, $val:expr) => {
        $raw[$off] = $val as u8;
        $raw[$off + 1] = ($val >> 8) as u8;
        $raw[$off + 2] = ($val >> 16) as u8;
        $raw[$off + 3] = ($val >> 24) as u8;
        $raw[$off + 4] = ($val >> 32) as u8;
        $raw[$off + 5] = ($val >> 40) as u8;
        $raw[$off + 6] = ($val >> 48) as u8;
        $raw[$off + 7] = ($val >> 56) as u8;
    };
}*/

pub const EI_NIDENT: usize = 16;
pub const ELFCLASS64: u8 = 0x02;
// `e_machine` (architecture) values, mirroring PE's IMAGE_FILE_MACHINE_*.
pub const EM_386: u16 = 0x03;
pub const EM_X86_64: u16 = 0x3E;
pub const EM_AARCH64: u16 = 0xB7;
pub const DT_NEEDED: u64 = 1;
pub const DT_PLTRELSZ: u64 = 2;
pub const DT_NULL: u64 = 0;
pub const DT_SYMTAB: u64 = 6;
pub const DT_RELA: u64 = 7;
pub const DT_RELASZ: u64 = 8;
pub const DT_RELAENT: u64 = 9;
pub const DT_STRTAB: u64 = 5;
pub const DT_SYMENT: u64 = 11;
pub const DT_JMPREL: u64 = 23;
pub const PT_DYNAMIC: u32 = 2;
pub const STT_FUNC: u8 = 2;
pub const STT_OBJECT: u8 = 1;
pub const R_X86_64_64: u32 = 1;
pub const R_X86_64_GLOB_DAT: u32 = 6;
pub const R_X86_64_JUMP_SLOT: u32 = 7;
pub const R_X86_64_RELATIVE: u32 = 8;
pub const R_X86_64_IRELATIVE: u32 = 37;
pub const STT_GNU_IFUNC: u8 = 10;
pub const R_AARCH64_GLOB_DAT: u32 = 1025;
pub const R_AARCH64_JUMP_SLOT: u32 = 1026;

#[derive(Debug)]
pub struct Elf64 {
    pub base: u64,
    pub bin: Vec<u8>,
    pub elf_hdr: Elf64Ehdr,
    pub elf_phdr: Vec<Elf64Phdr>,
    pub elf_shdr: Vec<Elf64Shdr>,
    pub elf_strtab: Vec<u8>, // no sense, use offset instead repeat the blob
    pub init: Option<u64>,
    pub elf_dynsym: Vec<Elf64Sym>,
    pub elf_dynstr_off: u64,
    pub elf_got_off: u64,
    pub needed_libs: Vec<String>,
    pub sym_to_addr: HashMap<String, u64>,
    pub addr_to_symbol: HashMap<u64, String>,
    /// When true, `load()` maps the image by PT_LOAD segments (covering the ELF
    /// header and program headers, exactly like the kernel/mmap) instead of by
    /// sections. Required to run the real ld.so / libc code.
    pub segment_mode: bool,
}

impl Elf64 {
    /// Parse an ELF64 image from its raw bytes. The struct keeps an owned copy
    /// of the bytes (relocations and symbol resolution read them at load time).
    pub fn parse(raw: &[u8]) -> Result<Elf64, ElfError> {
        if raw.len() < 64 {
            return Err(ElfError::new("elf64 image too small for its header"));
        }
        let bin = raw.to_vec();

        let ehdr: Elf64Ehdr = Elf64Ehdr::parse(&bin);
        let mut ephdr: Vec<Elf64Phdr> = Vec::new();
        let mut eshdr: Vec<Elf64Shdr> = Vec::new();
        let mut off = ehdr.e_phoff as usize;
        let dynsym: Vec<Elf64Sym> = Vec::new();

        // loading programs
        for _ in 0..ehdr.e_phnum {
            let phdr: Elf64Phdr = Elf64Phdr::parse(&bin, off);
            ephdr.push(phdr);
            off += ehdr.e_phentsize as usize;
        }

        off = ehdr.e_shoff as usize;

        // loading sections
        for _ in 0..ehdr.e_shnum {
            let shdr: Elf64Shdr = Elf64Shdr::parse(&bin, off);
            eshdr.push(shdr);
            off += ehdr.e_shentsize as usize;
        }

        let mut off_strtab: usize = 0;
        let mut sz_strtab: usize = 0;
        if (ehdr.e_shstrndx as usize) < eshdr.len() {
            off_strtab = eshdr[ehdr.e_shstrndx as usize].sh_offset as usize;
            sz_strtab = eshdr[ehdr.e_shstrndx as usize].sh_size as usize;
        }
        let mut blob_strtab: Vec<u8> = vec![];
        if off_strtab > 0 {
            blob_strtab = bin[off_strtab..(off_strtab + sz_strtab)].to_vec();
        }
        Ok(Elf64 {
            base: 0,
            bin,
            elf_hdr: ehdr,
            elf_phdr: ephdr,
            elf_shdr: eshdr,
            elf_strtab: blob_strtab,
            init: None,
            elf_dynsym: dynsym,
            elf_dynstr_off: 0,
            elf_got_off: 0,
            needed_libs: Vec::new(),
            sym_to_addr: HashMap::new(),
            addr_to_symbol: HashMap::new(),
            segment_mode: false,
        })
    }

    pub fn is_static(&self) -> bool {
        let is_dynamic = self.elf_phdr.iter().any(|ph| ph.p_type == PT_DYNAMIC);
        !is_dynamic
    }

    pub fn is_loadable(&self, addr: u64) -> bool {
        for phdr in &self.elf_phdr {
            if phdr.p_type == PT_LOAD
                && phdr.p_vaddr > 0
                && (phdr.p_vaddr <= addr || addr <= (phdr.p_vaddr + phdr.p_memsz))
            {
                //log::trace!("vaddr 0x{:x}", phdr.p_vaddr);
                return true;
            }
        }
        false
    }

    pub fn get_section_name(&self, offset: usize) -> String {
        let end = self.elf_strtab[offset..]
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(self.elf_strtab.len() - offset);
        let s = std::str::from_utf8(&self.elf_strtab[offset..offset + end])
            .expect("error reading elf64 shstrtab");
        s.to_string()
    }

    pub fn sym_get_addr_from_name(&self, name: &str) -> Option<u64> {
        self.sym_to_addr.get(name).copied().or_else(|| {
            self.elf_dynsym
                .iter()
                .find(|sym| sym.st_dynstr_name == name && sym.st_value > 0)
                .map(|sym| sym.st_value)
        })
    }

    pub fn sym_get_name_from_addr(&self, addr: u64) -> String {
        self.addr_to_symbol
            .get(&addr)
            .cloned()
            .or_else(|| {
                self.elf_dynsym
                    .iter()
                    .find(|sym| sym.st_value == addr)
                    .map(|sym| sym.st_dynstr_name.clone())
            })
            .unwrap_or_default()
    }

    pub fn exported_symbols(&self) -> Vec<(String, u64)> {
        let mut exports = Vec::new();
        for sym in &self.elf_dynsym {
            if !sym.st_dynstr_name.is_empty() && sym.st_value > 0 {
                exports.push((sym.st_dynstr_name.clone(), sym.st_value));
            }
        }
        exports
    }

    /// Addresses of exported STT_GNU_IFUNC resolvers. A relocation pointing at
    /// one of these must run the resolver to obtain the real implementation
    /// instead of being written directly.
    pub fn ifunc_resolver_addrs(&self) -> std::collections::HashSet<u64> {
        let mut set = std::collections::HashSet::new();
        for sym in &self.elf_dynsym {
            if sym.st_value > 0 && sym.get_st_type() == STT_GNU_IFUNC {
                set.insert(sym.st_value);
            }
        }
        set
    }

    pub fn rebase_vaddr(&self, vaddr: u64) -> u64 {
        if vaddr == 0 {
            0
        } else if vaddr < self.base {
            vaddr + self.base
        } else {
            vaddr
        }
    }

    fn image_size(&self) -> u64 {
        let mut max_end = 0;
        for phdr in &self.elf_phdr {
            if phdr.p_type == PT_LOAD {
                max_end = max_end.max(phdr.p_vaddr + phdr.p_memsz);
            }
        }

        if max_end == 0 {
            for shdr in &self.elf_shdr {
                max_end = max_end.max(shdr.sh_addr + shdr.sh_size);
            }
        }

        max_end.max(0x4000)
    }

    fn read_c_string(&self, offset: usize) -> Option<String> {
        if offset >= self.bin.len() {
            return None;
        }
        let end = self.bin[offset..]
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(self.bin.len() - offset);
        std::str::from_utf8(&self.bin[offset..offset + end])
            .ok()
            .map(|s| s.to_string())
    }

    fn dynamic_table_bounds(&self) -> Option<(usize, usize)> {
        for shdr in &self.elf_shdr {
            if self.get_section_name(shdr.sh_name as usize) == ".dynamic" {
                return Some((shdr.sh_offset as usize, shdr.sh_size as usize));
            }
        }

        for phdr in &self.elf_phdr {
            if phdr.p_type == PT_DYNAMIC {
                return Some((phdr.p_offset as usize, phdr.p_filesz as usize));
            }
        }

        None
    }

    fn vaddr_to_file_offset(&self, vaddr: u64) -> Option<usize> {
        for phdr in &self.elf_phdr {
            if phdr.p_type != PT_LOAD {
                continue;
            }

            let end = phdr.p_vaddr.saturating_add(phdr.p_filesz.max(phdr.p_memsz));
            if vaddr >= phdr.p_vaddr && vaddr < end {
                return Some((phdr.p_offset + (vaddr - phdr.p_vaddr)) as usize);
            }
        }

        for shdr in &self.elf_shdr {
            if shdr.sh_addr == 0 || shdr.sh_size == 0 {
                continue;
            }
            let end = shdr.sh_addr.saturating_add(shdr.sh_size);
            if vaddr >= shdr.sh_addr && vaddr < end {
                return Some((shdr.sh_offset + (vaddr - shdr.sh_addr)) as usize);
            }
        }

        None
    }

    pub fn dynamic_info(&self) -> Option<Elf64DynamicInfo> {
        let (mut off, size) = self.dynamic_table_bounds()?;
        let end = off.saturating_add(size).min(self.bin.len());
        let mut info = Elf64DynamicInfo::default();
        let mut needed_offsets = Vec::new();

        while off + 16 <= end {
            let d_tag = read_u64_le!(self.bin, off);
            let d_val = read_u64_le!(self.bin, off + 8);

            if d_tag == DT_NULL {
                break;
            }

            match d_tag {
                DT_NEEDED => needed_offsets.push(d_val),
                DT_STRTAB => info.strtab_addr = d_val,
                DT_SYMTAB => info.symtab_addr = d_val,
                DT_SYMENT => info.syment = d_val,
                DT_RELA => info.rela_addr = d_val,
                DT_RELASZ => info.rela_size = d_val,
                DT_RELAENT => info.rela_ent = d_val,
                DT_JMPREL => info.jmprel_addr = d_val,
                DT_PLTRELSZ => info.pltrelsz = d_val,
                _ => {}
            }

            off += 16;
        }

        let strtab_off = self
            .vaddr_to_file_offset(info.strtab_addr)
            .or_else(|| usize::try_from(info.strtab_addr).ok())
            .filter(|off| *off < self.bin.len())?;

        for needed_off in needed_offsets {
            let name_off = strtab_off.saturating_add(needed_off as usize);
            if let Some(name) = self.read_c_string(name_off) {
                info.needed.push(name);
            }
        }

        Some(info)
    }

    pub fn apply_dynamic_relocations<L: ElfLoader>(
        &self,
        loader: &mut L,
        export_map: &HashMap<String, u64>,
    ) -> Vec<String> {
        let empty = std::collections::HashSet::new();
        self.apply_dynamic_relocations_full(loader, export_map, &empty)
            .unresolved
    }

    /// Apply all dynamic relocations and report both the unresolved symbolic
    /// imports and the IRELATIVE (ifunc) relocations, which can only be
    /// completed once the emulator is able to *run* the resolver function.
    /// `ifunc_resolvers` is the set of resolver addresses across all loaded
    /// libraries; symbolic relocations that land on one are deferred too.
    pub fn apply_dynamic_relocations_full<L: ElfLoader>(
        &self,
        loader: &mut L,
        export_map: &HashMap<String, u64>,
        ifunc_resolvers: &std::collections::HashSet<u64>,
    ) -> RelocationOutcome {
        let mut outcome = RelocationOutcome::default();
        let Some(info) = self.dynamic_info() else {
            return outcome;
        };

        let rela_ent = if info.rela_ent == 0 {
            Elf64Rela::size()
        } else {
            info.rela_ent as usize
        };

        self.apply_rela_table(
            loader,
            export_map,
            ifunc_resolvers,
            info.rela_addr,
            info.rela_size,
            rela_ent,
            &mut outcome,
        );

        self.apply_rela_table(
            loader,
            export_map,
            ifunc_resolvers,
            info.jmprel_addr,
            info.pltrelsz,
            rela_ent,
            &mut outcome,
        );

        outcome
    }

    /// Patch a relocation slot. The consumer's `write_qword` is expected to
    /// handle writing into otherwise read-only maps; we only report a genuinely
    /// unmapped target.
    fn write_reloc_qword<L: ElfLoader>(&self, loader: &mut L, patch_addr: u64, value: u64, what: &str) {
        if !loader.write_qword(patch_addr, value) {
            log::warn!(
                "elf64: relocation target 0x{:x} for {} is not mapped",
                patch_addr, what
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn apply_rela_table<L: ElfLoader>(
        &self,
        loader: &mut L,
        export_map: &HashMap<String, u64>,
        ifunc_resolvers: &std::collections::HashSet<u64>,
        rela_addr: u64,
        rela_size: u64,
        rela_ent: usize,
        outcome: &mut RelocationOutcome,
    ) {
        if rela_addr == 0 || rela_size == 0 || rela_ent == 0 {
            return;
        }

        let Some(mut off) = self.vaddr_to_file_offset(rela_addr) else {
            log::warn!("elf64: could not translate rela addr 0x{:x}", rela_addr);
            return;
        };
        let end = off.saturating_add(rela_size as usize).min(self.bin.len());

        while off + rela_ent <= end {
            let rela = Elf64Rela::parse(&self.bin, off);
            let r_type = rela.r_type();
            let patch_addr = self.rebase_vaddr(rela.r_offset);

            // Base-relative relocations carry no symbol: *patch = base + addend.
            // These are what fill libc's own internal data pointers, so they are
            // essential when executing the real libc code.
            if r_type == R_X86_64_RELATIVE {
                let value = self.base.wrapping_add(rela.r_addend as u64);
                self.write_reloc_qword(loader, patch_addr, value, "RELATIVE");
                off += rela_ent;
                continue;
            }

            // IRELATIVE: the resolver lives at base+addend and must be executed
            // to obtain the real implementation address. Defer to the caller.
            if r_type == R_X86_64_IRELATIVE {
                let resolver = self.base.wrapping_add(rela.r_addend as u64);
                outcome.irelative.push((patch_addr, resolver));
                off += rela_ent;
                continue;
            }

            if r_type != R_X86_64_GLOB_DAT
                && r_type != R_X86_64_JUMP_SLOT
                && r_type != R_X86_64_64
            {
                off += rela_ent;
                continue;
            }

            let sym_idx = rela.r_sym() as usize;
            let Some(sym) = self.elf_dynsym.get(sym_idx) else {
                off += rela_ent;
                continue;
            };

            if sym.st_dynstr_name.is_empty() {
                off += rela_ent;
                continue;
            }

            let resolved = export_map
                .get(&sym.st_dynstr_name)
                .copied()
                .or_else(|| self.sym_get_addr_from_name(&sym.st_dynstr_name));

            let Some(mut target_addr) = resolved else {
                if !outcome.unresolved.contains(&sym.st_dynstr_name) {
                    outcome.unresolved.push(sym.st_dynstr_name.clone());
                }
                off += rela_ent;
                continue;
            };

            if r_type == R_X86_64_64 {
                target_addr = target_addr.wrapping_add(rela.r_addend as u64);
            }

            // If the resolved symbol is an ifunc, the address we have is its
            // resolver — defer so the emulator can run it and patch the slot
            // with the real implementation.
            if ifunc_resolvers.contains(&target_addr) {
                outcome.irelative.push((patch_addr, target_addr));
                off += rela_ent;
                continue;
            }

            self.write_reloc_qword(loader, patch_addr, target_addr, &sym.st_dynstr_name);

            off += rela_ent;
        }
    }

    /// Apply AArch64 relocations (.rela.dyn and .rela.plt) using section headers.
    /// Handles R_AARCH64_GLOB_DAT and R_AARCH64_JUMP_SLOT.
    pub fn apply_rela_aarch64<L: ElfLoader>(
        &mut self,
        loader: &mut L,
        export_map: &HashMap<String, u64>,
    ) {
        let rela_sections: Vec<(u64, u64)> = self
            .elf_shdr
            .iter()
            .filter(|shdr| {
                let name = self.get_section_name(shdr.sh_name as usize);
                name == ".rela.dyn" || name == ".rela.plt"
            })
            .map(|shdr| (shdr.sh_offset, shdr.sh_size))
            .collect();

        let entsize = 24usize; // sizeof(Elf64_Rela)

        for (sh_offset, sh_size) in rela_sections {
            let mut off = sh_offset as usize;
            let end = off + sh_size as usize;

            while off + entsize <= end && off + entsize <= self.bin.len() {
                let r_offset = read_u64_le!(self.bin, off);
                let r_info = read_u64_le!(self.bin, off + 8);

                let r_type = (r_info & 0xFFFFFFFF) as u32;
                let r_sym = (r_info >> 32) as u32;

                if r_type != R_AARCH64_GLOB_DAT && r_type != R_AARCH64_JUMP_SLOT {
                    off += entsize;
                    continue;
                }

                let sym_name = self.get_dynsym_name(r_sym);
                if sym_name.is_empty() {
                    off += entsize;
                    continue;
                }

                if let Some(&target_addr) = export_map.get(&sym_name) {
                    let got_addr = if r_offset < self.base {
                        r_offset + self.base
                    } else {
                        r_offset
                    };

                    loader.write_qword(got_addr, target_addr);
                    self.sym_to_addr.insert(sym_name.clone(), target_addr);
                    self.addr_to_symbol.insert(target_addr, sym_name);
                }

                off += entsize;
            }
        }
    }

    /// Look up a symbol name from .dynsym by index.
    fn get_dynsym_name(&self, sym_index: u32) -> String {
        let idx = sym_index as usize;
        if idx < self.elf_dynsym.len() {
            return self.elf_dynsym[idx].st_dynstr_name.clone();
        }

        // Fallback: read from raw binary
        let dynsym_shdr = self.elf_shdr.iter().find(|shdr| {
            self.get_section_name(shdr.sh_name as usize) == ".dynsym"
        });
        let Some(shdr) = dynsym_shdr else {
            return String::new();
        };

        let entry_off = shdr.sh_offset as usize + (idx * 24);
        if entry_off + 4 > self.bin.len() {
            return String::new();
        }

        let st_name = read_u32_le!(self.bin, entry_off);
        if st_name == 0 || self.elf_dynstr_off == 0 {
            return String::new();
        }

        let name_off = (self.elf_dynstr_off + st_name as u64) as usize;
        if name_off >= self.bin.len() {
            return String::new();
        }

        let end = self.bin[name_off..]
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(self.bin.len() - name_off);
        std::str::from_utf8(&self.bin[name_off..name_off + end])
            .unwrap_or("")
            .to_string()
    }

    /*
    pub fn dynsym_offset_to_addr(&self, off: usize) -> u64 {
        for sym in self.elf_dynsym.iter() {
            if sym.st_name as usize == off {
                return
            }
        }
        return 0;
    }

    pub fn dynstr_name_to_offset(&self, name: &str) -> Option<usize> {
        for i in 0..self.elf_dynstr.len() {
            if name == self.elf_dynstr[i] {
                return Some(i);
            }
        }
        None
    }*/

    pub fn load<L: ElfLoader>(
        &mut self,
        loader: &mut L,
        name: &str,
        is_lib: bool,
        dynamic_linking: bool,
        force_base: u64,
    ) {
        let elf64_base: u64;

        if dynamic_linking {
            elf64_base = if is_lib {
                loader
                    .lib_alloc(self.image_size())
                    .expect("cannot allocate elf64 library space")
            } else {
                ELF64_DYN_BASE
            };
        } else {
            if force_base == CFG_DEFAULT_BASE {
                elf64_base = ELF64_STA_BASE;
            } else {
                elf64_base = force_base;
            }

            // elf executable need to map the header.
            // Tiny ELFs (hand-crafted) may be smaller than 512 bytes; clamp the
            // copy length to the actual binary size to avoid a slice OOB panic.
            let hdr_copy_len = self.bin.len().min(512);
            loader
                .map("elf64.hdr", elf64_base, 512, Perm::READ_WRITE)
                .expect("cannot create elf64.hdr map");
            loader.write_bytes(elf64_base, &self.bin[..hdr_copy_len]);
        }

        self.base = elf64_base;
        self.sym_to_addr.clear();
        self.addr_to_symbol.clear();
        self.needed_libs = self.get_dynamic();

        // Segment (PT_LOAD) mapping for running the real ld.so / libc code.
        if self.segment_mode {
            self.map_segments(loader, name, elf64_base);
        }

        // pre-load .dynstr
        for shdr in &self.elf_shdr {
            let sname = self.get_section_name(shdr.sh_name as usize);
            if sname == ".dynstr" {
                self.elf_dynstr_off = shdr.sh_offset;
            }
        }

        // map sections, remember to skip section start from 0 because it is empty section
        for i in 1..self.elf_shdr.len() {
            let sh_name = self.elf_shdr[i].sh_name;
            let sh_offset = self.elf_shdr[i].sh_offset;
            let sh_size = self.elf_shdr[i].sh_size;
            let mut sh_addr = self.elf_shdr[i].sh_addr;

            let sh_flags = self.elf_shdr[i].sh_flags;
            // SHF_ALLOC (0x2) means the section occupies memory at runtime and should be readable
            let is_alloc = sh_flags & 0x2 != 0;
            let can_write = sh_flags & 0x1 != 0; // SHF_WRITE
            let can_execute = sh_flags & 0x4 != 0; // SHF_EXECINSTR
            let can_read = is_alloc;
            let permission = Perm::from_flags(can_read, can_write, can_execute);

            //TODO: align sh_size to page size by extending the size, something like:
            //sh_size = ((sh_size + constants::ELF_PAGE_SIZE - 1) / constants::ELF_PAGE_SIZE) * constants::ELF_PAGE_SIZE;

            let sname = self.get_section_name(sh_name as usize);

            //log::trace!("loading elf64 section {}", sname);
            if sname == ".comment"
                || sname.starts_with(".note")
                || sname == ".interp"
                || sname.starts_with(".gnu")
            {
                continue;
            }

            // get .got offset
            if sname == ".got" {
                self.elf_got_off = sh_offset;
            }

            // load dynsym
            if sname == ".dynsym" {
                self.elf_dynsym.clear();
                let mut off = sh_offset as usize;
                let entsize = if self.elf_shdr[i].sh_entsize == 0 {
                    Elf64Sym::size() as u64
                } else {
                    self.elf_shdr[i].sh_entsize
                };

                for _ in 0..(sh_size / entsize) {
                    let mut sym = Elf64Sym::parse(&self.bin, off);

                    let off2 = (self.elf_dynstr_off + sym.st_name as u64) as usize;
                    if let Some(name) = self.read_c_string(off2) {
                        sym.st_dynstr_name = name;
                    }

                    if sym.st_value > 0 {
                        sym.st_value = self.rebase_vaddr(sym.st_value);
                    }

                    if !sym.st_dynstr_name.is_empty() && sym.st_value > 0 {
                        self.sym_to_addr
                            .insert(sym.st_dynstr_name.clone(), sym.st_value);
                        self.addr_to_symbol
                            .insert(sym.st_value, sym.st_dynstr_name.clone());
                    }

                    self.elf_dynsym.push(sym);
                    off += entsize as usize;
                }
            }

            // Segment mode already mapped the whole image by PT_LOAD; here we
            // only need the section walk for symbol/.dynamic parsing, so skip
            // the per-section map creation entirely.
            if self.segment_mode {
                self.elf_shdr[i].sh_addr = self.rebase_vaddr(sh_addr);
                continue;
            }

            // map if its vaddr is on a PT_LOAD program
            if self.is_loadable(sh_addr) || !dynamic_linking {
                if sname == ".shstrtab" || sname == ".tbss" {
                    continue;
                }

                // Skip non-allocated sections (e.g. .symtab, .strtab) — they are metadata
                // and should not be mapped into the runtime address space.
                if !is_alloc {
                    continue;
                }

                let map_name: String = if sname == ".text" && !is_lib {
                    "code".to_string()
                } else {
                    format!("{}{}", name, sname) //self.get_section_name(shdr.sh_name as usize));
                };
                if sname == ".init" {
                    self.init = Some(sh_addr);
                }

                if sh_size == 0 {
                    log::trace!("section {} size is zero, skipping.", sname);
                    continue;
                }

                if sh_addr < elf64_base {
                    sh_addr += elf64_base;
                }
                // The loader maps the region and reports where it actually
                // landed; on an overlap it may relocate the map elsewhere, in
                // which case we keep that address for the section.
                sh_addr = match loader.map(&map_name, sh_addr, sh_size, permission) {
                    Some(addr) => addr,
                    None => {
                        log::warn!(
                            "elf64 {} could not map 0x{:x} {}",
                            map_name, sh_addr, sh_size
                        );
                        continue;
                    }
                };

                // SHT_NOBITS (.bss / .tbss) occupies memory but has NO file
                // contents — its `sh_offset` points at unrelated file bytes.
                // The map is already zero-filled, so just leave it untouched;
                // copying file bytes here would corrupt e.g. ld.so's `_rtld_local`.
                const SHT_NOBITS: u32 = 8;
                if self.elf_shdr[i].sh_type == SHT_NOBITS {
                    self.elf_shdr[i].sh_addr = sh_addr;
                    continue;
                }

                let mut end_off = (sh_offset + sh_size) as usize;
                if end_off > self.bin.len() {
                    end_off = self.bin.len();
                }

                if sh_offset > end_off as u64 {
                    log::trace!("invalid section {}: sh_offset > end_off", sname);
                    continue;
                }
                if end_off as u64 - sh_offset > sh_size {
                    log::trace!(
                        "no room at sh_size for all the data in the section, skipping {}",
                        sname
                    );
                    continue;
                }

                let segment = &self.bin[sh_offset as usize..end_off];
                loader.write_bytes(sh_addr, segment);

                self.elf_shdr[i].sh_addr = sh_addr;
            }
        }
    }

    /// Map the image by PT_LOAD segments (like the kernel/`mmap` does), so the
    /// ELF header, program headers and inter-section padding are all present and
    /// contiguous — required for the real ld.so / libc to introspect themselves.
    fn map_segments<L: ElfLoader>(&mut self, loader: &mut L, name: &str, base: u64) {
        const PAGE: u64 = 0x1000;
        for idx in 0..self.elf_phdr.len() {
            let phdr = &self.elf_phdr[idx];
            if phdr.p_type != PT_LOAD {
                continue;
            }

            let seg_vaddr = base + phdr.p_vaddr;
            let map_start = seg_vaddr & !(PAGE - 1);
            let map_end = (seg_vaddr + phdr.p_memsz + PAGE - 1) & !(PAGE - 1);
            let map_size = (map_end - map_start).max(PAGE);

            let can_x = phdr.p_flags & 1 != 0;
            let can_w = phdr.p_flags & 2 != 0;
            let can_r = phdr.p_flags & 4 != 0;
            // ld.so writes into its own RELRO/GOT during bootstrap before the
            // kernel would mprotect it read-only, so keep writable here.
            let perm = Perm::from_flags(can_r, true, can_x);
            let _ = can_w;

            let map_name = format!("{}.seg{}", name, idx);
            let file_start = phdr.p_offset as usize;
            let file_end = (phdr.p_offset + phdr.p_filesz).min(self.bin.len() as u64) as usize;

            match loader.map(&map_name, map_start, map_size, perm) {
                Some(_) => {
                    if file_end > file_start {
                        loader.write_bytes(seg_vaddr, &self.bin[file_start..file_end]);
                    }
                }
                None => {
                    log::warn!(
                        "segment map {} failed at 0x{:x} size 0x{:x}",
                        map_name, map_start, map_size
                    );
                }
            }
        }
    }

    /// Apply RELR (DT_RELR / `.relr.dyn`) relative relocations: `*where += base`
    /// for a compact bitmap-encoded list of addresses. Modern glibc ld.so uses
    /// this for most of its internal pointers; we apply it as the loader so the
    /// real interpreter's data pointers are valid before it runs.
    pub fn apply_relr<L: ElfLoader>(&self, loader: &mut L, base: u64) {
        const SHT_RELR: u32 = 19;
        let Some((off, size)) = self.elf_shdr.iter().find_map(|sh| {
            if sh.sh_type == SHT_RELR || self.get_section_name(sh.sh_name as usize) == ".relr.dyn" {
                Some((sh.sh_offset as usize, sh.sh_size as usize))
            } else {
                None
            }
        }) else {
            return;
        };

        let end = (off + size).min(self.bin.len());
        let mut cur = off;
        let mut where_addr: u64 = 0;

        let reloc = |loader: &mut L, at: u64, base: u64| {
            let v = loader.read_qword(at).unwrap_or(0).wrapping_add(base);
            loader.write_qword(at, v);
        };

        while cur + 8 <= end {
            let entry = read_u64_le!(self.bin, cur);
            cur += 8;
            if entry & 1 == 0 {
                // Address entry: relocate one slot, set the running pointer.
                where_addr = base.wrapping_add(entry);
                reloc(loader, where_addr, base);
                where_addr = where_addr.wrapping_add(8);
            } else {
                // Bitmap entry: bits 1..63 relocate consecutive slots.
                let mut bits = entry >> 1;
                let mut i = 0u64;
                while bits != 0 {
                    if bits & 1 != 0 {
                        reloc(loader, where_addr.wrapping_add(i * 8), base);
                    }
                    i += 1;
                    bits >>= 1;
                }
                where_addr = where_addr.wrapping_add(63 * 8);
            }
        }
    }

    pub fn get_dynamic(&self) -> Vec<String> {
        self.dynamic_info()
            .map(|info| info.needed)
            .unwrap_or_default()
    }

    /// Path of the program interpreter (PT_INTERP), e.g.
    /// `/lib64/ld-linux-x86-64.so.2`, if present.
    pub fn get_interp(&self) -> Option<String> {
        const PT_INTERP: u32 = 3;
        for phdr in &self.elf_phdr {
            if phdr.p_type == PT_INTERP {
                let start = phdr.p_offset as usize;
                let end = (phdr.p_offset + phdr.p_filesz) as usize;
                if end <= self.bin.len() {
                    let bytes = &self.bin[start..end];
                    let s: Vec<u8> = bytes.iter().take_while(|&&c| c != 0).copied().collect();
                    return String::from_utf8(s).ok();
                }
            }
        }
        None
    }

    /// `true` if `raw` is any ELF image (`0x7f 'E' 'L' 'F'` magic), regardless
    /// of class or architecture.
    pub fn is_elf(raw: &[u8]) -> bool {
        raw.len() >= 4 && raw[0] == 0x7f && raw[1] == b'E' && raw[2] == b'L' && raw[3] == b'F'
    }

    /// `true` if `raw` is a 64-bit ELF image (`ELFCLASS64`), regardless of
    /// architecture.
    pub fn is_elf64(raw: &[u8]) -> bool {
        Self::is_elf(raw) && raw.len() > 4 && raw[4] == ELFCLASS64
    }

    /// The `e_machine` (architecture) field of an ELF image, or `None` if `raw`
    /// is not an ELF. The value is one of the `EM_*` constants. Byte-based,
    /// mirroring PE's `pe_machine_type`. Works for both ELF32 and ELF64 (the
    /// field lives at the same offset).
    pub fn elf_machine(raw: &[u8]) -> Option<u16> {
        if !Self::is_elf(raw) || raw.len() < 20 {
            return None;
        }
        Some(u16::from_le_bytes([raw[18], raw[19]]))
    }

    pub fn is_elf64_x64(raw: &[u8]) -> bool {
        Self::is_elf64(raw) && Self::elf_machine(raw) == Some(EM_X86_64)
    }

    pub fn is_elf64_aarch64(raw: &[u8]) -> bool {
        Self::is_elf64(raw) && Self::elf_machine(raw) == Some(EM_AARCH64)
    }
}

#[derive(Debug)]
pub struct Elf64Ehdr {
    pub e_ident: [u8; EI_NIDENT],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl Elf64Ehdr {
    pub fn new() -> Elf64Ehdr {
        Elf64Ehdr {
            e_ident: [0; EI_NIDENT],
            e_type: 0,
            e_machine: 0,
            e_version: 0,
            e_entry: 0,
            e_phoff: 0,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: 0,
            e_phentsize: 0,
            e_phnum: 0,
            e_shentsize: 0,
            e_shnum: 0,
            e_shstrndx: 0,
        }
    }

    pub fn parse(bin: &[u8]) -> Elf64Ehdr {
        Elf64Ehdr {
            e_ident: [
                read_u8!(bin, 0),
                read_u8!(bin, 1),
                read_u8!(bin, 2),
                read_u8!(bin, 3),
                read_u8!(bin, 4),
                read_u8!(bin, 5),
                read_u8!(bin, 6),
                read_u8!(bin, 7),
                read_u8!(bin, 8),
                read_u8!(bin, 9),
                read_u8!(bin, 10),
                read_u8!(bin, 11),
                read_u8!(bin, 12),
                read_u8!(bin, 13),
                read_u8!(bin, 14),
                read_u8!(bin, 15),
            ],
            e_type: read_u16_le!(bin, 16),
            e_machine: read_u16_le!(bin, 18),
            e_version: read_u32_le!(bin, 20),
            e_entry: read_u64_le!(bin, 24),
            e_phoff: read_u64_le!(bin, 32),
            e_shoff: read_u64_le!(bin, 40),
            e_flags: read_u32_le!(bin, 48),
            e_ehsize: read_u16_le!(bin, 52),
            e_phentsize: read_u16_le!(bin, 54),
            e_phnum: read_u16_le!(bin, 56),
            e_shentsize: read_u16_le!(bin, 58),
            e_shnum: read_u16_le!(bin, 60),
            e_shstrndx: read_u16_le!(bin, 62),
        }
    }
}

#[derive(Debug)]
pub struct Elf64Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

impl Elf64Phdr {
    pub fn parse(bin: &[u8], phoff: usize) -> Elf64Phdr {
        Elf64Phdr {
            p_type: read_u32_le!(bin, phoff),
            p_flags: read_u32_le!(bin, phoff + 4),
            p_offset: read_u64_le!(bin, phoff + 8),
            p_vaddr: read_u64_le!(bin, phoff + 16),
            p_paddr: read_u64_le!(bin, phoff + 24),
            p_filesz: read_u64_le!(bin, phoff + 32),
            p_memsz: read_u64_le!(bin, phoff + 40),
            p_align: read_u64_le!(bin, phoff + 48),
        }
    }
}

#[derive(Debug)]
pub struct Elf64Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

impl Elf64Shdr {
    pub fn parse(bin: &[u8], shoff: usize) -> Elf64Shdr {
        Elf64Shdr {
            sh_name: read_u32_le!(bin, shoff),
            sh_type: read_u32_le!(bin, shoff + 4),
            sh_flags: read_u64_le!(bin, shoff + 8),
            sh_addr: read_u64_le!(bin, shoff + 16),
            sh_offset: read_u64_le!(bin, shoff + 24),
            sh_size: read_u64_le!(bin, shoff + 32),
            sh_link: read_u32_le!(bin, shoff + 40),
            sh_info: read_u32_le!(bin, shoff + 44),
            sh_addralign: read_u64_le!(bin, shoff + 48),
            sh_entsize: read_u64_le!(bin, shoff + 56),
        }
    }
}

#[derive(Debug)]
pub struct Elf64Sym {
    pub st_dynstr_name: String,
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}

/// Result of applying a binary's dynamic relocations.
#[derive(Debug, Default)]
pub struct RelocationOutcome {
    /// Symbolic imports that could not be resolved against any loaded library.
    pub unresolved: Vec<String>,
    /// IRELATIVE (ifunc) relocations as `(patch_addr, resolver_addr)`; the
    /// resolver must be executed by the emulator to obtain the final value.
    pub irelative: Vec<(u64, u64)>,
}

#[derive(Debug, Default)]
pub struct Elf64DynamicInfo {
    pub needed: Vec<String>,
    pub strtab_addr: u64,
    pub symtab_addr: u64,
    pub syment: u64,
    pub rela_addr: u64,
    pub rela_size: u64,
    pub rela_ent: u64,
    pub jmprel_addr: u64,
    pub pltrelsz: u64,
}

#[derive(Debug)]
pub struct Elf64Rela {
    pub r_offset: u64,
    pub r_info: u64,
    pub r_addend: i64,
}

impl Elf64Rela {
    pub fn parse(bin: &[u8], off: usize) -> Elf64Rela {
        Elf64Rela {
            r_offset: read_u64_le!(bin, off),
            r_info: read_u64_le!(bin, off + 8),
            r_addend: read_u64_le!(bin, off + 16) as i64,
        }
    }

    pub fn size() -> usize {
        24
    }

    pub fn r_sym(&self) -> u32 {
        (self.r_info >> 32) as u32
    }

    pub fn r_type(&self) -> u32 {
        self.r_info as u32
    }
}

impl Elf64Sym {
    pub fn parse(bin: &[u8], off: usize) -> Elf64Sym {
        Elf64Sym {
            st_dynstr_name: String::new(),
            st_name: read_u32_le!(bin, off),
            st_info: read_u8!(bin, off + 4),
            st_other: read_u8!(bin, off + 5),
            st_shndx: read_u16_le!(bin, off + 6),
            st_value: read_u64_le!(bin, off + 8),
            st_size: read_u64_le!(bin, off + 16),
        }
    }

    pub fn size() -> usize {
        24
    }

    pub fn get_st_type(&self) -> u8 {
        self.st_info & 0x0f
    }
}
