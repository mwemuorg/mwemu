use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

use crate::emu::Emu;
use crate::loaders::macho::macho64::Macho64;

impl Emu {
    /// Load a 64-bit Mach-O binary.
    pub fn load_macho64(&mut self, filename: &str) {
        let mut macho = Macho64::parse(filename).expect("cannot parse macho64 binary");
        macho.load(&mut self.maps);
        if self.cfg.arch.is_aarch64() {
            self.init_macos_aarch64();
        } else if self.cfg.arch.is_x64() {
            self.init_macos64();
        } else {
            unimplemented!("unsupported Mach-O architecture: {:?}", self.cfg.arch);
        }
        self.set_pc(macho.entry);

        // Write macOS ABI stack layout (argc/argv/envp/apple[])
        if self.cfg.arch.is_aarch64() {
            self.write_macos_stack_layout();
        }

        // --- Dylib loading and GOT resolution ---

        // Stage 1+2: Recursively load dylibs, following reexports (LC_REEXPORT_DYLIB).
        // Goblin's .libs includes both LC_LOAD_DYLIB and LC_REEXPORT_DYLIB entries.
        let mut export_map: HashMap<String, u64> = HashMap::new();
        let mut loaded: HashSet<String> = HashSet::new();
        let mut worklist: VecDeque<String> = macho.get_libs().into_iter().collect();

        // The tracked stub libSystem.B.dylib does not carry LC_REEXPORT_DYLIB
        // entries for the real sub-libraries. Seed the worklist with the known
        // sub-libraries so their exports are available for GOT resolution.
        const LIBSYSTEM_SUBS: &[&str] = &[
            "libsystem_c.dylib",
            "libsystem_kernel.dylib",
            "libsystem_platform.dylib",
            "libsystem_pthread.dylib",
            "libsystem_malloc.dylib",
            "libsystem_info.dylib",
            "libsystem_darwin.dylib",
            "libsystem_blocks.dylib",
            "libsystem_trace.dylib",
            "libsystem_notify.dylib",
            "libsystem_networkextension.dylib",
            "libsystem_asl.dylib",
            "libsystem_sandbox.dylib",
            "libsystem_secinit.dylib",
            "libsystem_containermanager.dylib",
            "libsystem_configuration.dylib",
            "libsystem_coreservices.dylib",
            "libsystem_collections.dylib",
            "libsystem_featureflags.dylib",
            "libdispatch.dylib",
            "libxpc.dylib",
            "libcorecrypto.dylib",
            "libcommonCrypto.dylib",
            "liblaunch.dylib",
            "libcompiler_rt.dylib",
            "libcache.dylib",
            "libmacho.dylib",
            "libremovefile.dylib",
            "libcopyfile.dylib",
            "libunwind.dylib",
            "libdyld.dylib",
            "libkeymgr.dylib",
            "libquarantine.dylib",
            "libclosured.dylib",
        ];
        for sub in LIBSYSTEM_SUBS {
            worklist.push_back(sub.to_string());
        }

        while let Some(lib_path) = worklist.pop_front() {
            let lib_name = lib_path.rsplit('/').next().unwrap_or(&lib_path).to_string();
            if loaded.contains(&lib_name) {
                continue;
            }
            loaded.insert(lib_name.clone());

            let local_path = self.cfg.get_maps_folder(&lib_name);
            if !Path::new(&local_path).exists() {
                log::warn!(
                    "macho64: dylib not found: {} (looked at {})",
                    lib_name,
                    local_path
                );
                continue;
            }

            let (_base, exports) = self.map_dylib_macho64(&local_path, &lib_name);
            for (sym, addr) in exports {
                macho.addr_to_symbol.insert(addr, sym.clone());
                export_map.insert(sym, addr);
            }

            // Follow reexports: parse this dylib's own dependencies and enqueue them.
            if let Ok(sub_dylib) = Macho64::parse(&local_path) {
                for sub_lib in sub_dylib.get_libs() {
                    let sub_name = sub_lib.rsplit('/').next().unwrap_or(&sub_lib).to_string();
                    if !loaded.contains(&sub_name) {
                        worklist.push_back(sub_lib);
                    }
                }
            }
        }

        // Stage 2.5: Inject macOS global data symbols into the export map.
        // These are data symbols (__stdoutp, __stderrp, etc.) that must point
        // to valid memory so the binary can dereference them.
        if self.cfg.arch.is_aarch64() {
            self.inject_macos_globals(&mut export_map, &mut macho.addr_to_symbol);
        }

        // Stage 3: Parse chained fixups and resolve GOT entries
        let (imports, binds) = macho.parse_chained_fixups();
        log::trace!(
            "macho64: chained fixups: {} imports, {} binds",
            imports.len(),
            binds.len()
        );
        for (i, imp) in imports.iter().enumerate() {
            log::trace!(
                "  import[{}]: {} (lib_ordinal={})",
                i,
                imp.name,
                imp.lib_ordinal
            );
        }
        for b in &binds {
            log::trace!(
                "  bind: GOT 0x{:x} -> import[{}]",
                b.got_vmaddr,
                b.import_ordinal
            );
        }
        for bind in &binds {
            if let Some(imp) = imports.get(bind.import_ordinal as usize) {
                if let Some(&resolved_addr) = export_map.get(&imp.name) {
                    log::trace!(
                        "macho64: resolved {} -> 0x{:x} (GOT 0x{:x})",
                        imp.name,
                        resolved_addr,
                        bind.got_vmaddr
                    );
                    self.maps.write_qword(bind.got_vmaddr, resolved_addr);
                } else {
                    log::warn!(
                        "macho64: unresolved import {} (GOT at 0x{:x})",
                        imp.name,
                        bind.got_vmaddr
                    );
                }
            }
        }

        self.macho64 = Some(macho);
    }

    /// Allocate global data symbols required by macOS binaries and inject them
    /// into the export map so GOT resolution picks them up.
    fn inject_macos_globals(
        &mut self,
        export_map: &mut HashMap<String, u64>,
        addr_to_sym: &mut HashMap<u64, String>,
    ) {
        use crate::maps::mem64::Permission;

        // Allocate a single "macos_globals" region for small data symbols.
        let globals_size: u64 = 0x8000;
        let globals_base = self
            .maps
            .alloc(globals_size)
            .expect("cannot alloc macos globals");
        let mem = self
            .maps
            .create_map(
                "macos_globals",
                globals_base,
                globals_size,
                Permission::READ_WRITE,
            )
            .expect("cannot create macos_globals map");
        // Zero-fill
        for i in 0..globals_size {
            mem.write_byte(globals_base + i, 0);
        }

        // Remove any dylib exports that we are about to override with our
        // own globals, otherwise addr_to_symbol will have duplicates and
        // lookups by name can return the dylib's stale address.
        let overridden: &[&str] = &[
            "___stdoutp",
            "___stderrp",
            "___stack_chk_guard",
            "_optarg",
            "_optind",
            "_errno",
            "__DefaultRuneLocale",
        ];
        addr_to_sym.retain(|_, name| !overridden.contains(&name.as_str()));
        export_map.retain(|name, _| !overridden.contains(&name.as_str()));

        let mut off: u64 = 0;

        // Helper: carve out `size` bytes from the globals block.
        let mut carve = |maps: &mut crate::maps::Maps, size: u64| -> u64 {
            let addr = globals_base + off;
            off += (size + 7) & !7; // align to 8
            let _ = maps; // suppress unused warning
            addr
        };

        // __stack_chk_guard (8 bytes) — non-zero canary
        let chk_guard = carve(&mut self.maps, 8);
        self.maps.write_qword(chk_guard, 0xDEAD_BEEF_CAFE_BABE);
        export_map.insert("___stack_chk_guard".to_string(), chk_guard);
        addr_to_sym.insert(chk_guard, "___stack_chk_guard".to_string());

        // FILE structs (macOS __sFILE, 152 bytes) with real buffers for putc
        // Layout: _p(+0,8) _r(+8,4) _w(+12,4) _flags(+16,2) _file(+18,2)
        //         _bf._base(+24,8) _bf._size(+32,4) _lbfsize(+40,4)
        let buf_size: u32 = 8192;
        let stdout_buf = carve(&mut self.maps, buf_size as u64);
        let stdout_file = carve(&mut self.maps, 256);
        self.maps.write_qword(stdout_file, stdout_buf); // _p = buf start
        self.maps.write_dword(stdout_file + 12, buf_size); // _w = buf_size
        self.maps.write_word(stdout_file + 16, 0x0008); // _flags = __SWR
        self.maps.write_word(stdout_file + 18, 1); // _file = fd 1
        self.maps.write_qword(stdout_file + 24, stdout_buf); // _bf._base
        self.maps.write_dword(stdout_file + 32, buf_size); // _bf._size

        let stderr_buf = carve(&mut self.maps, buf_size as u64);
        let stderr_file = carve(&mut self.maps, 256);
        self.maps.write_qword(stderr_file, stderr_buf); // _p = buf start
        self.maps.write_dword(stderr_file + 12, buf_size); // _w = buf_size
        self.maps.write_word(stderr_file + 16, 0x0008); // _flags = __SWR
        self.maps.write_word(stderr_file + 18, 2); // _file = fd 2
        self.maps.write_qword(stderr_file + 24, stderr_buf); // _bf._base
        self.maps.write_dword(stderr_file + 32, buf_size); // _bf._size

        // __stdoutp: pointer to stdout FILE
        let stdoutp = carve(&mut self.maps, 8);
        self.maps.write_qword(stdoutp, stdout_file);
        export_map.insert("___stdoutp".to_string(), stdoutp);
        addr_to_sym.insert(stdoutp, "___stdoutp".to_string());

        // __stderrp: pointer to stderr FILE
        let stderrp = carve(&mut self.maps, 8);
        self.maps.write_qword(stderrp, stderr_file);
        export_map.insert("___stderrp".to_string(), stderrp);
        addr_to_sym.insert(stderrp, "___stderrp".to_string());

        // optarg (8 bytes, NULL pointer)
        let optarg = carve(&mut self.maps, 8);
        export_map.insert("_optarg".to_string(), optarg);
        addr_to_sym.insert(optarg, "_optarg".to_string());

        // optind (4 bytes, value 1)
        let optind = carve(&mut self.maps, 4);
        self.maps.write_dword(optind, 1);
        export_map.insert("_optind".to_string(), optind);
        addr_to_sym.insert(optind, "_optind".to_string());

        // errno storage (4 bytes)
        let errno_addr = carve(&mut self.maps, 4);
        export_map.insert("_errno".to_string(), errno_addr);
        addr_to_sym.insert(errno_addr, "_errno".to_string());

        // _DefaultRuneLocale (256 bytes, zero-filled — enough for __maskrune)
        let rune_locale = carve(&mut self.maps, 256);
        export_map.insert("__DefaultRuneLocale".to_string(), rune_locale);
        addr_to_sym.insert(rune_locale, "__DefaultRuneLocale".to_string());

        log::info!(
            "macho64: injected macOS globals at 0x{:x}-0x{:x}",
            globals_base,
            globals_base + off
        );
    }

    /// Load a Mach-O dylib from disk and map its segments into memory.
    /// Returns (base_address, vec of (symbol_name, absolute_address)).
    pub fn map_dylib_macho64(
        &mut self,
        filename: &str,
        lib_name: &str,
    ) -> (u64, Vec<(String, u64)>) {
        use crate::loaders::macho::macho64::prot_to_permission;

        let dylib = Macho64::parse(filename).expect("cannot parse dylib");

        // Calculate total size needed
        let total_size: u64 = dylib.segments.iter().map(|s| s.vmsize).sum();

        // Allocate in library address range
        let base = self
            .maps
            .lib64_alloc(total_size.max(0x4000))
            .expect("cannot allocate space for dylib");

        // Strip extension for map naming: "libSystem.B.dylib" -> "libSystem.B"
        let base_name = lib_name.strip_suffix(".dylib").unwrap_or(lib_name);

        // Cache-extracted dylibs have absolute vmaddrs; subtract the minimum
        // segment vmaddr so offsets become zero-based before adding our base.
        let vmaddr_base = dylib
            .segments
            .iter()
            .filter(|s| s.vmsize > 0)
            .map(|s| s.vmaddr)
            .min()
            .unwrap_or(0);

        // Map each segment
        for seg in &dylib.segments {
            if seg.vmsize == 0 {
                continue;
            }

            let perm = prot_to_permission(seg.initprot);
            let seg_addr = base + (seg.vmaddr - vmaddr_base);
            let map_name = format!("{}.{}", base_name, seg.name);

            let mem = match self.maps.create_map(&map_name, seg_addr, seg.vmsize, perm) {
                Ok(m) => m,
                Err(_) => {
                    log::warn!(
                        "cannot create map for dylib segment '{}', skipping",
                        map_name
                    );
                    continue;
                }
            };

            if !seg.data.is_empty() {
                mem.force_write_bytes(seg_addr, &seg.data);
            }
        }

        // Get exports and rebase addresses
        let exports: Vec<(String, u64)> = dylib
            .get_exports()
            .into_iter()
            .map(|(name, offset)| (name, base + offset))
            .collect();

        (base, exports)
    }
}
