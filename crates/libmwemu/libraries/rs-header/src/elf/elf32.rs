use crate::elf::loader::{ElfLoader, Perm};
use crate::elf::ElfError;

// Load address / segment type previously sourced from
// `libmwemu::windows::constants`. Kept here so the parser is self-contained.
const PT_LOAD: u32 = 1;
const ELF32_DYN_BASE: u64 = 0x56555000;

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

pub const EI_NIDENT: usize = 16;
pub const ELFCLASS32: u8 = 0x01;

#[derive(Debug)]
pub struct Elf32 {
    pub bin: Vec<u8>,
    pub elf_hdr: Elf32Ehdr,
    pub elf_phdr: Vec<Elf32Phdr>,
    pub elf_shdr: Vec<Elf32Shdr>,
    pub base: u64,
}

impl Elf32 {
    /// Parse an ELF32 image from its raw bytes. Program/section headers are
    /// walked later, in [`Elf32::load`].
    pub fn parse(raw: &[u8]) -> Result<Elf32, ElfError> {
        if raw.len() < 52 {
            return Err(ElfError::new("elf32 image too small for its header"));
        }
        let bin = raw.to_vec();

        let ehdr: Elf32Ehdr = Elf32Ehdr::parse(&bin);

        Ok(Elf32 {
            bin,
            elf_hdr: ehdr,
            elf_phdr: Vec::new(),
            elf_shdr: Vec::new(),
            base: 0,
        })
    }

    pub fn is_dynamic(&self) -> bool {
        const PT_DYNAMIC: u32 = 2;
        self.elf_phdr.iter().any(|ph| ph.p_type == PT_DYNAMIC)
    }

    /// Return the base address used when loading this binary.
    /// Valid after `load()` has been called.
    pub fn base(&self) -> u64 {
        self.base
    }

    pub fn load<L: ElfLoader>(&mut self, loader: &mut L) {
        let mut off = self.elf_hdr.e_phoff as usize;

        for _ in 0..self.elf_hdr.e_phnum {
            let phdr: Elf32Phdr = Elf32Phdr::parse(&self.bin, off);
            self.elf_phdr.push(phdr);
            off += self.elf_hdr.e_phentsize as usize;
        }

        off = self.elf_hdr.e_shoff as usize;

        for _ in 0..self.elf_hdr.e_shnum {
            let shdr: Elf32Shdr = Elf32Shdr::parse(&self.bin, off);
            self.elf_shdr.push(shdr);
            off += self.elf_hdr.e_shentsize as usize;
        }

        // Dynamic/PIE ELF32 binaries have segments starting at vaddr 0;
        // rebase them to a sensible load address so we never write to 0x0.
        let base: u64 = if self.is_dynamic() {
            ELF32_DYN_BASE
        } else {
            0
        };
        self.base = base;

        let mut seg_idx = 0u32;
        for phdr in &self.elf_phdr {
            if phdr.p_type == PT_LOAD {
                let vaddr = (phdr.p_vaddr as u64) + base;

                // Convert ELF p_flags (PF_X=1, PF_W=2, PF_R=4) to Permission
                // (READ=1, WRITE=2, EXECUTE=4). R and X bits are swapped.
                let elf_r = phdr.p_flags & 4 != 0;
                let elf_w = phdr.p_flags & 2 != 0;
                let elf_x = phdr.p_flags & 1 != 0;
                let final_perm = Perm::from_flags(elf_r, elf_w, elf_x);

                // The loader maps the segment (write_bytes bypasses the final
                // permissions when populating it) and reports where it landed.
                let seg_addr = match loader.map(
                    &format!("elf32_seg{}", seg_idx),
                    vaddr,
                    phdr.p_memsz as u64,
                    final_perm,
                ) {
                    Some(addr) => addr,
                    None => {
                        log::warn!("elf32: cannot map segment {} at 0x{:x}", seg_idx, vaddr);
                        seg_idx += 1;
                        continue;
                    }
                };
                seg_idx += 1;

                if phdr.p_filesz > phdr.p_memsz {
                    log::trace!("p_filesz > p_memsz bigger in file than in memory.");
                }
                log::trace!(
                    "segment {} - {}",
                    phdr.p_offset,
                    (phdr.p_offset + phdr.p_filesz)
                );
                let segment =
                    &self.bin[phdr.p_offset as usize..(phdr.p_offset + phdr.p_filesz) as usize];
                loader.write_bytes(seg_addr, segment);
            }
        }
    }

    /// Identify an ELF32 image from its leading bytes (magic + `ELFCLASS32`).
    pub fn is_elf32(raw: &[u8]) -> bool {
        raw.len() >= 5
            && raw[0] == 0x7f
            && raw[1] == b'E'
            && raw[2] == b'L'
            && raw[3] == b'F'
            && raw[4] == ELFCLASS32
    }
}

#[derive(Debug)]
pub struct Elf32Ehdr {
    pub e_ident: [u8; EI_NIDENT],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl Elf32Ehdr {
    pub fn new() -> Elf32Ehdr {
        Elf32Ehdr {
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

    pub fn parse(bin: &[u8]) -> Elf32Ehdr {
        Elf32Ehdr {
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
            e_entry: read_u32_le!(bin, 24),
            e_phoff: read_u32_le!(bin, 28),
            e_shoff: read_u32_le!(bin, 32),
            e_flags: read_u32_le!(bin, 36),
            e_ehsize: read_u16_le!(bin, 40),
            e_phentsize: read_u16_le!(bin, 42),
            e_phnum: read_u16_le!(bin, 44),
            e_shentsize: read_u16_le!(bin, 46),
            e_shnum: read_u16_le!(bin, 48),
            e_shstrndx: read_u16_le!(bin, 50),
        }
    }
}

#[derive(Debug)]
pub struct Elf32Phdr {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_filesz: u32,
    pub p_memsz: u32,
    pub p_flags: u32,
    pub p_align: u32,
}

impl Elf32Phdr {
    pub fn parse(bin: &[u8], phoff: usize) -> Elf32Phdr {
        Elf32Phdr {
            p_type: read_u32_le!(bin, phoff),
            p_offset: read_u32_le!(bin, phoff + 4),
            p_vaddr: read_u32_le!(bin, phoff + 8),
            p_paddr: read_u32_le!(bin, phoff + 12),
            p_filesz: read_u32_le!(bin, phoff + 16),
            p_memsz: read_u32_le!(bin, phoff + 20),
            p_flags: read_u32_le!(bin, phoff + 24),
            p_align: read_u32_le!(bin, phoff + 28),
        }
    }
}

#[derive(Debug)]
pub struct Elf32Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u32,
    pub sh_addr: u32,
    pub sh_offset: u32,
    pub sh_size: u32,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u32,
    pub sh_entsize: u32,
}

impl Elf32Shdr {
    pub fn parse(bin: &[u8], shoff: usize) -> Elf32Shdr {
        Elf32Shdr {
            sh_name: read_u32_le!(bin, shoff),
            sh_type: read_u32_le!(bin, shoff + 4),
            sh_flags: read_u32_le!(bin, shoff + 8),
            sh_addr: read_u32_le!(bin, shoff + 12),
            sh_offset: read_u32_le!(bin, shoff + 16),
            sh_size: read_u32_le!(bin, shoff + 20),
            sh_link: read_u32_le!(bin, shoff + 24),
            sh_info: read_u32_le!(bin, shoff + 28),
            sh_addralign: read_u32_le!(bin, shoff + 32),
            sh_entsize: read_u32_le!(bin, 36),
        }
    }
}
