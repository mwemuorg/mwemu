use std::collections::HashMap;
use std::path::Path;

use crate::emu::Emu;
use crate::windows::constants;
use rs_header::elf::ElfError;
use rs_header::elf::elf64::Elf64;

/// Read an ELF64 off disk and parse it through rs-header. Plain I/O here; all
/// byte interpretation lives in rs-header.
fn parse_elf64_file(path: &str) -> Result<Elf64, ElfError> {
    let raw = std::fs::read(path)
        .map_err(|e| ElfError::new(&format!("cannot read {}: {}", path, e)))?;
    Elf64::parse(&raw)
}

impl Emu {
    /// Loads an ELF64 parsing sections etc, powered by elf64.rs
    /// This is called from load_code() if the sample is ELF64
    pub fn load_elf64(&mut self, filename: &str) {
        let mut elf64 = parse_elf64_file(filename).unwrap();
        let dyn_link = !elf64.get_dynamic().is_empty();

        if dyn_link {
            log::trace!("dynamic elf64 detected.");
        } else {
            log::trace!("static elf64 detected.");
        }

        // Decide up front whether the real ld.so will drive the bootstrap: in
        // that case the main executable must also be mapped by PT_LOAD segments
        // so its ELF header / program headers (AT_PHDR) are visible to ld.so.
        let interp = elf64.get_interp();
        // Only drive the real ld.so when its host file is actually available;
        // otherwise gracefully fall back to the in-Rust libc-hook path.
        let host_interp = interp.as_deref().and_then(|p| self.resolve_host_lib(p));
        let ld_bootstrap = dyn_link
            && self.cfg.linux_real_libc
            && !self.cfg.arch.is_aarch64()
            && host_interp.is_some();
        if ld_bootstrap {
            elf64.segment_mode = true;
        }

        elf64.load(
            &mut self.maps,
            "elf64",
            false,
            dyn_link,
            self.cfg.code_base_addr,
        );
        if self.cfg.arch.is_aarch64() {
            self.init_linux64_aarch64();
        } else {
            self.init_linux64(dyn_link);
        }

        // --- Real ld.so bootstrap path (counterpart of --ssdt for Linux) ---
        // If the program has an interpreter and we're in --libc mode, hand the
        // whole job (mapping libc, relocations, TLS, ifunc) to the real ld.so,
        // exactly like the kernel does: map ld.so, build the initial stack with
        // a full auxv, and start executing at ld.so's entry point.
        if ld_bootstrap {
            self.setup_ld_so_bootstrap(&mut elf64, &interp.unwrap());
            self.elf64 = Some(elf64);
            return;
        }

        // --- Dynamic linking: load stub libs and apply relocations ---
        if dyn_link {
            self.load_elf64_dynamic_libs(&mut elf64);
        }

        // Get .text addr and size
        let mut text_addr: u64 = 0;
        let mut text_sz = 0;
        for i in 0..elf64.elf_shdr.len() {
            let sname = elf64.get_section_name(elf64.elf_shdr[i].sh_name as usize);
            if sname == ".text" {
                text_addr = elf64.elf_shdr[i].sh_addr;
                text_sz = elf64.elf_shdr[i].sh_size;
                break;
            }
        }

        // Tiny / hand-crafted ELFs may have no section headers at all (or no
        // `.text` section). Fall back to the first executable PT_LOAD segment.
        if text_addr == 0 {
            for phdr in &elf64.elf_phdr {
                if phdr.p_type == constants::PT_LOAD && (phdr.p_flags & 1) != 0 {
                    text_addr = phdr.p_vaddr;
                    text_sz = phdr.p_memsz;
                    break;
                }
            }
        }

        if text_addr == 0 {
            panic!(".text not found on this elf64");
        }

        // entry point logic:

        // 1. Configured entry point
        if self.cfg.entry_point != constants::CFG_DEFAULT_BASE {
            log::trace!("forcing entry point to 0x{:x}", self.cfg.entry_point);
            self.set_pc(self.cfg.entry_point);

        // 2. Entry point pointing inside .text
        } else if elf64.elf_hdr.e_entry >= text_addr && elf64.elf_hdr.e_entry < text_addr + text_sz
        {
            log::trace!(
                "Entry point pointing to .text 0x{:x}",
                elf64.elf_hdr.e_entry
            );
            self.set_pc(elf64.elf_hdr.e_entry);

        // 3. Entry point points above .text, relative entry point
        } else if elf64.elf_hdr.e_entry < text_addr {
            self.set_pc(elf64.elf_hdr.e_entry + elf64.base);
            log::trace!(
                "relative entry point: 0x{:x}  fixed: 0x{:x}",
                elf64.elf_hdr.e_entry,
                self.pc()
            );

        // 4. Entry point points below .text, weird case.
        } else {
            panic!(
                "Entry points is pointing below .text 0x{:x}",
                elf64.elf_hdr.e_entry
            );
        }

        // Write the Linux initial stack layout (argc, argv, envp, auxv)
        // so _start can read argc/argv and __libc_start_main gets proper args.
        let phdr_addr = elf64.base + elf64.elf_hdr.e_phoff;
        self.write_linux_stack_layout(
            self.pc(),
            phdr_addr,
            elf64.elf_hdr.e_phentsize,
            elf64.elf_hdr.e_phnum,
        );

        self.elf64 = Some(elf64);
    }

    /// Load dynamic libraries for an ELF64 binary.
    /// Loads real stub ELFs from disk, then applies relocations.
    fn load_elf64_dynamic_libs(&mut self, elf64: &mut Elf64) {
        let mut export_map: HashMap<String, u64> = HashMap::new();
        let mut ifunc_resolvers: std::collections::HashSet<u64> = std::collections::HashSet::new();
        let real_libc = self.cfg.linux_real_libc && !self.cfg.arch.is_aarch64();

        let libs = elf64.get_dynamic();
        elf64.needed_libs = libs.clone();

        // Keep the parsed libraries around so that — in real-libc mode — we can
        // also apply each library's *own* relocations (RELATIVE / IRELATIVE /
        // symbolic), which is what `ld.so` would do before handing control to
        // the program. Without it the real libc code dereferences null pointers.
        let mut loaded_libs: Vec<(String, Elf64)> = Vec::new();

        for lib in &libs {
            log::trace!("dynamic library {}", lib);

            let Some(local_path) = self.resolve_linux_stub_path(lib) else {
                log::warn!("elf64: could not locate linux stub library {}", lib);
                continue;
            };

            let mut elflib = match parse_elf64_file(&local_path) {
                Ok(lib) => lib,
                Err(err) => {
                    log::warn!("elf64: failed to parse {}: {}", local_path, err);
                    continue;
                }
            };

            let map_name = lib.rsplit('/').next().unwrap_or(lib);
            elflib.load(
                &mut self.maps,
                map_name,
                true,
                true,
                constants::CFG_DEFAULT_BASE,
            );

            for (sym, addr) in elflib.exported_symbols() {
                export_map.entry(sym.clone()).or_insert(addr);
                elf64.addr_to_symbol.insert(addr, sym.clone());
                elf64.sym_to_addr.insert(sym, addr);
            }

            if real_libc {
                ifunc_resolvers.extend(elflib.ifunc_resolver_addrs());
                loaded_libs.push((map_name.to_string(), elflib));
            }
        }

        if export_map.is_empty() {
            return;
        }

        if self.cfg.arch.is_aarch64() {
            elf64.apply_rela_aarch64(&mut self.maps, &export_map);
            return;
        }

        // Real-libc mode: relocate the libraries themselves first (this is the
        // ld.so role), collecting their ifunc relocations to resolve later.
        let mut irelative: Vec<(u64, u64)> = Vec::new();
        if real_libc {
            for (name, lib) in &loaded_libs {
                let outcome = lib.apply_dynamic_relocations_full(
                    &mut self.maps,
                    &export_map,
                    &ifunc_resolvers,
                );
                if !outcome.unresolved.is_empty() {
                    log::trace!(
                        "elf64: {} unresolved imports: {:?}",
                        name,
                        outcome.unresolved
                    );
                }
                irelative.extend(outcome.irelative);
            }
        }

        // Relocate the main executable.
        let outcome =
            elf64.apply_dynamic_relocations_full(&mut self.maps, &export_map, &ifunc_resolvers);
        if !outcome.unresolved.is_empty() {
            log::warn!(
                "elf64: unresolved dynamic imports: {:?}",
                outcome.unresolved
            );
        }
        irelative.extend(outcome.irelative);

        // Resolve ifunc / IRELATIVE relocations by actually executing each
        // resolver inside the emulator and patching the slot with the result.
        if real_libc && !irelative.is_empty() {
            self.resolve_ifunc_relocations(irelative);
        }
    }

    /// Execute each ifunc resolver and patch its relocation slot with the
    /// returned implementation address. Resolvers are tiny (they read the CPU
    /// feature globals and pick an implementation), so running them in the
    /// emulator is cheap; results are cached per resolver address.
    fn resolve_ifunc_relocations(&mut self, irelative: Vec<(u64, u64)>) {
        let mut cache: HashMap<u64, u64> = HashMap::new();

        // Save emulator state so the load phase stays transparent.
        let saved_regs = *self.regs();
        let saved_pos = self.pos;

        for (patch_addr, resolver) in irelative {
            let impl_addr = if let Some(&a) = cache.get(&resolver) {
                a
            } else {
                let a = match self.linux_call64(resolver, &[0, 0]) {
                    Ok(ret) if ret != 0 => ret,
                    Ok(_) => {
                        log::debug!("ifunc resolver 0x{:x} returned null", resolver);
                        resolver
                    }
                    Err(err) => {
                        // Expected until ld.so global state (_rtld_global_ro /
                        // cpu_features) is provided — the resolvers read it.
                        log::debug!("ifunc resolver 0x{:x} failed: {}", resolver, err);
                        resolver
                    }
                };
                cache.insert(resolver, a);
                a
            };

            if let Some(map_name) = self.maps.get_addr_name(patch_addr).map(|s| s.to_string()) {
                self.maps
                    .get_mem_mut(&map_name)
                    .force_write_qword(patch_addr, impl_addr);
            }
        }

        *self.regs_mut() = saved_regs;
        self.pos = saved_pos;
    }

    /// Resolve a library/interpreter path to an existing host file.
    fn resolve_host_lib(&self, path: &str) -> Option<String> {
        if Path::new(path).exists() {
            return Some(path.to_string());
        }
        let base = path.rsplit('/').next().unwrap_or(path);
        for dir in [
            "/usr/lib/x86_64-linux-gnu",
            "/lib/x86_64-linux-gnu",
            "/usr/lib64",
            "/lib64",
            "/usr/lib",
            "/lib",
        ] {
            let cand = format!("{}/{}", dir, base);
            if Path::new(&cand).exists() {
                return Some(cand);
            }
        }
        None
    }

    /// Map the real program interpreter (ld.so) and set up the initial process
    /// stack with a full auxiliary vector, then start execution at ld.so's entry
    /// point — exactly like the Linux kernel does. ld.so then maps libc, applies
    /// every relocation, resolves ifuncs and sets up TLS by itself.
    fn setup_ld_so_bootstrap(&mut self, elf64: &mut Elf64, interp_path: &str) {
        let Some(host) = self.resolve_host_lib(interp_path) else {
            log::error!(
                "ld.so bootstrap: interpreter {} not found on host",
                interp_path
            );
            return;
        };

        let mut ld = match parse_elf64_file(&host) {
            Ok(l) => l,
            Err(err) => {
                log::error!("ld.so bootstrap: failed to parse {}: {}", host, err);
                return;
            }
        };
        ld.segment_mode = true;
        ld.load(
            &mut self.maps,
            "ld-linux",
            true,
            true,
            constants::CFG_DEFAULT_BASE,
        );

        let ld_base = ld.base;
        let ld_entry = ld.rebase_vaddr(ld.elf_hdr.e_entry);

        // Keep ld.so's symbol names for nicer disassembly/tracing.
        for (sym, addr) in ld.exported_symbols() {
            elf64.addr_to_symbol.entry(addr).or_insert(sym);
        }

        let prog_entry = elf64.base + elf64.elf_hdr.e_entry;
        let prog_phdr = elf64.base + elf64.elf_hdr.e_phoff;
        let phent = elf64.elf_hdr.e_phentsize;
        let phnum = elf64.elf_hdr.e_phnum;
        let execfn = self.filename.clone();

        self.write_linux_ld_stack(prog_entry, prog_phdr, phent, phnum, ld_base, &execfn);

        self.ld_bootstrap = true;
        self.set_pc(ld_entry);
        log::info!(
            "ld.so bootstrap: ld_base=0x{:x} ld_entry=0x{:x} prog_entry=0x{:x}",
            ld_base,
            ld_entry,
            prog_entry
        );
    }

    /// Build the System V initial process stack for the ld.so entry:
    /// `[argc][argv..][NULL][envp..][NULL][auxv..][AT_NULL]`, with strings and
    /// the AT_RANDOM bytes placed in a scratch area above it.
    fn write_linux_ld_stack(
        &mut self,
        prog_entry: u64,
        prog_phdr: u64,
        phent: u16,
        phnum: u16,
        ld_base: u64,
        execfn: &str,
    ) {
        let sp = self.regs().rsp;

        // --- scratch strings/data above the structured area ---
        let mut scratch = sp + 0x400;
        let mut put_str = |emu: &mut Emu, s: &str| -> u64 {
            let addr = scratch;
            for (i, b) in s.bytes().enumerate() {
                emu.maps.write_byte(addr + i as u64, b);
            }
            emu.maps.write_byte(addr + s.len() as u64, 0);
            scratch += s.len() as u64 + 1;
            addr
        };
        let execfn_ptr = put_str(self, execfn);
        // Extra argv from --args (e.g. `-i` to start bash interactively).
        let extra: Vec<String> = self
            .cfg
            .arguments
            .split_whitespace()
            .map(|s| s.trim_matches('"').to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let extra_argv: Vec<u64> = extra.iter().map(|a| put_str(self, a)).collect();
        let platform_ptr = put_str(self, "x86_64");
        // AT_RANDOM: 16 bytes used for the stack canary / pointer guard.
        let random_ptr = scratch;
        for i in 0..16u64 {
            self.maps.write_byte(random_ptr + i, (0x41 + i) as u8);
        }

        // --- structured area at rsp ---
        const AT_NULL: u64 = 0;
        const AT_PHDR: u64 = 3;
        const AT_PHENT: u64 = 4;
        const AT_PHNUM: u64 = 5;
        const AT_PAGESZ: u64 = 6;
        const AT_BASE: u64 = 7;
        const AT_FLAGS: u64 = 8;
        const AT_ENTRY: u64 = 9;
        const AT_UID: u64 = 11;
        const AT_EUID: u64 = 12;
        const AT_GID: u64 = 13;
        const AT_EGID: u64 = 14;
        const AT_HWCAP: u64 = 16;
        const AT_CLKTCK: u64 = 17;
        const AT_PLATFORM: u64 = 15;
        const AT_SECURE: u64 = 23;
        const AT_RANDOM: u64 = 25;
        const AT_EXECFN: u64 = 31;

        let auxv: &[(u64, u64)] = &[
            (AT_PHDR, prog_phdr),
            (AT_PHENT, phent as u64),
            (AT_PHNUM, phnum as u64),
            (AT_PAGESZ, 4096),
            (AT_BASE, ld_base),
            (AT_FLAGS, 0),
            (AT_ENTRY, prog_entry),
            (AT_UID, 1000),
            (AT_EUID, 1000),
            (AT_GID, 1000),
            (AT_EGID, 1000),
            (AT_HWCAP, 0),
            (AT_CLKTCK, 100),
            (AT_SECURE, 0),
            (AT_PLATFORM, platform_ptr),
            (AT_RANDOM, random_ptr),
            (AT_EXECFN, execfn_ptr),
            (AT_NULL, 0),
        ];

        let mut off = sp;
        let mut push = |emu: &mut Emu, v: u64| {
            emu.maps.write_qword(off, v);
            off += 8;
        };

        push(self, 1 + extra_argv.len() as u64); // argc
        push(self, execfn_ptr); // argv[0]
        for &a in &extra_argv {
            push(self, a); // argv[1..]
        }
        push(self, 0); // argv terminator
        push(self, 0); // envp terminator
        for &(atype, aval) in auxv {
            push(self, atype);
            push(self, aval);
        }
    }

    fn resolve_linux_stub_path(&self, lib_name: &str) -> Option<String> {
        let mut candidates = Vec::new();

        // Real-libc mode: prefer the genuine system libraries so the real
        // machine code (not a hand-crafted stub) gets mapped and executed.
        if self.cfg.linux_real_libc && !self.cfg.arch.is_aarch64() {
            for dir in [
                "/usr/lib/x86_64-linux-gnu",
                "/lib/x86_64-linux-gnu",
                "/usr/lib64",
                "/usr/lib",
                "/lib",
            ] {
                candidates.push(format!("{}/{}", dir, lib_name));
            }
        }

        if !self.cfg.maps_folder.is_empty() {
            candidates.push(self.cfg.get_maps_folder(lib_name));
        }

        if self.cfg.arch.is_aarch64() {
            candidates.push(format!("maps/linux/aarch64/{}", lib_name));
            candidates.push(format!("../../maps/linux/aarch64/{}", lib_name));
        } else {
            candidates.push(format!("maps/linux/x86_64/{}", lib_name));
            candidates.push(format!("../../maps/linux/x86_64/{}", lib_name));
        }

        candidates
            .into_iter()
            .find(|candidate| Path::new(candidate).exists())
    }
}
