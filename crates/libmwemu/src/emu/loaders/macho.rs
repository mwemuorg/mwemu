use std::collections::HashMap;
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
            panic!("unsupported Mach-O architecture: {:?}", self.cfg.arch);
        }
        self.set_pc(macho.entry);

        // --- Dylib loading and GOT resolution ---

        // Stage 1: Discover dependent dylibs
        let libs = macho.get_libs();

        // Stage 2: Load each dylib from maps/macos/{arch}/
        let mut export_map: HashMap<String, u64> = HashMap::new();
        for lib_path in &libs {
            // Extract filename from path: "/usr/lib/libSystem.B.dylib" -> "libSystem.B.dylib"
            let lib_name = lib_path.rsplit('/').next().unwrap_or(lib_path);
            let local_path = self.cfg.get_maps_folder(lib_name);
            if Path::new(&local_path).exists() {
                let (_base, exports) = self.map_dylib_macho64(&local_path, lib_name);
                for (sym, addr) in exports {
                    macho.addr_to_symbol.insert(addr, sym.clone());
                    export_map.insert(sym, addr);
                }
            } else {
                log::warn!(
                    "macho64: dylib not found: {} (looked at {})",
                    lib_name,
                    local_path
                );
            }
        }

        // Stage 3: Parse chained fixups and resolve GOT entries
        let (imports, binds) = macho.parse_chained_fixups();
        for bind in &binds {
            if let Some(imp) = imports.get(bind.import_ordinal as usize) {
                if let Some(&resolved_addr) = export_map.get(&imp.name) {
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

        // Map each segment
        for seg in &dylib.segments {
            if seg.vmsize == 0 {
                continue;
            }

            let perm = prot_to_permission(seg.initprot);
            let seg_addr = base + seg.vmaddr; // Rebase: dylib vmaddr is relative to 0
            let map_name = format!("{}.{}", base_name, seg.name);

            let mem = self
                .maps
                .create_map(&map_name, seg_addr, seg.vmsize, perm)
                .unwrap_or_else(|_| panic!("cannot create map for dylib segment '{}'",
                    map_name));

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
