//! ET_REL (relocatable object) loading: Linux kernel modules (`.ko`), plain
//! `.o` files, and anything else the static linker has not laid out yet.
//!
//! Unlike ET_EXEC / ET_DYN there are no program headers and no `.dynamic`:
//! nothing in the file says *where* the code goes. The consumer has to place
//! every `SHF_ALLOC` section itself and then patch every relocation, which is
//! exactly what `kernel/module.c` does with `layout_sections()` followed by
//! `apply_relocate_add()`. This module is the parsing half of that job; the
//! memory half goes through [`ElfLoader`] like the rest of `rs-header`.
//!
//! External symbols (`kmalloc`, `printk`, …) are left to the caller: it passes
//! a resolver closure, mirroring the kernel's `resolve_symbol()` against
//! `__ksymtab`. Anything the resolver declines is reported in
//! [`RelObject::unresolved`] instead of failing the load, so a module can be
//! emulated even when only part of the kernel surface is implemented.

use std::collections::HashMap;

use crate::elf::ElfError;
use crate::elf::elf64::{Elf64, Elf64Rela, Elf64Sym};
use crate::elf::loader::{ElfLoader, Perm};

/// `e_type` of a relocatable object.
pub const ET_REL: u16 = 1;

const SHT_SYMTAB: u32 = 2;
const SHT_RELA: u32 = 4;
const SHT_NOBITS: u32 = 8;

const SHF_WRITE: u64 = 0x1;
const SHF_ALLOC: u64 = 0x2;
const SHF_EXECINSTR: u64 = 0x4;

const SHN_UNDEF: u16 = 0;
const SHN_ABS: u16 = 0xfff1;
const SHN_COMMON: u16 = 0xfff2;

const STB_LOCAL: u8 = 0;
const STT_FUNC: u8 = 2;
const STT_SECTION: u8 = 3;

// x86_64 relocation types emitted into kernel modules (the set
// `arch/x86/kernel/module.c:apply_relocate_add()` knows about).
const R_X86_64_NONE: u32 = 0;
const R_X86_64_64: u32 = 1;
const R_X86_64_PC32: u32 = 2;
const R_X86_64_PLT32: u32 = 4;
const R_X86_64_32: u32 = 10;
const R_X86_64_32S: u32 = 11;
const R_X86_64_PC64: u32 = 24;

// aarch64 relocation types emitted into kernel modules (the common subset of
// `arch/arm64/kernel/module.c:apply_relocate_add()`).
const R_AARCH64_ABS64: u32 = 257;
const R_AARCH64_ABS32: u32 = 258;
const R_AARCH64_PREL64: u32 = 260;
const R_AARCH64_PREL32: u32 = 261;
const R_AARCH64_ADR_PREL_PG_HI21: u32 = 275;
const R_AARCH64_ADD_ABS_LO12_NC: u32 = 277;
const R_AARCH64_JUMP26: u32 = 282;
const R_AARCH64_CALL26: u32 = 283;
const R_AARCH64_LDST8_ABS_LO12_NC: u32 = 278;
const R_AARCH64_LDST16_ABS_LO12_NC: u32 = 284;
const R_AARCH64_LDST32_ABS_LO12_NC: u32 = 285;
const R_AARCH64_LDST64_ABS_LO12_NC: u32 = 286;
const R_AARCH64_LDST128_ABS_LO12_NC: u32 = 299;

/// One `SHF_ALLOC` section after placement.
#[derive(Debug, Clone)]
pub struct RelSection {
    /// Index into the object's section-header table.
    pub index: usize,
    pub name: String,
    /// Address the section was mapped at.
    pub addr: u64,
    pub size: u64,
    pub perm: Perm,
}

/// One symbol defined by the object, with its final (post-placement) address.
#[derive(Debug, Clone)]
pub struct RelSymbol {
    pub name: String,
    pub addr: u64,
    pub size: u64,
    pub is_func: bool,
    pub is_global: bool,
}

/// Everything the consumer needs after an ET_REL object has been placed.
#[derive(Debug, Default)]
pub struct RelObject {
    /// Lowest address of the placed image.
    pub base: u64,
    /// Total bytes spanned by the placed image.
    pub size: u64,
    pub sections: Vec<RelSection>,
    /// Symbols the object defines (functions and objects, local and global).
    pub symbols: Vec<RelSymbol>,
    /// External symbols the resolver declined, i.e. unimplemented kernel API.
    pub unresolved: Vec<String>,
    /// Relocations skipped because their symbol was unresolved.
    pub skipped_relocations: usize,
}

impl RelObject {
    /// Address of a symbol this object defines. Globals win over locals when a
    /// name appears twice (a static and an exported function can share a name
    /// across translation units merged into one module).
    pub fn symbol(&self, name: &str) -> Option<u64> {
        self.symbols
            .iter()
            .filter(|s| s.name == name)
            .max_by_key(|s| s.is_global)
            .map(|s| s.addr)
    }

    /// Address of a placed section by name, e.g. `.text` or `.gnu.linkonce.this_module`.
    pub fn section(&self, name: &str) -> Option<&RelSection> {
        self.sections.iter().find(|s| s.name == name)
    }
}

/// Alignment helper: rounds `value` up to `align` (a zero/one alignment is a
/// no-op, matching the ELF convention).
fn align_up(value: u64, align: u64) -> u64 {
    if align <= 1 {
        return value;
    }
    value.div_ceil(align) * align
}

/// Read a NUL-terminated name out of a string table blob.
fn strtab_name(blob: &[u8], off: usize) -> String {
    if off >= blob.len() {
        return String::new();
    }
    let end = blob[off..]
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(blob.len() - off);
    String::from_utf8_lossy(&blob[off..off + end]).into_owned()
}

impl Elf64 {
    /// True when this image is a relocatable object (a `.ko`, a `.o`).
    pub fn is_relocatable(&self) -> bool {
        self.elf_hdr.e_type == ET_REL
    }

    /// Byte-level detector, usable before parsing (mirrors `is_elf64_x64`).
    pub fn is_elf64_relocatable(raw: &[u8]) -> bool {
        Elf64::is_elf64(raw)
            && raw.len() > 17
            && (raw[16] as u16 | ((raw[17] as u16) << 8)) == ET_REL
    }

    /// Place every `SHF_ALLOC` section of a relocatable object contiguously
    /// from `base`, then apply its `SHT_RELA` relocations.
    ///
    /// `resolve_extern` is called once per undefined symbol name; returning
    /// `None` leaves the symbol unresolved (recorded, relocation skipped)
    /// rather than aborting, so a partially implemented kernel surface still
    /// yields a runnable image.
    ///
    /// Sections are placed contiguously on purpose: `R_X86_64_PC32` /
    /// `PLT32` (and aarch64 `CALL26`) encode signed 32-bit / 26-bit
    /// displacements, so `.text` and `.data` must stay within range of each
    /// other — and of whatever the resolver hands back for external symbols.
    pub fn load_relocatable<L: ElfLoader>(
        &mut self,
        loader: &mut L,
        name: &str,
        base: u64,
        resolve_extern: &mut dyn FnMut(&str) -> Option<u64>,
    ) -> Result<RelObject, ElfError> {
        if !self.is_relocatable() {
            return Err(ElfError::new("not an ET_REL object"));
        }

        let mut obj = RelObject {
            base,
            ..Default::default()
        };

        // --- 1. place the allocatable sections -----------------------------
        // Section addresses are recorded back into `elf_shdr[i].sh_addr` so the
        // relocation pass (and the caller's symbol lookups) can use them
        // directly, the same way the kernel rewrites `sechdrs[i].sh_addr`.
        let mut cursor = base;
        for i in 0..self.elf_shdr.len() {
            let shdr_flags = self.elf_shdr[i].sh_flags;
            let sh_size = self.elf_shdr[i].sh_size;
            if shdr_flags & SHF_ALLOC == 0 || sh_size == 0 {
                self.elf_shdr[i].sh_addr = 0;
                continue;
            }

            let sname = self.get_section_name(self.elf_shdr[i].sh_name as usize);
            let align = self.elf_shdr[i].sh_addralign.max(16);
            cursor = align_up(cursor, align);

            let perm = Perm::from_flags(
                true,
                shdr_flags & SHF_WRITE != 0,
                shdr_flags & SHF_EXECINSTR != 0,
            );
            let map_name = format!("{}{}", name, sname);
            let addr = loader
                .map(&map_name, cursor, sh_size, perm)
                .ok_or_else(|| ElfError::new(&format!("cannot map section {}", sname)))?;

            // SHT_NOBITS (.bss) has no file image: the map is already zeroed.
            if self.elf_shdr[i].sh_type != SHT_NOBITS {
                let off = self.elf_shdr[i].sh_offset as usize;
                let end = off.saturating_add(sh_size as usize);
                if end > self.bin.len() {
                    return Err(ElfError::new(&format!(
                        "section {} extends past end of object",
                        sname
                    )));
                }
                loader.write_bytes(addr, &self.bin[off..end]);
            }

            self.elf_shdr[i].sh_addr = addr;
            cursor = addr + sh_size;
            obj.sections.push(RelSection {
                index: i,
                name: sname,
                addr,
                size: sh_size,
                perm,
            });
        }
        obj.size = cursor.saturating_sub(base);

        // --- 2. resolve the symbol table -----------------------------------
        let symbols = self.parse_symtab()?;
        let mut values: Vec<u64> = Vec::with_capacity(symbols.len());
        let mut externs: HashMap<String, Option<u64>> = HashMap::new();

        self.sym_to_addr.clear();
        self.addr_to_symbol.clear();

        for sym in &symbols {
            let value = match sym.st_shndx {
                SHN_UNDEF => {
                    if sym.st_dynstr_name.is_empty() {
                        Some(0)
                    } else {
                        // Cache per name: a module references `kmalloc` dozens
                        // of times and the resolver may create a stub per call.
                        *externs
                            .entry(sym.st_dynstr_name.clone())
                            .or_insert_with(|| resolve_extern(&sym.st_dynstr_name))
                    }
                }
                SHN_ABS => Some(sym.st_value),
                // SHN_COMMON is only produced with -fcommon; kernel modules are
                // built with -fno-common, so treat it as unresolved rather than
                // silently allocating storage the object does not describe.
                SHN_COMMON => None,
                idx => {
                    let idx = idx as usize;
                    if idx < self.elf_shdr.len() && self.elf_shdr[idx].sh_addr != 0 {
                        Some(self.elf_shdr[idx].sh_addr + sym.st_value)
                    } else {
                        // Symbol in a non-allocated section (e.g. debug info):
                        // harmless, nothing will branch there.
                        Some(0)
                    }
                }
            };

            values.push(value.unwrap_or(0));

            match value {
                Some(addr) if addr != 0 && !sym.st_dynstr_name.is_empty() => {
                    let is_global = (sym.st_info >> 4) != STB_LOCAL;
                    if sym.st_shndx != SHN_UNDEF {
                        obj.symbols.push(RelSymbol {
                            name: sym.st_dynstr_name.clone(),
                            addr,
                            size: sym.st_size,
                            is_func: sym.get_st_type() == STT_FUNC,
                            is_global,
                        });
                    }
                    self.sym_to_addr.insert(sym.st_dynstr_name.clone(), addr);
                    self.addr_to_symbol.insert(addr, sym.st_dynstr_name.clone());
                }
                None => obj.unresolved.push(sym.st_dynstr_name.clone()),
                _ => {}
            }
        }
        obj.unresolved.sort();
        obj.unresolved.dedup();

        // --- 3. apply the relocations --------------------------------------
        let machine = self.elf_hdr.e_machine;
        for i in 0..self.elf_shdr.len() {
            if self.elf_shdr[i].sh_type != SHT_RELA {
                continue;
            }
            let target_idx = self.elf_shdr[i].sh_info as usize;
            if target_idx >= self.elf_shdr.len() || self.elf_shdr[target_idx].sh_addr == 0 {
                continue; // relocations for a non-allocated section (debug info)
            }
            let target_base = self.elf_shdr[target_idx].sh_addr;

            let mut off = self.elf_shdr[i].sh_offset as usize;
            let entsize = if self.elf_shdr[i].sh_entsize == 0 {
                Elf64Rela::size()
            } else {
                self.elf_shdr[i].sh_entsize as usize
            };
            let count = (self.elf_shdr[i].sh_size as usize) / entsize.max(1);

            for _ in 0..count {
                if off + Elf64Rela::size() > self.bin.len() {
                    break;
                }
                let rela = Elf64Rela::parse(&self.bin, off);
                off += entsize;

                let sym_idx = rela.r_sym() as usize;
                if sym_idx >= values.len() {
                    continue;
                }
                let sym = &symbols[sym_idx];
                if sym.st_shndx == SHN_UNDEF
                    && !sym.st_dynstr_name.is_empty()
                    && values[sym_idx] == 0
                {
                    obj.skipped_relocations += 1;
                    continue;
                }

                let place = target_base + rela.r_offset;
                let value = values[sym_idx];
                let addend = rela.r_addend;

                let applied = if machine == crate::elf::EM_AARCH64 {
                    apply_aarch64(loader, rela.r_type(), place, value, addend)
                } else {
                    apply_x86_64(loader, rela.r_type(), place, value, addend)
                };

                if !applied {
                    obj.skipped_relocations += 1;
                }
            }
        }

        self.base = base;
        Ok(obj)
    }

    /// Parse `.symtab` (with the `.strtab` its `sh_link` points at) into owned
    /// symbols with their names filled in. ET_REL objects carry no `.dynsym`,
    /// so this is the only symbol source for a kernel module.
    pub fn parse_symtab(&self) -> Result<Vec<Elf64Sym>, ElfError> {
        let Some(symtab_idx) = self.elf_shdr.iter().position(|sh| sh.sh_type == SHT_SYMTAB) else {
            return Err(ElfError::new("object has no .symtab"));
        };

        let strtab_idx = self.elf_shdr[symtab_idx].sh_link as usize;
        if strtab_idx >= self.elf_shdr.len() {
            return Err(ElfError::new(".symtab sh_link is out of range"));
        }
        let str_off = self.elf_shdr[strtab_idx].sh_offset as usize;
        let str_end = str_off
            .saturating_add(self.elf_shdr[strtab_idx].sh_size as usize)
            .min(self.bin.len());
        let strtab = if str_end > str_off {
            &self.bin[str_off..str_end]
        } else {
            &[][..]
        };

        let entsize = if self.elf_shdr[symtab_idx].sh_entsize == 0 {
            Elf64Sym::size()
        } else {
            self.elf_shdr[symtab_idx].sh_entsize as usize
        };
        let mut off = self.elf_shdr[symtab_idx].sh_offset as usize;
        let count = (self.elf_shdr[symtab_idx].sh_size as usize) / entsize.max(1);

        let mut symbols = Vec::with_capacity(count);
        for _ in 0..count {
            if off + Elf64Sym::size() > self.bin.len() {
                break;
            }
            let mut sym = Elf64Sym::parse(&self.bin, off);
            off += entsize;
            sym.st_dynstr_name = if sym.get_st_type() == STT_SECTION {
                // Section symbols name themselves through their section, and a
                // module has many of them; leaving the name empty keeps them
                // out of the exported symbol map.
                String::new()
            } else {
                strtab_name(strtab, sym.st_name as usize)
            };
            symbols.push(sym);
        }

        Ok(symbols)
    }
}

/// Patch one x86_64 relocation. Returns false for a type we do not implement.
fn apply_x86_64<L: ElfLoader>(
    loader: &mut L,
    r_type: u32,
    place: u64,
    value: u64,
    addend: i64,
) -> bool {
    match r_type {
        R_X86_64_NONE => true,
        R_X86_64_64 => loader.write_qword(place, value.wrapping_add(addend as u64)),
        R_X86_64_PC64 => {
            let v = value.wrapping_add(addend as u64).wrapping_sub(place);
            loader.write_qword(place, v)
        }
        // PLT32 has no PLT inside a module: the kernel treats it exactly like
        // PC32 (arch/x86/kernel/module.c).
        R_X86_64_PC32 | R_X86_64_PLT32 => {
            let v = value.wrapping_add(addend as u64).wrapping_sub(place) as u32;
            loader.write_bytes(place, &v.to_le_bytes())
        }
        R_X86_64_32 | R_X86_64_32S => {
            let v = value.wrapping_add(addend as u64) as u32;
            loader.write_bytes(place, &v.to_le_bytes())
        }
        _ => false,
    }
}

/// Read the 32-bit instruction word at `place` so a bitfield relocation can be
/// merged into it.
fn read_insn<L: ElfLoader>(loader: &L, place: u64) -> Option<u32> {
    // ElfLoader exposes qword reads only; take the low half of the containing
    // qword when `place` is 8-byte aligned, the high half otherwise.
    let aligned = place & !7;
    let q = loader.read_qword(aligned)?;
    Some(if place == aligned {
        q as u32
    } else {
        (q >> 32) as u32
    })
}

/// Patch one aarch64 relocation. Returns false for a type we do not implement.
fn apply_aarch64<L: ElfLoader>(
    loader: &mut L,
    r_type: u32,
    place: u64,
    value: u64,
    addend: i64,
) -> bool {
    let s_a = value.wrapping_add(addend as u64);

    // Merge `bits` into the existing instruction word under `mask`.
    let patch_insn = |loader: &mut L, bits: u32, mask: u32| -> bool {
        let Some(insn) = read_insn(loader, place) else {
            return false;
        };
        let merged = (insn & !mask) | (bits & mask);
        loader.write_bytes(place, &merged.to_le_bytes())
    };

    match r_type {
        R_AARCH64_ABS64 => loader.write_qword(place, s_a),
        R_AARCH64_ABS32 => loader.write_bytes(place, &(s_a as u32).to_le_bytes()),
        R_AARCH64_PREL64 => loader.write_qword(place, s_a.wrapping_sub(place)),
        R_AARCH64_PREL32 => {
            loader.write_bytes(place, &(s_a.wrapping_sub(place) as u32).to_le_bytes())
        }
        R_AARCH64_CALL26 | R_AARCH64_JUMP26 => {
            let off = s_a.wrapping_sub(place) as i64 >> 2;
            patch_insn(loader, off as u32 & 0x03ff_ffff, 0x03ff_ffff)
        }
        R_AARCH64_ADR_PREL_PG_HI21 => {
            let off = ((s_a & !0xfff) as i64) - ((place & !0xfff) as i64);
            let imm = (off >> 12) as u32;
            // ADRP: immlo at bits 29-30, immhi at bits 5-23.
            let bits = ((imm & 0x3) << 29) | (((imm >> 2) & 0x7_ffff) << 5);
            patch_insn(loader, bits, (0x3 << 29) | (0x7_ffff << 5))
        }
        R_AARCH64_ADD_ABS_LO12_NC => patch_insn(loader, ((s_a & 0xfff) as u32) << 10, 0xfff << 10),
        R_AARCH64_LDST8_ABS_LO12_NC => {
            patch_insn(loader, ((s_a & 0xfff) as u32) << 10, 0xfff << 10)
        }
        R_AARCH64_LDST16_ABS_LO12_NC => {
            patch_insn(loader, (((s_a & 0xfff) >> 1) as u32) << 10, 0xfff << 10)
        }
        R_AARCH64_LDST32_ABS_LO12_NC => {
            patch_insn(loader, (((s_a & 0xfff) >> 2) as u32) << 10, 0xfff << 10)
        }
        R_AARCH64_LDST64_ABS_LO12_NC => {
            patch_insn(loader, (((s_a & 0xfff) >> 3) as u32) << 10, 0xfff << 10)
        }
        R_AARCH64_LDST128_ABS_LO12_NC => {
            patch_insn(loader, (((s_a & 0xfff) >> 4) as u32) << 10, 0xfff << 10)
        }
        _ => false,
    }
}
