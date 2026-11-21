use crate::{emu::Emu, threading::context::ArchThreadState};

impl Emu {
    pub fn tls32(&self) -> &Vec<u32> {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { tls32, .. } => tls32,
            _ => unreachable!("tls32() called on aarch64 emu"),
        }
    }

    pub fn tls32_mut(&mut self) -> &mut Vec<u32> {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { tls32, .. } => tls32,
            _ => unreachable!("tls32_mut() called on aarch64 emu"),
        }
    }

    pub fn tls64(&self) -> &Vec<u64> {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { tls64, .. } => tls64,
            _ => unreachable!("tls64() called on aarch64 emu"),
        }
    }

    pub fn tls64_mut(&mut self) -> &mut Vec<u64> {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { tls64, .. } => tls64,
            _ => unreachable!("tls64_mut() called on aarch64 emu"),
        }
    }

    /// Initialize TLS for a PE32 already mapped in emulator memory.
    /// Reads PE headers from `base`, finds TLS data directory, and sets up
    /// _tls_index, .tls_data copy, .tls_ptrs array, and TEB+0x2C.
    /// Returns true if TLS was initialized.
    pub fn init_tls_from_pe32_in_memory(&mut self, base: u64) -> bool {
        // MZ check
        let mz = match self.maps.read_word(base) {
            Some(v) => v,
            None => return false,
        };
        if mz != 0x5A4D {
            return false;
        }

        // e_lfanew
        let e_lfanew = match self.maps.read_dword(base + 0x3C) {
            Some(v) => v as u64,
            None => return false,
        };
        let pe_off = base + e_lfanew;

        // PE signature
        let pe_sig = match self.maps.read_dword(pe_off) {
            Some(v) => v,
            None => return false,
        };
        if pe_sig != 0x00004550 {
            return false;
        }

        // Optional header magic (PE32=0x10B, PE32+=0x20B)
        let opt_off = pe_off + 24;
        let magic = match self.maps.read_word(opt_off) {
            Some(v) => v,
            None => return false,
        };
        if magic != 0x10B {
            return false; // only PE32 here
        }

        // NumberOfRvaAndSizes at optional_header + 92
        let num_dirs = match self.maps.read_dword(opt_off + 92) {
            Some(v) => v,
            None => return false,
        };
        if num_dirs <= 9 {
            return false; // no TLS directory entry
        }

        // Data directories start at optional_header + 96
        // TLS is entry 9: offset = 96 + 9*8 = 168
        let tls_dd_off = opt_off + 96 + 9 * 8;
        let tls_rva = match self.maps.read_dword(tls_dd_off) {
            Some(v) => v,
            None => return false,
        };
        let tls_size = match self.maps.read_dword(tls_dd_off + 4) {
            Some(v) => v,
            None => return false,
        };
        if tls_rva == 0 || tls_size == 0 {
            return false;
        }

        let tls_va = base + tls_rva as u64;

        // Read TlsDirectory32 fields from memory
        let tls_data_start = match self.maps.read_dword(tls_va) {
            Some(v) => v as u64,
            None => return false,
        };
        let tls_data_end = match self.maps.read_dword(tls_va + 4) {
            Some(v) => v as u64,
            None => return false,
        };
        let tls_index_va = match self.maps.read_dword(tls_va + 8) {
            Some(v) => v as u64,
            None => return false,
        };
        let zero_fill_size = match self.maps.read_dword(tls_va + 16) {
            Some(v) => v as u64,
            None => return false,
        };

        // Write _tls_index = 0
        self.maps.write_dword(tls_index_va, 0);
        log::info!("TLS-reinit: wrote _tls_index=0 at 0x{:x}", tls_index_va);

        let template_sz = tls_data_end.saturating_sub(tls_data_start) as usize;
        let total_sz = template_sz + zero_fill_size as usize;

        if total_sz > 0
            && let Some(tls_data_addr) = self.maps.alloc(total_sz.max(0x1000) as u64)
        {
            let _ = self.maps.create_map(
                &format!(".tls_data_{:x}", base),
                tls_data_addr,
                total_sz.max(0x1000) as u64,
                crate::maps::mem64::Permission::READ_WRITE,
            );
            if template_sz > 0 {
                self.maps.memcpy(tls_data_addr, tls_data_start, template_sz);
            }
            if let Some(tls_ptrs_addr) = self.maps.alloc(0x1000) {
                let _ = self.maps.create_map(
                    &format!(".tls_ptrs_{:x}", base),
                    tls_ptrs_addr,
                    0x1000,
                    crate::maps::mem64::Permission::READ_WRITE,
                );
                self.maps.write_dword(tls_ptrs_addr, tls_data_addr as u32);
                if self.maps.exists_mapname("teb") {
                    let teb_base = self.maps.get_mem("teb").get_base();
                    self.maps.write_dword(teb_base + 0x2C, tls_ptrs_addr as u32);
                    log::info!(
                        "TLS-reinit: TEB+0x2C=0x{:x} -> [0]=0x{:x} ({}+{} bytes) for PE at 0x{:x}",
                        tls_ptrs_addr,
                        tls_data_addr,
                        template_sz,
                        zero_fill_size,
                        base
                    );
                }
            }
        }

        true
    }

    /// Initialize TLS for a PE64 already mapped in emulator memory.
    pub fn init_tls_from_pe64_in_memory(&mut self, base: u64) -> bool {
        let mz = match self.maps.read_word(base) {
            Some(v) => v,
            None => return false,
        };
        if mz != 0x5A4D {
            return false;
        }

        let e_lfanew = match self.maps.read_dword(base + 0x3C) {
            Some(v) => v as u64,
            None => return false,
        };
        let pe_off = base + e_lfanew;

        let pe_sig = match self.maps.read_dword(pe_off) {
            Some(v) => v,
            None => return false,
        };
        if pe_sig != 0x00004550 {
            return false;
        }

        let opt_off = pe_off + 24;
        let magic = match self.maps.read_word(opt_off) {
            Some(v) => v,
            None => return false,
        };
        if magic != 0x20B {
            return false; // only PE64 here
        }

        let num_dirs = match self.maps.read_dword(opt_off + 108) {
            Some(v) => v,
            None => return false,
        };
        if num_dirs <= 9 {
            return false;
        }

        // Data directories start at optional_header + 112 for PE64
        let tls_dd_off = opt_off + 112 + 9 * 8;
        let tls_rva = match self.maps.read_dword(tls_dd_off) {
            Some(v) => v,
            None => return false,
        };
        let tls_size = match self.maps.read_dword(tls_dd_off + 4) {
            Some(v) => v,
            None => return false,
        };
        if tls_rva == 0 || tls_size == 0 {
            return false;
        }

        let tls_va = base + tls_rva as u64;

        let tls_data_start = match self.maps.read_qword(tls_va) {
            Some(v) => v,
            None => return false,
        };
        let tls_data_end = match self.maps.read_qword(tls_va + 8) {
            Some(v) => v,
            None => return false,
        };
        let tls_index_va = match self.maps.read_qword(tls_va + 16) {
            Some(v) => v,
            None => return false,
        };
        let zero_fill_size = match self.maps.read_dword(tls_va + 32) {
            Some(v) => v as u64,
            None => return false,
        };

        self.maps.write_qword(tls_index_va, 0);
        log::info!("TLS-reinit64: wrote _tls_index=0 at 0x{:x}", tls_index_va);

        let template_sz = tls_data_end.saturating_sub(tls_data_start) as usize;
        let total_sz = template_sz + zero_fill_size as usize;

        if total_sz > 0
            && let Some(tls_data_addr) = self.maps.alloc(total_sz.max(0x1000) as u64)
        {
            let _ = self.maps.create_map(
                &format!(".tls_data_{:x}", base),
                tls_data_addr,
                total_sz.max(0x1000) as u64,
                crate::maps::mem64::Permission::READ_WRITE,
            );
            if template_sz > 0 {
                self.maps.memcpy(tls_data_addr, tls_data_start, template_sz);
            }
            if let Some(tls_ptrs_addr) = self.maps.alloc(0x1000) {
                let _ = self.maps.create_map(
                    &format!(".tls_ptrs_{:x}", base),
                    tls_ptrs_addr,
                    0x1000,
                    crate::maps::mem64::Permission::READ_WRITE,
                );
                self.maps.write_qword(tls_ptrs_addr, tls_data_addr);
                if self.maps.exists_mapname("teb64") {
                    let teb_base = self.maps.get_mem("teb64").get_base();
                    self.maps.write_qword(teb_base + 0x58, tls_ptrs_addr);
                    log::info!(
                        "TLS-reinit64: TEB+0x58=0x{:x} -> [0]=0x{:x} ({}+{} bytes) for PE at 0x{:x}",
                        tls_ptrs_addr,
                        tls_data_addr,
                        template_sz,
                        zero_fill_size,
                        base
                    );
                }
            }
        }

        true
    }
}
