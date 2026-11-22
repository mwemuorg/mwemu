//! The `ntdll!LdrInitializeThunk` bootstrap for 64-bit Windows processes.
//!
//! A real Windows process does not jump straight to its entry point: the
//! loader initializes the module lists, the PEB and the process heap first.
//! mwemu reproduces that by calling `ntdll!LdrInitializeThunk` with a CONTEXT
//! that points at the entry point, so ntdll runs its own init and then
//! continues there.

use goblin::pe::header::PE_POINTER_OFFSET;

use crate::context::context64::Context64;
use crate::emu::Emu;
use crate::err::MwemuError;
use crate::maps::mem64::Permission;
use crate::winapi::winapi64;

use crate::windows::structures::{LdrDataTableEntry64, Peb64, PebLdrData64, UnicodeString64};

// Mask used by the ntdll bootstrap context.
const BOOTSTRAP_CONTEXT_FLAGS: u32 = 0x0010_007f;

impl Emu {
    /// Run the loader bootstrap and leave the CPU at `entry_point`.
    ///
    /// `LdrInitializeThunk(PCONTEXT Context, PVOID NtdllBase, PVOID Unused)`
    /// needs ntdll's own image base so the loader can parse its PE headers
    /// during init.
    pub(crate) fn bootstrap_loader_init(&mut self, entry_point: u64) -> Result<(), MwemuError> {
        let ldr_init =
            winapi64::kernel32::resolve_api_name_in_module(self, "ntdll.dll", "LdrInitializeThunk");
        if ldr_init == 0 {
            if self.cfg.verbose >= 1 {
                log::trace!("ssdt: could not resolve ntdll!LdrInitializeThunk");
            }
            return Ok(());
        }

        // Arrange return to entrypoint so execution continues normally.
        self.regs_mut().rip = entry_point;
        log::trace!("Initializing win32 64bits emulating ntdll!LdrInitializeThunk");
        let ntdll_base = self.maps.get_mem("ntdll.pe").get_base();

        // Build a minimal x64 CONTEXT structure so LdrInitializeThunk does not
        // null-deref when it reads CONTEXT.Rip to find the process entry point.
        let Some(ctx_addr) = self.maps.lib64_alloc(Context64::GUEST_SIZE) else {
            log::error!("cannot alloc CONTEXT for LdrInitializeThunk");
            return Err(MwemuError::new(
                "cannot alloc CONTEXT for LdrInitializeThunk",
            ));
        };
        if self
            .maps
            .create_map(
                "ldr_context",
                ctx_addr,
                Context64::GUEST_SIZE,
                Permission::READ_WRITE,
            )
            .is_err()
        {
            log::error!("cannot create ldr_context map");
            return Err(MwemuError::new("cannot create ldr_context map"));
        }
        // ContextFlags
        let _ = self
            .maps
            .write_dword(ctx_addr + Context64::FLAGS_OFFSET, BOOTSTRAP_CONTEXT_FLAGS);
        // Rsp: current stack pointer
        let _ = self
            .maps
            .write_qword(ctx_addr + Context64::RSP_OFFSET, self.regs().rsp);
        // Rip: entry point — NtContinue will redirect execution here
        let _ = self
            .maps
            .write_qword(ctx_addr + Context64::RIP_OFFSET, entry_point);

        crate::windows::peb::peb64::ensure_ntdll_loader_globals(self);
        crate::syscall::windows::syscall64::memory::patch_ntdll_load_reason_checks(self);
        let _ = self.call64(ldr_init, &[ctx_addr, ntdll_base, 0]);
        self.check_api_stop()?;
        self.ldr_init_done = true;
        if self.process_terminated {
            log::trace!(
                "ntdll!LdrInitializeThunk DID NOT complete — bailed out mid-init (process_terminated set). pos={}",
                self.pos,
            );
        } else if self.regs().rip == entry_point {
            log::trace!(
                "ntdll!LdrInitializeThunk emulated completely. pos={} rip=ep=0x{:x}",
                self.pos,
                entry_point,
            );
        } else {
            log::trace!(
                "ntdll!LdrInitializeThunk returned but rip=0x{:x} (expected ep=0x{:x}). pos={}",
                self.regs().rip,
                entry_point,
                self.pos,
            );
        }

        self.repopulate_ldr_module_list();

        // Some ntdll versions allocate the EXE module's name buffer
        // during LdrInitializeThunk but never copy the path into it
        // under emulation, leaving FullDllName/BaseDllName pointing at
        // uninitialized (later freed) heap. Patch it with a stable
        // mwemu-owned buffer so PEB-walking code reads the real name.
        crate::windows::peb::peb64::fix_exe_module_name(self);

        self.dump_ldr_chain();
        Ok(())
    }

    /// Rebuild the PEB loader module list when ntdll left it empty.
    ///
    /// Some ntdll versions (notably newer Win10/Win11/Server2022) reset
    /// PEB_LDR_DATA during LdrInitializeThunk and rely on an internal
    /// RB-tree (LdrpModuleBaseAddressIndex) for lookups, leaving the
    /// legacy `In{Load,Memory,Initialization}OrderModuleList` linked
    /// lists empty. PEB-walking shellcode still walks the linked list,
    /// so we re-populate it here from our `.pe` maps in the order
    /// expected on real Windows: EXE, ntdll, kernel32, kernelbase, ...
    fn repopulate_ldr_module_list(&mut self) {
        let peb_base = self.maps.get_mem("peb").get_base();
        let ldr_addr = self
            .maps
            .read_qword(peb_base + Peb64::LDR_OFFSET)
            .unwrap_or(0);
        if ldr_addr == 0 {
            return;
        }
        let sentinel_mem = ldr_addr + PebLdrData64::IN_MEMORY_ORDER_MODULE_LIST_OFFSET;
        let first = self.maps.read_qword(sentinel_mem).unwrap_or(0);
        if first != 0 && first != sentinel_mem {
            return;
        }
        log::trace!("LDR InMemoryOrder list empty post-LdrInit — repopulating");
        let exe_name = self.cfg.exe_name.clone();
        let exe_base = self.base;
        // Canonical Win10+ early-list order
        let preferred: Vec<(String, u64)> = vec![
            (exe_name.clone(), exe_base),
            (
                "ntdll.dll".into(),
                self.maps
                    .get_map_by_name("ntdll.pe")
                    .map(|m| m.get_base())
                    .unwrap_or(0),
            ),
            (
                "kernel32.dll".into(),
                self.maps
                    .get_map_by_name("kernel32.pe")
                    .map(|m| m.get_base())
                    .unwrap_or(0),
            ),
            (
                "kernelbase.dll".into(),
                self.maps
                    .get_map_by_name("kernelbase.pe")
                    .map(|m| m.get_base())
                    .unwrap_or(0),
            ),
        ];
        for (name, base) in preferred {
            if base == 0 {
                continue;
            }
            let pe_off = self
                .maps
                .read_dword(base + PE_POINTER_OFFSET as u64)
                .unwrap_or(0);
            crate::windows::peb::peb64::dynamic_link_module(base, pe_off, &name, self);
            log::trace!("  repopulated LDR entry {} base=0x{:x}", name, base);
        }
    }

    /// Dump the InMemoryOrder chain so we can verify a PEB-walking
    /// shellcode finds the expected DllBase at the expected index.
    fn dump_ldr_chain(&self) {
        let peb_base = self.maps.get_mem("peb").get_base();
        let ldr = self
            .maps
            .read_qword(peb_base + Peb64::LDR_OFFSET)
            .unwrap_or(0);
        log::trace!("DEBUG ldr_chain: PEB=0x{:x} Ldr=0x{:x}", peb_base, ldr);
        // Dump first 64 bytes of the PEB_LDR_DATA so we can see if
        // ntdll restructured the lists.
        let mut hex = String::new();
        for j in 0..64u64 {
            let b = self.maps.read_byte(ldr + j).unwrap_or(0);
            hex.push_str(&format!("{:02x} ", b));
        }
        log::trace!("DEBUG ldr_dump[0..64]: {}", hex);
        let sentinel = ldr + PebLdrData64::IN_MEMORY_ORDER_MODULE_LIST_OFFSET;
        let mut cur = self.maps.read_qword(sentinel).unwrap_or(0);
        log::trace!("DEBUG sentinel=0x{:x} first_flink=0x{:x}", sentinel, cur);
        let mut i = 0;
        while cur != 0 && cur != sentinel && i < 16 {
            // cur points to &entry.InMemoryOrderLinks (offset 0x10 in LDR_DATA_TABLE_ENTRY)
            let entry = cur.wrapping_sub(LdrDataTableEntry64::IN_MEMORY_ORDER_LINKS_OFFSET);
            let dll_base = self
                .maps
                .read_qword(entry + LdrDataTableEntry64::DLL_BASE_OFFSET)
                .unwrap_or(0);
            // BaseDllName UNICODE_STRING is at offset 0x58: Length(W), MaxLen(W), pad(D), Buffer(Q)
            let name_len = self
                .maps
                .read_word(entry + LdrDataTableEntry64::BASE_DLL_NAME_OFFSET)
                .unwrap_or(0) as u64;
            let name_buf = self
                .maps
                .read_qword(
                    entry
                        + LdrDataTableEntry64::BASE_DLL_NAME_OFFSET
                        + UnicodeString64::BUFFER_OFFSET,
                )
                .unwrap_or(0);
            let mut s = String::new();
            let mut j = 0u64;
            while j < name_len.min(128) {
                let w = self.maps.read_word(name_buf + j).unwrap_or(0);
                if w == 0 {
                    break;
                }
                s.push(char::from_u32(w as u32).unwrap_or('?'));
                j += 2;
            }
            log::trace!(
                "DEBUG ldr_chain[{}] entry=0x{:x} DllBase=0x{:x} name='{}'",
                i,
                entry,
                dll_base,
                s
            );
            cur = self.maps.read_qword(cur).unwrap_or(0);
            i += 1;
        }
    }
}
