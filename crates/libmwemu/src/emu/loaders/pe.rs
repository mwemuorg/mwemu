use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::windows::constants;
use crate::windows::peb::{peb32, peb64};
use rs_header::pe::pe32::PE32;
use rs_header::pe::pe64::PE64;

macro_rules! align_up {
    ($size:expr, $align:expr) => {{
        // Ensure alignment is a power of two at compile time if possible
        ($size + $align - 1) & !($align - 1)
    }};
}

impl Emu {
    /// Read a PE image off disk. This is plain I/O — all interpretation of the
    /// bytes is done by `rs-header`; libmwemu never parses the raw itself.
    fn read_pe_raw(filename: &str) -> Vec<u8> {
        std::fs::read(filename)
            .unwrap_or_else(|e| panic!("cannot read PE file {}: {}", filename, e))
    }

    /// Prefer PE `ImageBase` when it is in canonical user space and does not overlap existing maps;
    /// otherwise fall back to `lib64_alloc` in `LIBS64_*`.
    /// `raw_len` is the on-disk image size (rs-header's borrow-based PE no longer
    /// owns the bytes, so the caller supplies it).
    fn pick_pe64_dll_base(&mut self, pe64: &PE64, raw_len: u64) -> u64 {
        const USER_MAX: u64 = 0x7FFF_FFFF_FFFF;
        let ib = pe64.opt.image_base;
        let span = (pe64.opt.size_of_image as u64).max(raw_len);
        if ib < 0x10000 {
            return self.maps.lib64_alloc(raw_len).expect("out of memory");
        }
        let Some(end) = ib.checked_add(span) else {
            return self.maps.lib64_alloc(raw_len).expect("out of memory");
        };
        if end > USER_MAX || self.maps.overlaps(ib, span) {
            return self.maps.lib64_alloc(raw_len).expect("out of memory");
        }
        ib
    }

    /// Complex funtion called from many places and with multiple purposes.
    /// This is called from load_code() if sample is PE32, but also from load_library etc.
    /// cyclic stuff: [load_pe] -> [iat-binding]  ->  [load_library] -> [load_pe]
    /// Powered by rs-header's pe32 implementation.
    pub fn load_pe32(&mut self, filename: &str, set_entry: bool, force_base: u32) -> (u32, u32) {
        let is_maps = filename.contains("windows/x86/");
        let map_name = self.filename_to_mapname(filename);
        let filename2 = map_name;
        let raw = Self::read_pe_raw(filename);
        let mut pe32 = PE32::parse(filename, &raw);
        let raw_len = raw.len() as u64;
        let base: u32;

        log::trace!("loading pe32 {}", filename);

        // 1. base logic

        // base is forced by libmwemu
        if force_base > 0 {
            if self.maps.overlaps(force_base as u64, raw_len) {
                panic!("the forced base address overlaps");
            } else {
                base = force_base;
            }

        // base is setted by user
        } else if !is_maps
            && self.cfg.code_base_addr != constants::CFG_DEFAULT_BASE
            && !self.cfg.emulate_winapi
        {
            base = self.cfg.code_base_addr as u32;
            if self.maps.overlaps(base as u64, raw_len) {
                panic!("the setted base address overlaps");
            }

        // base is setted by image base (if overlapps, alloc)
        } else {
            // user's program
            if set_entry {
                if pe32.opt.image_base >= constants::LIBS32_MIN as u32
                    || self
                        .maps
                        .overlaps(pe32.opt.image_base as u64, pe32.mem_size() as u64)
                {
                    base = self
                        .maps
                        .alloc(pe32.mem_size() as u64 + 0xff)
                        .expect("out of memory") as u32;
                } else {
                    base = pe32.opt.image_base;
                }

            // system library
            } else {
                base = self
                    .maps
                    .lib32_alloc(pe32.mem_size() as u64)
                    .expect("out of memory") as u32;
            }
        }

        if set_entry || self.cfg.emulate_winapi {
            // 2. own this image (binding now happens after the sections are
            // mapped — rs-header's binding patches the *mapped* IAT in guest
            // memory, not the raw buffer, so the slots must exist first).
            if !is_maps || self.cfg.emulate_winapi {
                self.base = base as u64;
            }

            // 3. entry point logic
            if self.cfg.entry_point == constants::CFG_DEFAULT_BASE {
                self.regs_mut().rip = base as u64 + pe32.opt.address_of_entry_point as u64;
                log::trace!("entry point at 0x{:x}", self.regs().rip);
            } else {
                self.regs_mut().rip = self.cfg.entry_point;
                log::trace!(
                    "entry point at 0x{:x} but forcing it at 0x{:x}",
                    base as u64 + pe32.opt.address_of_entry_point as u64,
                    self.regs().rip
                );
            }

            log::trace!("base: 0x{:x}", base);
        }

        let sec_allign = pe32.opt.section_alignment;
        // 4. map pe and then sections
        let pemap = self
            .maps
            .create_map(
                &format!("{}.pe", filename2),
                base.into(),
                align_up!(pe32.opt.size_of_headers, sec_allign) as u64,
                Permission::READ_WRITE,
            )
            .expect("cannot create pe map");
        pemap.memcpy(pe32.headers(&raw), pe32.opt.size_of_headers as usize);

        for i in 0..pe32.num_of_sections() {
            let ptr = pe32.get_section_ptr(&raw, i);
            let sect = pe32.get_section(i);
            let charactis = sect.characteristics;
            let is_exec = charactis & 0x20000000 != 0x0;
            let is_read = charactis & 0x40000000 != 0x0;
            let mut is_write = charactis & 0x80000000 != 0x0;
            // The loader patches `.didat` (delay-load IAT) during init without
            // calling NtProtectVirtualMemory first — it relies on SEC_IMAGE COW
            // promotion, which the emulator does not model. Force RW so the
            // page-touching helper in ntdll does not fault.
            if sect.get_name().trim() == ".didat" {
                is_write = true;
            }
            let permission = Permission::from_flags(is_read, is_write, is_exec);

            let sz: u64 = if sect.virtual_size > sect.size_of_raw_data {
                sect.virtual_size as u64
            } else {
                sect.size_of_raw_data as u64
            };

            if sz == 0 {
                log::trace!("size of section {} is 0", sect.get_name());
                continue;
            }

            let mut sect_name = sect
                .get_name()
                .replace(" ", "")
                .replace("\t", "")
                .replace("\x0a", "")
                .replace("\x0d", "");

            if sect_name.is_empty() {
                sect_name = format!("{:x}", sect.virtual_address);
            }

            let map = match self.maps.create_map(
                &format!("{}{}", filename2, sect_name),
                base as u64 + sect.virtual_address as u64,
                align_up!(sz, sec_allign as u64),
                permission,
            ) {
                Ok(m) => m,
                Err(_e) => {
                    log::trace!(
                        "weird pe, skipping section {} {} because overlaps",
                        filename2,
                        sect.get_name()
                    );
                    continue;
                }
            };

            if ptr.len() > sz as usize {
                panic!(
                    "overflow {} {} {} {}",
                    filename2,
                    sect.get_name(),
                    ptr.len(),
                    sz
                );
            }
            if !ptr.is_empty() {
                map.memcpy(ptr, ptr.len());
            }
        }

        // 4b. pe binding — sections (incl. the IAT) are mapped now.
        if (set_entry || self.cfg.emulate_winapi) && (!is_maps || self.cfg.emulate_winapi) {
            pe32.iat_binding(&raw, self, base);
            pe32.delay_load_binding(&raw, self, base);
        }

        // 5. ldr table entry creation and link
        if set_entry {
            let _space_addr = peb32::create_ldr_entry(
                self,
                base,
                self.regs().rip as u32,
                &filename2,
                0,
                0x2c1950,
            );
            let exe_name = self.cfg.exe_name.clone();
            peb32::update_ldr_entry_base(&exe_name, base as u64, self);
        }

        // 6. return values
        let pe_hdr_off = pe32.dos.e_lfanew;
        self.pe32 = Some(pe32);
        self.pe32_raw = Some(raw);
        (base, pe_hdr_off)
    }

    pub fn map_dll_pe64(&mut self, filename: &str) -> (u64, PE64, Vec<u8>) {
        let map_name = self.filename_to_mapname(filename);
        let raw = Self::read_pe_raw(&filename.to_lowercase());
        let pe64 = PE64::parse(&filename.to_lowercase(), &raw);
        let raw_len = raw.len() as u64;

        let base = self.pick_pe64_dll_base(&pe64, raw_len);

        let sec_allign = pe64.opt.section_alignment;

        let pemap = match self.maps.create_map(
            &format!("{}.pe", map_name),
            base,
            align_up!(pe64.opt.size_of_headers, sec_allign) as u64,
            Permission::READ_WRITE,
        ) {
            Ok(m) => m,
            Err(e) => {
                panic!("cannot create pe64 map: {}", e);
            }
        };
        pemap.memcpy(pe64.headers(&raw), pe64.opt.size_of_headers as usize);
        for i in 0..pe64.num_of_sections() {
            let ptr = pe64.get_section_ptr(&raw, i);
            let sect = pe64.get_section(i);
            let charistic = sect.characteristics;
            let is_exec = charistic & 0x20000000 != 0x0;
            let is_read = charistic & 0x40000000 != 0x0;
            let mut is_write = charistic & 0x80000000 != 0x0;
            // The loader patches `.didat` during init without calling
            // NtProtectVirtualMemory first — relies on SEC_IMAGE COW we do
            // not model. Force RW so ntdll's page-touching helper succeeds.
            if sect.get_name().trim() == ".didat" {
                is_write = true;
            }
            let permission = Permission::from_flags(is_read, is_write, is_exec);

            let map_sz: u64 = if sect.virtual_size > 0 {
                sect.virtual_size as u64
            } else {
                sect.size_of_raw_data as u64
            };

            if map_sz == 0 {
                log::trace!("size of section {} is 0", sect.get_name());
                continue;
            }

            let mut sect_name = sect
                .get_name()
                .replace(" ", "")
                .replace("\t", "")
                .replace("\x0a", "")
                .replace("\x0d", "");

            if sect_name.is_empty() {
                sect_name = format!("{:x}", sect.virtual_address);
            }

            let map = match self.maps.create_map(
                &format!("{}{}", map_name, sect_name),
                base + sect.virtual_address as u64,
                align_up!(map_sz, sec_allign as u64),
                permission,
            ) {
                Ok(m) => m,
                Err(_e) => {
                    log::trace!(
                        "weird pe, skipping section because overlaps {} {}",
                        map_name,
                        sect.get_name()
                    );
                    continue;
                }
            };

            let copy_len = (sect.size_of_raw_data as usize)
                .min(map_sz as usize)
                .min(ptr.len());
            if copy_len > 0 {
                map.memcpy(&ptr[..copy_len], copy_len);
            }
        }

        pe64.apply_relocations(&raw, self, base);

        (base, pe64, raw)
    }

    /// Complex funtion called from many places and with multiple purposes.
    /// This is called from load_code() if sample is PE64, but also from load_library etc.
    /// cyclic stuff: [load_pe] -> [iat-binding]  ->  [load_library] -> [load_pe]
    /// Powered by rs-header's pe64 implementation.
    pub fn load_pe64(&mut self, filename: &str, set_entry: bool, force_base: u64) -> (u64, u32) {
        let is_maps = filename.contains("windows/x86_64/") || filename.contains("windows/aarch64/");
        let map_name = self.filename_to_mapname(filename);
        let filename2 = map_name;
        let raw = Self::read_pe_raw(filename);
        let mut pe64 = PE64::parse(filename, &raw);
        let raw_len = raw.len() as u64;
        let base: u64;

        // 1. base logic

        // base is setted by libmwemu
        if force_base > 0 {
            if self.maps.overlaps(force_base, raw_len) {
                panic!("the forced base address overlaps");
            } else {
                base = force_base;
            }

        // base is setted by user
        } else if !is_maps && self.cfg.code_base_addr != constants::CFG_DEFAULT_BASE {
            base = self.cfg.code_base_addr;
            if self.maps.overlaps(base, raw_len) {
                panic!("the setted base address overlaps");
            }

        // base is setted by image base (if overlapps, alloc)
        } else {
            // user's program
            if set_entry {
                if pe64.opt.image_base >= constants::LIBS64_MIN {
                    base = self.maps.alloc(raw_len + 0xff).expect("out of memory");
                } else if self.maps.overlaps(pe64.opt.image_base, raw_len) {
                    base = self.maps.alloc(raw_len + 0xff).expect("out of memory");
                } else {
                    base = pe64.opt.image_base;
                }

            // system library
            } else {
                base = self.pick_pe64_dll_base(&pe64, raw_len);
            }
        }

        // Only the main image owns `self.base` and the initial RIP. System DLLs loaded
        // via `load_library` (e.g. NtMapViewOfSection's KnownDll path under --ssdt) must
        // never clobber these — otherwise the post-LdrInitializeThunk IAT binding in
        // loaders.rs reads the DLL's base instead of the EXE's, and the EXE's IAT stays
        // unbound (call rax → rax=0 → crash).
        if set_entry {
            self.base = base;

            // 2. entry point logic (relocs + IAT run after PE maps exist; see step 4b below)
            if self.cfg.entry_point == constants::CFG_DEFAULT_BASE {
                self.set_pc(base + pe64.opt.address_of_entry_point as u64);
                log::trace!("entry point at 0x{:x}", self.pc());
            } else {
                self.set_pc(self.cfg.entry_point);
                log::trace!(
                    "entry point at 0x{:x} but forcing it at 0x{:x} by -a flag",
                    base + pe64.opt.address_of_entry_point as u64,
                    self.pc()
                );
            }
            log::trace!("base: 0x{:x}", base);
        }

        let sec_allign = pe64.opt.section_alignment;
        // 4. map pe and then sections
        let pemap = match self.maps.create_map(
            &format!("{}.pe", filename2),
            base,
            align_up!(pe64.opt.size_of_headers, sec_allign) as u64,
            Permission::READ_WRITE,
        ) {
            Ok(m) => m,
            Err(e) => {
                panic!("cannot create pe64 map: {}", e);
            }
        };
        pemap.memcpy(pe64.headers(&raw), pe64.opt.size_of_headers as usize);

        for i in 0..pe64.num_of_sections() {
            let ptr = pe64.get_section_ptr(&raw, i);
            let sect = pe64.get_section(i);

            let charistic = sect.characteristics;
            let is_exec = charistic & 0x20000000 != 0x0;
            let is_read = charistic & 0x40000000 != 0x0;
            let mut is_write = charistic & 0x80000000 != 0x0;
            // The loader patches `.didat` during init without calling
            // NtProtectVirtualMemory first — relies on SEC_IMAGE COW we do
            // not model. Force RW so ntdll's page-touching helper succeeds.
            if sect.get_name().trim() == ".didat" {
                is_write = true;
            }
            let permission = Permission::from_flags(is_read, is_write, is_exec);

            // Virtual size determines how much address space the section occupies.
            // Raw size is the on-disk data size and may exceed virtual size for
            // packed/overlay sections — using raw size would create an oversized map
            // that overlaps subsequent sections.
            let map_sz: u64 = if sect.virtual_size > 0 {
                sect.virtual_size as u64
            } else {
                sect.size_of_raw_data as u64
            };

            if map_sz == 0 {
                log::trace!("size of section {} is 0", sect.get_name());
                continue;
            }

            let mut sect_name = sect
                .get_name()
                .replace(" ", "")
                .replace("\t", "")
                .replace("\x0a", "")
                .replace("\x0d", "");

            if sect_name.is_empty() {
                sect_name = format!("{:x}", sect.virtual_address);
            }

            let map = match self.maps.create_map(
                &format!("{}{}", filename2, sect_name),
                base + sect.virtual_address as u64,
                align_up!(map_sz, sec_allign as u64),
                permission,
            ) {
                Ok(m) => m,
                Err(_e) => {
                    log::trace!(
                        "weird pe, skipping section because overlaps {} {}",
                        filename2,
                        sect.get_name()
                    );
                    continue;
                }
            };

            // Copy only as many bytes as fit in the virtual mapping.
            let copy_len = (sect.size_of_raw_data as usize)
                .min(map_sz as usize)
                .min(ptr.len());

            if copy_len > 0 {
                map.memcpy(&ptr[..copy_len], copy_len);
            }
        }

        // 4b. Base relocs on the mapped image (all load paths, including DLL without emulate_winapi).
        pe64.apply_relocations(&raw, self, base);

        // Decide whether to eagerly bind this image's IAT.
        let bind_iat = if self.cfg.emulate_winapi {
            // SSDT/syscall mode: the main image's IAT is bound later by the
            // LdrInitializeThunk bootstrap, so skip it here; bind everything else.
            !set_entry
        } else {
            // Default API-stub mode: bind the main image AND every dependency DLL
            // loaded from maps/. mwemu executes the *real* bytes of any exported
            // function it has no stub for (e.g. msvcrt's CRT startup `_initterm`,
            // `__wgetmainargs`), and that real code calls further APIs through this
            // image's own IAT. Pointing those slots at the real export VAs is safe:
            // mwemu still intercepts them via its API gateway. Without this, the
            // dependency's IAT stays unbound and the first such call jumps to the
            // raw import-name RVA and crashes.
            true
        };
        if bind_iat {
            pe64.iat_binding(&raw, self, base);
            pe64.delay_load_binding(&raw, self, base);
        }

        // 5. ldr table entry creation and link
        if set_entry {
            if !self.cfg.emulate_winapi {
                let _space_addr =
                    peb64::create_ldr_entry(self, base, self.pc(), &filename2, 0, 0x2c1950);
                let exe_name = self.cfg.exe_name.clone();
                peb64::update_ldr_entry_base(&exe_name, base, self);
            }
            if self.cfg.emulate_winapi {
                peb64::update_peb_image_base(self, base);
            }
        }

        // 6. return values
        let pe_hdr_off = pe64.dos.e_lfanew;
        self.pe64 = Some(pe64);
        self.pe64_raw = Some(raw);
        (base, pe_hdr_off)
    }
}
