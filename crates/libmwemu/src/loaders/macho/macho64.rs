use crate::err::MwemuError;
use crate::maps::Maps;
use crate::maps::mem64::Permission;
use goblin::mach::load_command::CommandVariant;
use std::fs;
use std::io::Read;

const MH_MAGIC_64: u32 = 0xFEEDFACF;
const FAT_MAGIC_64: u32 = 0xBEBAFECA; // = CAFEBABE
const CPU_TYPE_ARM64: u32 = 0x0100000C;
const CPU_TYPE_X86_64: u32 = 0x01000007;

// Chained fixup pointer format constants
const DYLD_CHAINED_PTR_64_OFFSET: u16 = 6;

#[derive(Debug)]
pub struct Macho64Segment {
    pub name: String,
    pub vmaddr: u64,
    pub vmsize: u64,
    pub data: Vec<u8>,
    pub initprot: u32, // VM_PROT_READ=1, VM_PROT_WRITE=2, VM_PROT_EXECUTE=4
}

#[derive(Debug)]
pub struct Macho64 {
    pub bin: Vec<u8>,
    pub entry: u64,
    pub segments: Vec<Macho64Segment>,
    pub addr_to_symbol: std::collections::HashMap<u64, String>,
    /// Byte offset of the active Mach-O slice inside `bin`. `0` for plain
    /// Mach-O containers; non-zero when `bin` is a FAT (CAFEBABE) container
    /// and we picked an arch slice further into the file. Load-command
    /// `dataoff`/`fileoff` fields are relative to the *slice* start, so any
    /// raw indexing into `bin` for those needs to add this offset.
    pub slice_offset: u64,
}

/// A resolved chained fixup bind entry: GOT address -> import ordinal
#[derive(Debug)]
pub struct ChainedBind {
    pub got_vmaddr: u64,
    pub import_ordinal: u32,
}

/// An import from the chained fixups import table
#[derive(Debug)]
pub struct ChainedImport {
    pub name: String,
    pub lib_ordinal: i8,
    pub weak: bool,
}

impl Macho64 {
    fn read_magic_and_cputype(filename: &str) -> Option<(u32, u32)> {
        let mut f = match std::fs::File::open(filename) {
            Ok(f) => f,
            Err(_) => return None,
        };
        let mut buf = [0u8; 8];
        if f.read_exact(&mut buf).is_err() {
            return None;
        }
        let magic = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let cputype = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
        Some((magic, cputype))
    }

    /// Detect a 64-bit AArch64 Mach-O file by reading the first 8 bytes.
    pub fn is_macho64_aarch64(filename: &str) -> bool {
        if let Some((magic, cpu)) = Self::read_magic_and_cputype(filename) {
            if magic == MH_MAGIC_64 && cpu == CPU_TYPE_ARM64 {
                return true;
            }

            if magic == FAT_MAGIC_64 {
                return true;
            }
        }

        false
    }

    /// Detect a 64-bit x86_64 Mach-O file by reading the first 8 bytes.
    pub fn is_macho64_x64(filename: &str) -> bool {
        matches!(
            Self::read_magic_and_cputype(filename),
            Some((MH_MAGIC_64, CPU_TYPE_X86_64))
        )
    }

    /// Parse a 64-bit Mach-O binary using goblin.
    /// Supports plain Mach-O and FAT/universal binaries.
    pub fn parse(filename: &str) -> Result<Macho64, MwemuError> {
        use goblin::mach::Mach;

        let bin = fs::read(filename)
            .map_err(|e| MwemuError::new(&format!("cannot read macho binary: {}", e)))?;

        let macho = Mach::parse(&bin)
            .map_err(|e| MwemuError::new(&format!("cannot parse macho binary: {}", e)))?;

        let mut slice_offset: u64 = 0;
        let macho = match macho {
            Mach::Binary(m) => m,

            Mach::Fat(fat) => {
                let mut selected = None;

                for arch in fat.iter_arches() {
                    let arch = arch
                        .map_err(|e| MwemuError::new(&format!("cannot parse fat arch: {}", e)))?;

                    if arch.cputype == goblin::mach::constants::cputype::CPU_TYPE_ARM64 {
                        slice_offset = arch.offset as u64;
                        selected = Some(
                            goblin::mach::MachO::parse(&bin, arch.offset as usize).map_err(
                                |e| MwemuError::new(&format!("cannot parse arm64 slice: {}", e)),
                            )?,
                        );
                        break;
                    }
                }

                selected.ok_or_else(|| MwemuError::new("no ARM64 slice found in FAT Mach-O"))?
            }
        };

        if !macho.is_64 {
            return Err(MwemuError::new("not a 64-bit Mach-O"));
        }

        let entry = macho.entry;

        let mut segments = Vec::new();

        for segment in &macho.segments {
            let name = segment
                .name()
                .map_err(|e| MwemuError::new(&format!("cannot read segment name: {}", e)))?
                .to_string();

            if name == "__PAGEZERO" {
                continue;
            }

            // Extract segment bytes ourselves instead of using `segment.data`.
            // For FAT (CAFEBABE) containers goblin's `segment.data` is sliced
            // from offset 0 of the *whole* bin rather than the active slice,
            // so for the arm64 slice of /bin/ls we'd otherwise see __TEXT
            // start with `ca fe ba be …` (the FAT magic) and __LINKEDIT
            // start with the second slice's MH_MAGIC_64 — neither what we
            // want. `fileoff`/`filesize` are relative to the slice, so add
            // `slice_offset` to land in the right bytes for both plain and
            // FAT containers.
            let slice_start = slice_offset as usize + segment.fileoff as usize;
            let slice_len = segment.filesize as usize;
            let slice_end = slice_start.saturating_add(slice_len);
            let data = if slice_len == 0 || slice_end > bin.len() {
                Vec::new()
            } else {
                bin[slice_start..slice_end].to_vec()
            };

            segments.push(Macho64Segment {
                name,
                vmaddr: segment.vmaddr,
                vmsize: segment.vmsize,
                data,
                initprot: segment.initprot,
            });
        }

        Ok(Macho64 {
            bin,
            entry,
            segments,
            addr_to_symbol: std::collections::HashMap::new(),
            slice_offset,
        })
    }

    /// Parse a 64-bit Mach-O binary using goblin.
    pub fn parse_prev(filename: &str) -> Result<Macho64, MwemuError> {
        let bin = fs::read(filename)
            .map_err(|e| MwemuError::new(&format!("cannot read macho binary: {}", e)))?;

        let macho = goblin::mach::MachO::parse(&bin, 0)
            .map_err(|e| MwemuError::new(&format!("cannot parse macho binary: {}", e)))?;

        if !macho.is_64 {
            return Err(MwemuError::new("not a 64-bit Mach-O"));
        }

        let entry = macho.entry;

        let mut segments = Vec::new();
        for segment in &macho.segments {
            let name = segment
                .name()
                .map_err(|e| MwemuError::new(&format!("cannot read segment name: {}", e)))?
                .to_string();

            // __PAGEZERO is a guard page with no file data — skip it
            if name == "__PAGEZERO" {
                continue;
            }

            segments.push(Macho64Segment {
                name,
                vmaddr: segment.vmaddr,
                vmsize: segment.vmsize,
                data: segment.data.to_vec(),
                initprot: segment.initprot,
            });
        }

        Ok(Macho64 {
            bin,
            entry,
            segments,
            addr_to_symbol: std::collections::HashMap::new(),
            slice_offset: 0,
        })
    }

    /// Load parsed segments into emulator memory maps.
    pub fn load(&self, maps: &mut Maps) {
        for seg in &self.segments {
            if seg.vmsize == 0 {
                continue;
            }

            let perm = prot_to_permission(seg.initprot);

            let mem = maps
                .create_map(&seg.name, seg.vmaddr, seg.vmsize, perm)
                .unwrap_or_else(|_| panic!("cannot create map for segment '{}' at 0x{:x}",
                    seg.name, seg.vmaddr));

            if !seg.data.is_empty() {
                mem.force_write_bytes(seg.vmaddr, &seg.data);
            }
        }
    }

    /// Get the list of dependent dylib paths from load commands.
    /// Supports plain Mach-O and FAT/universal binaries.
    pub fn get_libs(&self) -> Vec<String> {
        let macho = self.reparse().expect("re-parse for libs");
        macho
            .libs
            .iter()
            .filter(|l| **l != "self")
            .map(|l| l.to_string())
            .collect()
    }

    /// Re-parse `self.bin` returning the inner 64-bit slice. Handles both
    /// `Mach::Binary` (regular Mach-O) and `Mach::Fat` (CAFEBABE) — for FAT
    /// containers we pick the `arm64` slice (matches the slice we kept at
    /// load time in `parse_prev` / `parse`). Use this from all helpers that
    /// need a `goblin::mach::MachO` view of the same bytes; calling
    /// `MachO::parse(&bin, 0)` directly panics with `BadMagic(0xCAFEBABE)`
    /// when `bin` is a FAT container.
    fn reparse(&self) -> Result<goblin::mach::MachO<'_>, MwemuError> {
        use goblin::mach::Mach;

        match Mach::parse(&self.bin)
            .map_err(|e| MwemuError::new(&format!("cannot parse mach: {}", e)))?
        {
            Mach::Binary(m) => Ok(m),
            Mach::Fat(fat) => {
                for arch in fat.iter_arches() {
                    let arch = arch
                        .map_err(|e| MwemuError::new(&format!("cannot parse fat arch: {}", e)))?;
                    if arch.cputype == goblin::mach::constants::cputype::CPU_TYPE_ARM64 {
                        return goblin::mach::MachO::parse(&self.bin, arch.offset as usize)
                            .map_err(|e| {
                                MwemuError::new(&format!("cannot parse arm64 slice: {}", e))
                            });
                    }
                }
                Err(MwemuError::new("no ARM64 slice found in FAT Mach-O"))
            }
        }
    }

    /// Get the list of dependent dylib paths from load commands.
    pub fn get_libs_prev(&self) -> Vec<String> {
        let macho = self.reparse().expect("re-parse for libs");
        macho
            .libs
            .iter()
            .filter(|l| **l != "self")
            .map(|l| l.to_string())
            .collect()
    }

    /// Get exported symbols with their offsets (relative to binary load address).
    pub fn get_exports(&self) -> Vec<(String, u64)> {
        let macho = self.reparse().expect("re-parse for exports");
        match macho.exports() {
            Ok(exports) => exports.iter().map(|e| (e.name.clone(), e.offset)).collect(),
            Err(e) => {
                log::warn!("macho64: cannot read exports: {}", e);
                Vec::new()
            }
        }
    }

    /// Parse chained fixups to extract imports and their GOT bind locations.
    /// Returns (imports_table, bind_entries) where each bind entry references
    /// an import by ordinal and specifies the GOT vmaddr to patch.
    pub fn parse_chained_fixups(&self) -> (Vec<ChainedImport>, Vec<ChainedBind>) {
        let macho = self.reparse().expect("re-parse for fixups");

        let mut imports = Vec::new();
        let mut binds = Vec::new();

        for lc in &macho.load_commands {
            if let CommandVariant::DyldChainedFixups(cmd) = &lc.command {
                // `dataoff` is relative to the Mach-O slice start. For FAT
                // (CAFEBABE) containers the slice lives at `slice_offset`
                // inside `bin`; without this adjustment we'd index into the
                // FAT header bytes (or far past the end of `bin`).
                let base = self.slice_offset as usize + cmd.dataoff as usize;
                let end = base.saturating_add(cmd.datasize as usize);
                if end > self.bin.len() {
                    log::warn!(
                        "macho64: chained fixups data range [{:#x}..{:#x}] exceeds bin len {:#x}",
                        base,
                        end,
                        self.bin.len()
                    );
                    continue;
                }
                let data = &self.bin[base..end];

                // dyld_chained_fixups_header
                let starts_offset = u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize;
                let imports_offset = u32::from_le_bytes(data[8..12].try_into().unwrap()) as usize;
                let symbols_offset = u32::from_le_bytes(data[12..16].try_into().unwrap()) as usize;
                let imports_count = u32::from_le_bytes(data[16..20].try_into().unwrap());
                let imports_format = u32::from_le_bytes(data[20..24].try_into().unwrap());

                // Parse import table
                for i in 0..imports_count {
                    let imp = match imports_format {
                        1 => {
                            // DYLD_CHAINED_IMPORT: lib_ordinal:8, weak_import:1, name_offset:23
                            let off = imports_offset + (i as usize * 4);
                            let raw = u32::from_le_bytes(data[off..off + 4].try_into().unwrap());
                            let lib_ordinal = (raw & 0xFF) as i8;
                            let weak = (raw >> 8) & 1 != 0;
                            let name_offset = (raw >> 9) as usize;
                            let name_start = symbols_offset + name_offset;
                            let name_end =
                                data[name_start..].iter().position(|b| *b == 0).unwrap_or(0)
                                    + name_start;
                            let name = std::str::from_utf8(&data[name_start..name_end])
                                .unwrap_or("")
                                .to_string();
                            ChainedImport {
                                name,
                                lib_ordinal,
                                weak,
                            }
                        }
                        _ => {
                            log::warn!(
                                "macho64: unsupported chained import format {}",
                                imports_format
                            );
                            break;
                        }
                    };
                    imports.push(imp);
                }

                // Parse starts_in_image to walk chains and find BIND entries
                let seg_count =
                    u32::from_le_bytes(data[starts_offset..starts_offset + 4].try_into().unwrap())
                        as usize;

                for seg_idx in 0..seg_count {
                    let seg_info_off = u32::from_le_bytes(
                        data[starts_offset + 4 + seg_idx * 4..starts_offset + 8 + seg_idx * 4]
                            .try_into()
                            .unwrap(),
                    );
                    if seg_info_off == 0 {
                        continue;
                    }
                    let si = starts_offset + seg_info_off as usize;

                    let page_size =
                        u16::from_le_bytes(data[si + 4..si + 6].try_into().unwrap()) as usize;
                    let pointer_format =
                        u16::from_le_bytes(data[si + 6..si + 8].try_into().unwrap());
                    let page_count =
                        u16::from_le_bytes(data[si + 20..si + 22].try_into().unwrap()) as usize;

                    // Get segment vmaddr and fileoff from goblin's segment list
                    let (seg_vmaddr, seg_fileoff) = macho
                        .segments
                        .get(seg_idx)
                        .map(|s| (s.vmaddr, s.fileoff as usize))
                        .unwrap_or((0, 0));

                    for p in 0..page_count {
                        let page_start = u16::from_le_bytes(
                            data[si + 22 + p * 2..si + 24 + p * 2].try_into().unwrap(),
                        );
                        if page_start == 0xFFFF {
                            continue;
                        }

                        // `seg_fileoff` is relative to the Mach-O slice; for
                        // FAT (CAFEBABE) containers we need to index into the
                        // outer `self.bin` so add `slice_offset`.
                        let chain_file_base = self.slice_offset as usize
                            + seg_fileoff
                            + p * page_size
                            + page_start as usize;
                        let chain_vm_base =
                            seg_vmaddr + (p as u64 * page_size as u64) + page_start as u64;

                        self.walk_chain(pointer_format, chain_file_base, chain_vm_base, &mut binds);
                    }
                }
            }
        }

        (imports, binds)
    }

    /// Walk a chain of fixup pointers starting at file_off/vmaddr.
    fn walk_chain(
        &self,
        pointer_format: u16,
        mut file_off: usize,
        mut vmaddr: u64,
        binds: &mut Vec<ChainedBind>,
    ) {
        loop {
            if file_off + 8 > self.bin.len() {
                break;
            }
            let raw = u64::from_le_bytes(self.bin[file_off..file_off + 8].try_into().unwrap());

            // Per `dyld_chained_fixups.h`. We only need bind status, ordinal,
            // and stride to chain forward; rebase targets are baked into the
            // raw bytes already and don't need rewriting for our purposes.
            let (bind, next, ordinal, stride) = match pointer_format {
                // DYLD_CHAINED_PTR_ARM64E (format 1) — used by macOS arm64e
                // userland binaries like /bin/ls. 8-byte stride, layout:
                //   bind:1 (bit 62), auth:1 (bit 63), next:11 (bits 51..61).
                //   bind variant: ordinal:16 in bits 0..15 (auth uses same 16
                //   bits for ordinal).
                1 => {
                    let bind = (raw >> 62) & 1;
                    let next = ((raw >> 51) & 0x7FF) as usize;
                    let ordinal = (raw & 0xFFFF) as u32;
                    (bind, next, ordinal, 8usize)
                }
                // DYLD_CHAINED_PTR_64 (2) / DYLD_CHAINED_PTR_64_OFFSET (6) —
                // 4-byte stride. bind:1 (bit 63), next:12 (bits 51..62),
                // ordinal:24 (bits 0..23) for bind variant.
                DYLD_CHAINED_PTR_64_OFFSET | 2 => {
                    let bind = raw >> 63;
                    let next = ((raw >> 51) & 0xFFF) as usize;
                    let ordinal = (raw & 0xFFFFFF) as u32;
                    (bind, next, ordinal, 4usize)
                }
                // DYLD_CHAINED_PTR_ARM64E_USERLAND24 (12) — 8-byte stride,
                // 24-bit ordinal in the bind variant.
                12 => {
                    let bind = (raw >> 62) & 1;
                    let next = ((raw >> 52) & 0x7FF) as usize;
                    let ordinal = (raw & 0xFFFFFF) as u32;
                    (bind, next, ordinal, 8usize)
                }
                _ => {
                    log::warn!(
                        "macho64: unsupported chained pointer format {} at 0x{:x}",
                        pointer_format,
                        vmaddr
                    );
                    break;
                }
            };

            if bind == 1 {
                binds.push(ChainedBind {
                    got_vmaddr: vmaddr,
                    import_ordinal: ordinal,
                });
            }

            if next == 0 {
                break;
            }
            file_off += next * stride;
            vmaddr += (next * stride) as u64;
        }
    }
}

pub(crate) fn prot_to_permission(prot: u32) -> Permission {
    let r = prot & 1 != 0; // VM_PROT_READ
    let w = prot & 2 != 0; // VM_PROT_WRITE
    let x = prot & 4 != 0; // VM_PROT_EXECUTE

    match (r, w, x) {
        (true, true, true) => Permission::READ_WRITE_EXECUTE,
        (true, true, false) => Permission::READ_WRITE,
        (true, false, true) => Permission::READ_EXECUTE,
        (true, false, false) => Permission::READ,
        (false, true, false) => Permission::WRITE,
        (false, false, true) => Permission::EXECUTE,
        _ => Permission::READ_WRITE, // fallback
    }
}
