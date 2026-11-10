//! Setting an emulator up as a Windows process: the stack, the PEB and TEB,
//! the loader data and the system DLLs.

use std::collections::BTreeSet;
use std::io::IsTerminal;

use crate::emu::object_handle::HeapHandle;
use crate::emu::{Emu, InstructionState};
use crate::err::MwemuError;
use crate::maps::heap_allocation::O1Heap;
use crate::maps::mem64::Permission;
use crate::threading::context::ThreadContext;
use crate::windows::peb::{peb32, peb64};
use crate::windows::{kuser_shared, loader32, loader64, structures};

/// Initialising an emulator as a Windows process.
pub(crate) trait WindowsInit {
    /// Initialize windows simulator, this does like init_cpu() but also setup the windows memory.
    /// This require having the map files in place, otherwise use just init_cpu() but emu32() and
    /// emu64() already call init_cpu()
    /// This is called from load_code if the code is a PE or shellcode.
    /// load_code_bytes() and other loading ways don't call this, if you need windows simulation call this.
    fn init_win32(&mut self, clear_registers: bool, clear_flags: bool) -> Result<(), MwemuError>;

    /// This is called from init(), this setup the 32bits windows memory simulation.
    fn init_win32_mem32(&mut self);

    /// Initialize Windows ARM64 session: sets up aarch64 stack and full PEB64/TEB64/LDR64
    /// memory layout. Arch state and thread context must already be switched to AArch64
    /// before calling this (done at the top of init_win32).
    ///
    /// Reuses the same 64-bit Windows memory setup as x86_64 since PEB64/TEB64/LDR
    /// structures are architecture-neutral; the difference is register semantics and stack init.
    fn init_win_aarch64(&mut self) -> Result<(), MwemuError>;

    /// This is called from init(), this setup the 64bits windows memory simulation.
    fn init_win32_mem64(&mut self) -> Result<(), MwemuError>;
}

impl WindowsInit for Emu {
    /// Initialize windows simulator, this does like init_cpu() but also setup the windows memory.
    /// This require having the map files in place, otherwise use just init_cpu() but emu32() and
    /// emu64() already call init_cpu()
    /// This is called from load_code if the code is a PE or shellcode.
    /// load_code_bytes() and other loading ways don't call this, if you need windows simulation call this.
    fn init_win32(&mut self, clear_registers: bool, clear_flags: bool) -> Result<(), MwemuError> {
        self.pos = 0;

        if !std::io::stdout().is_terminal() {
            self.cfg.nocolors = true;
            self.colors.disable();
            // Auto-disable the interactive console only when the user did NOT
            // explicitly ask for it (`--handle` sets `enabled_ctrlc`, `-c`/`-C`
            // set `console_enabled`). An explicit request must work even with
            // redirected/piped stdout.
            if !self.enabled_ctrlc && !self.cfg.console_enabled {
                self.cfg.console_enabled = false;
                self.disable_ctrlc();
            }
        }

        // Ensure arch_state and thread context match the target architecture before
        // touching any registers, since regs_mut()/regs_aarch64_mut() panic on mismatch.
        if self.cfg.is_aarch64() {
            self.instruction_state = InstructionState::default();
            if matches!(
                self.threads[self.current_thread_id].arch,
                crate::threading::context::ArchThreadState::X86 { .. }
            ) {
                let id = self.threads[self.current_thread_id].id;
                self.threads[self.current_thread_id] = ThreadContext::new(id, self.cfg.arch);
            }
        }

        //log::trace!("initializing regs");
        if self.cfg.is_aarch64() {
            // AArch64: zero all registers; no x86 flags to clear.
            if clear_registers {
                *self.regs_aarch64_mut() = crate::arch::aarch64::regs::RegsAarch64::new();
            }
            self.regs_aarch64_mut().pc = self.cfg.entry_point;
        } else {
            if clear_registers {
                self.regs_mut().clear::<64>();
            }
            if clear_flags {
                self.flags_mut().clear();
            }
            self.regs_mut().rip = self.cfg.entry_point;
        }
        if self.cfg.arch.is_64bits() {
            self.maps.is_64bits = true;
            if self.cfg.is_aarch64() {
                self.init_win_aarch64()?;
            } else {
                self.init_win32_mem64()?;
                self.init_stack64();
            }
        } else {
            // 32bits
            self.maps.is_64bits = false;
            self.regs_mut().sanitize32();
            self.init_win32_mem32();
            self.init_stack32();
        }

        // Load banzai metadata only when 32-bit skip-unimplemented mode needs it.
        if !self.cfg.arch.is_64bits() && self.cfg.skip_unimplemented {
            self.banzai.load(&self.cfg.get_maps_folder("banzai.csv"));
        }

        Ok(())
    }

    /// This is called from init(), this setup the 32bits windows memory simulation.
    fn init_win32_mem32(&mut self) {
        log::trace!("loading memory maps");

        self.maps.is_64bits = self.cfg.arch.is_64bits();

        // (Historic `set_current_dir(maps_folder)` removed — the block in
        // between had no file-system calls and the chdir raced with parallel
        // tests that needed the process CWD to point at the repo root.)

        peb32::init_peb(self);
        kuser_shared::init_kuser_shared_data(self);
        loader32::load_library(self, "ntdll.dll");
        let ntdll_base = self
            .maps
            .get_map_by_name("ntdll.pe")
            .map(|m| m.get_base())
            .unwrap_or(0);
        if ntdll_base != 0 {
            peb32::update_peb_image_base(self, ntdll_base as u32);
        }

        loader32::load_library(self, "kernel32.dll");
        loader32::load_library(self, "kernelbase.dll");
        loader32::load_library(self, "iphlpapi.dll");
        loader32::load_library(self, "ws2_32.dll");
        loader32::load_library(self, "advapi32.dll");
        //loader32::load_library(self, "comctl32.dll");
        loader32::load_library(self, "winhttp.dll");
        loader32::load_library(self, "wininet.dll");
        //loader32::load_library(self, "dnsapi.dll");
        loader32::load_library(self, "shell32.dll");
        //loader32::load_library(self, "shlwapi.dll");

        let teb_map = self.maps.get_mem_mut("teb");
        let mut teb = structures::TEB::load_map(teb_map.get_base(), teb_map);
        teb.nt_tib.stack_base = self.cfg.stack_addr as u32;
        teb.nt_tib.stack_limit = (self.cfg.stack_addr + 0x30000) as u32;
        teb.save(teb_map);
    }

    /// Initialize Windows ARM64 session: sets up aarch64 stack and full PEB64/TEB64/LDR64
    /// memory layout. Arch state and thread context must already be switched to AArch64
    /// before calling this (done at the top of init_win32).
    ///
    /// Reuses the same 64-bit Windows memory setup as x86_64 since PEB64/TEB64/LDR
    /// structures are architecture-neutral; the difference is register semantics and stack init.
    fn init_win_aarch64(&mut self) -> Result<(), MwemuError> {
        // Set up the aarch64 stack first (uses SP register instead of RSP/RBP)
        // so that the stack map exists when init_win32_mem64 writes TEB stack bounds.
        self.init_stack_aarch64();

        // Set up the 64-bit Windows memory (PEB64, TEB64, LDR, DLLs, heap)
        self.init_win32_mem64()
    }

    /// This is called from init(), this setup the 64bits windows memory simulation.
    fn init_win32_mem64(&mut self) -> Result<(), MwemuError> {
        log::trace!("loading memory maps");
        self.maps.is_64bits = self.cfg.arch.is_64bits();

        // In SSDT mode we can optionally let `ntdll!LdrInitializeThunk` bootstrap the loader.
        // This path intentionally maps far fewer DLLs up front.
        if self.cfg.emulate_winapi {
            // Make sure cfg.exe_name reflects the real image basename BEFORE
            // PEB/LDR init writes it into the canonical structures.
            peb64::refresh_exe_name_from_filename(self);
            // Empty PEB + TEB only; `ntdll!LdrInitializeThunk` is expected to initialize loader state.
            peb64::init_peb_teb_empty(self);
            kuser_shared::init_kuser_shared_data(self);

            // ntdll must be pre-mapped so we can resolve and call
            // `LdrInitializeThunk`.
            loader64::load_library(self, "ntdll.dll");

            // Build the syscall number translation table from this ntdll's
            // actual syscall stubs. Microsoft renumbers syscalls between
            // builds (Win10 vs Win11/Server2022 are notably different), and
            // our dispatcher matches on hard-coded constants tied to one
            // build. Without this step a Win2022 ntdll calling syscall 0x16d
            // (NtRaiseException) lands in our 0x16d handler (was the older
            // NtQuerySystemEnvironmentValueEx) and silently misroutes —
            // RtlRaiseException then returns "0" instead of dispatching the
            // exception, leading to a RtlRaiseStatus(0) recursion.
            crate::layers::windows().build_syscall_translation_table(self);

            // Pre-map kernel32 and kernelbase even though modern ntdll
            // clears the legacy `InMemoryOrderModuleList` during init —
            // we don't pre-load them for ordering, but so their `.pe`
            // maps exist when we re-populate the linked list afterwards
            // (LdrInit on Win10+ relies on syscalls we don't fully model,
            // and bails before bringing these in via the KnownDll path).
            //loader64::load_library(self, "kernel32.dll");
            //loader64::load_library(self, "kernelbase.dll");

            // ntdll patches its own globals during LdrInitializeThunk (LdrpHashTable,
            // LdrpModuleBaseAddressIndex, etc.).  These often fall in the .rdata section
            // which our PE loader maps read-only; add write permission to all ntdll maps
            // so the self-patching writes succeed.
            let ntdll_map_names: Vec<String> = self
                .maps
                .name_map
                .keys()
                .filter(|n| n.starts_with("ntdll"))
                .cloned()
                .collect();
            for name in ntdll_map_names {
                if let Some(mem) = self.maps.get_map_by_name_mut(&name) {
                    mem.add_permission(Permission::WRITE);
                }
            }

            // Patch ntdll's LdrpHandleInvalidUserCallTarget / RtlFailFast2 terminal block.
            // The sequence `mov edx, 0xc0000409; mov rcx, -1; call NtTerminateProcess` triggers
            // during LdrInitializeThunk for CFG violations in our emulated environment.
            // Replace the `mov edx` with a `jmp` to the `ret` that follows the call, so the
            // handler returns harmlessly instead of terminating the process.
            // Pattern: ba 09 04 00 c0  48 c7 c1 ff ff ff ff  e8 ?? ?? ?? ??  [??x7]  c3
            // (5 bytes mov edx) + (7 bytes mov rcx,-1) + (5 bytes call) + (7 bytes add rsp) + ret
            if let Some(ntdll_pe) = self.maps.get_map_by_name("ntdll.pe") {
                let ntdll_base = ntdll_pe.get_base();
                // ntdll.pe is just the PE header — scan the full image (≤ 8 MiB covers all sections).
                let ntdll_size: usize = 0x800000;
                // Search for the pattern in the ntdll image.
                // We look for mov-edx + mov-rcx(-1) = ba 09 04 00 c0  48 c7 c1 ff ff ff ff
                let needle: &[u8] = &[
                    0xba, 0x09, 0x04, 0x00, 0xc0, 0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff,
                ];
                let mut found_va: Option<u64> = None;
                for off in 0..ntdll_size.saturating_sub(needle.len() + 10) {
                    let va = ntdll_base + off as u64;
                    let mut matches = true;
                    for (i, &b) in needle.iter().enumerate() {
                        if self.maps.read_byte(va + i as u64).unwrap_or(0xff) != b {
                            matches = false;
                            break;
                        }
                    }
                    if matches {
                        // Verify there is a `ret` (c3) 24 bytes after pattern start:
                        // needle(12) + call(5) + add rsp,0x88(7) = 24.
                        let ret_off = needle.len() as u64 + 5 + 7; // = 24
                        if self.maps.read_byte(va + ret_off).unwrap_or(0) == 0xc3 {
                            found_va = Some(va);
                            break;
                        }
                    }
                }
                if let Some(patch_va) = found_va {
                    // Layout from patch_va:
                    //  +0x00 mov edx,0xc0000409  (5 bytes)  ← replaced with jmp rel32
                    //  +0x05 mov rcx,-1           (7 bytes)
                    //  +0x0c call NtTerminateProcess (5 bytes)
                    //  +0x11 add rsp,0x88          (7 bytes)  ← jump TARGET (must not skip this)
                    //  +0x18 ret
                    // displacement = 0x11 - 5 = 0x0C
                    let _ = self.maps.write_byte(patch_va, 0xe9); // jmp rel32
                    let _ = self.maps.write_byte(patch_va + 1, 0x0c); // disp: jump to add rsp,0x88
                    let _ = self.maps.write_byte(patch_va + 2, 0x00);
                    let _ = self.maps.write_byte(patch_va + 3, 0x00);
                    let _ = self.maps.write_byte(patch_va + 4, 0x00);
                    log::debug!("ntdll CFG RtlFailFast2 patch applied at 0x{:x}", patch_va);
                } else {
                    log::debug!("ntdll CFG RtlFailFast2 pattern not found — skipping patch");
                }

                // LdrpInitSecurityCookie double-init patch
                // ---------------------------------------------------------------
                // ntdll's per-DLL security-cookie randomiser does:
                //   mov r12, 0x2b992ddfa232  ; uninitialised "magic"
                //   cmp [rdi], r12
                //   jne <fail>               ; not magic → STATUS_INVALID_IMAGE_FORMAT
                //   ...randomise rbx...
                //   mov [rdi], rbx           ; store new cookie
                //   mov eax, 1; ret
                //
                // The loader on this ntdll build invokes the routine twice for
                // kernelbase during LdrInitializeThunk; the second call finds
                // the cookie already randomised and trips the `jne` → process
                // aborts with `STATUS_INVALID_IMAGE_FORMAT (0xC000007B)`,
                // killing every subsequent `LoadLibraryA`.
                //
                // Patch the cookie-store `mov [rdi], rbx` (48 89 1f) to three
                // NOPs.  Effect: the cookie keeps its on-disk "magic" forever.
                // First and every later call see magic, pass the `cmp`, and
                // return success without mutating the page. ntdll continues
                // unaware; stack-canary checks all run against the same magic
                // value, which is fine for emulation.
                if let Some(ntdll_pe) = self.maps.get_map_by_name("ntdll.pe") {
                    let ntdll_base = ntdll_pe.get_base();
                    let ntdll_size: usize = 0x800000;
                    // Pattern: movabs r12,0x2b992ddfa232 (49 bc 32 a2 df 2d 99 2b 00 00)
                    //          cmp [rdi], r12          (4c 39 27)
                    //          jne short ??            (75 ??)
                    //          mov rcx, rdi            (48 8b cf)
                    //          call ??                 (e8 ?? ?? ?? ??)
                    // followed downstream by the `mov [rdi], rbx` (48 89 1f).
                    let needle: &[u8] = &[
                        0x49, 0xbc, 0x32, 0xa2, 0xdf, 0x2d, 0x99, 0x2b, 0x00, 0x00, 0x4c, 0x39,
                        0x27,
                    ];
                    let mut anchor_va: Option<u64> = None;
                    for off in 0..ntdll_size.saturating_sub(needle.len() + 0x80) {
                        let va = ntdll_base + off as u64;
                        let mut matches = true;
                        for (i, &b) in needle.iter().enumerate() {
                            if self.maps.read_byte(va + i as u64).unwrap_or(0xff) != b {
                                matches = false;
                                break;
                            }
                        }
                        if matches {
                            anchor_va = Some(va);
                            break;
                        }
                    }
                    if let Some(anchor) = anchor_va {
                        // Walk forward looking for `48 89 1f` (mov [rdi], rbx)
                        // within ~0x80 bytes of the cmp.
                        let mut store_va: Option<u64> = None;
                        for d in 0..0x80u64 {
                            let p = anchor + needle.len() as u64 + d;
                            if self.maps.read_byte(p).unwrap_or(0) == 0x48
                                && self.maps.read_byte(p + 1).unwrap_or(0) == 0x89
                                && self.maps.read_byte(p + 2).unwrap_or(0) == 0x1f
                            {
                                store_va = Some(p);
                                break;
                            }
                        }
                        if let Some(p) = store_va {
                            let _ = self.maps.write_byte(p, 0x90);
                            let _ = self.maps.write_byte(p + 1, 0x90);
                            let _ = self.maps.write_byte(p + 2, 0x90);
                            log::debug!(
                                "ntdll LdrpInitSecurityCookie store-cookie nop patch at 0x{:x}",
                                p
                            );
                        } else {
                            log::debug!(
                                "ntdll LdrpInitSecurityCookie cookie-store not found near 0x{:x}",
                                anchor
                            );
                        }
                    } else {
                        log::debug!(
                            "ntdll LdrpInitSecurityCookie cmp pattern not found — skipping patch"
                        );
                    }
                }

                // LdrInitializeThunk error-status bypass patch
                // ---------------------------------------------------------------
                // After `LdrpInitializeProcess` returns, LdrInitializeThunk
                // checks the status with:
                //   41 bc 00 20 00 00   mov  r12d, 0x2000
                //   85 ff               test edi, edi
                //   0f 88 .. .. .. ..   js   <fatal-error path>
                //
                // The fatal path calls `LdrpLogFatalUserError` (→
                // `NtRaiseHardError(STATUS_APP_INIT_FAILURE)`) and then
                // `NtTerminateProcess(edi)`. In Win2022 emulation,
                // LdrpInitializeProcess fails inside `LdrpResGetMappingSize`
                // (resource-section processing of a KnownDll) — a path we
                // don't fully model — and returns STATUS_INTERNAL_ERROR.
                // The rest of the loader state is already populated, so the
                // failure isn't actually fatal for the EXE we want to run.
                //
                // Replace the 6-byte `js rel32` with 6 NOPs so the test
                // result is ignored. Execution falls through to the normal
                // post-LdrInit path that does `test [r14+0x17ee], r12w;
                // call ZwTestAlert; add rsp, 0x40; ret` — leaving the
                // partially-initialised loader state in place for the EXE
                // entrypoint. (Combined with the AC-bypass patch above this
                // gets us all the way past LdrInit on Win2022 ntdll.)
                if let Some(ntdll_pe) = self.maps.get_map_by_name("ntdll.pe") {
                    let ntdll_base = ntdll_pe.get_base();
                    let ntdll_size: usize = 0x800000;
                    // Pattern: `mov r12d, 0x2000 ; test edi, edi ; js rel32`
                    let needle: &[u8] =
                        &[0x41, 0xbc, 0x00, 0x20, 0x00, 0x00, 0x85, 0xff, 0x0f, 0x88];
                    let mut found_va: Option<u64> = None;
                    for off in 0..ntdll_size.saturating_sub(needle.len() + 4) {
                        let va = ntdll_base + off as u64;
                        let mut matches = true;
                        for (i, &b) in needle.iter().enumerate() {
                            if self.maps.read_byte(va + i as u64).unwrap_or(0xff) != b {
                                matches = false;
                                break;
                            }
                        }
                        if matches {
                            found_va = Some(va);
                            break;
                        }
                    }
                    if let Some(anchor) = found_va {
                        // The 6-byte `js rel32` starts at anchor + 8.
                        let js_va = anchor + 8;
                        for i in 0..6 {
                            let _ = self.maps.write_byte(js_va + i, 0x90);
                        }
                        log::debug!(
                            "ntdll LdrInitializeThunk status-check bypass patch applied at 0x{:x}",
                            js_va,
                        );
                    } else {
                        log::debug!(
                            "ntdll LdrInitializeThunk status-check pattern not found — skipping patch"
                        );
                    }
                }

                // RtlpGetSystemDefaultUILanguage AC-init-call bypass patch
                // ---------------------------------------------------------------
                // Win2022 `ntdll!RtlpGetSystemDefaultUILanguage` (the function
                // body around RVA 0x3b4a0) reaches an internal helper whose
                // job is to write into an activation-context-derived struct.
                // The relevant fragment is:
                //
                //   83 79 38 00       cmp dword ptr [rcx + 0x38], 0
                //   48 8b cb          mov rcx, rbx
                //   74 1a             je  skip                    ; ← patched
                //   e8 .. .. .. ..    call <internal AC helper>   ; faults
                //
                // The helper reads `arg->[0x38]->[0x30]` as a base pointer and
                // adds `arg->[0x84] * 8` as an index, writing one qword at the
                // computed address. Under emulation we don't initialise the
                // activation-context table the helper expects, so the index
                // ends up landing inside `kernelbase.rdata` (read-only) and
                // the write faults — taking the whole `LdrInitializeThunk`
                // down with it (now that the no-handler exception path stops
                // emulation).
                //
                // The branch ABOVE this site (`je 0x18003b4f8`) is the normal
                // "AC table absent → skip helper" path on real Windows; we
                // flip it to an unconditional `jmp` so the call is *always*
                // skipped. Downstream code reads the function's return value
                // as an NTSTATUS, but the success path in the caller already
                // jumps to the same continuation when the helper is skipped,
                // so bypassing it leaves the function semantically equivalent
                // to "no activation context applied" — which matches what
                // mwemu actually models.
                //
                // Pattern intentionally omits the rel8 displacement of the
                // `je` (the last byte): the offset varies between builds, but
                // the cmp/mov-rcx-rbx/je-against-zero sequence is unique.
                if let Some(ntdll_pe) = self.maps.get_map_by_name("ntdll.pe") {
                    let ntdll_base = ntdll_pe.get_base();
                    let ntdll_size: usize = 0x800000;
                    let needle: &[u8] = &[0x83, 0x79, 0x38, 0x00, 0x48, 0x8b, 0xcb, 0x74];
                    let mut found_va: Option<u64> = None;
                    for off in 0..ntdll_size.saturating_sub(needle.len() + 1) {
                        let va = ntdll_base + off as u64;
                        let mut matches = true;
                        for (i, &b) in needle.iter().enumerate() {
                            if self.maps.read_byte(va + i as u64).unwrap_or(0xff) != b {
                                matches = false;
                                break;
                            }
                        }
                        if matches {
                            found_va = Some(va);
                            break;
                        }
                    }
                    if let Some(anchor) = found_va {
                        // `je` opcode lives at anchor + 7; rewrite as `jmp short`.
                        let je_va = anchor + 7;
                        let _ = self.maps.write_byte(je_va, 0xeb);
                        log::debug!(
                            "ntdll RtlpGetSystemDefaultUILanguage AC bypass patch applied at 0x{:x}",
                            je_va,
                        );
                    } else {
                        log::debug!(
                            "ntdll RtlpGetSystemDefaultUILanguage AC bypass pattern not found — skipping patch"
                        );
                    }
                }
            }

            // Minimal TEB stack bounds (NtTib) so stack probes do not fault.
            // NtTib.StackBase (off +0x08) = HIGH address — the address where RSP
            //   was at thread creation (the "top of stack"). On Windows this is
            //   *not* the end of the reservation but the initial RSP value.
            // NtTib.StackLimit (off +0x10) = LOW address (bottom of committed region).
            //
            // We previously used `map.get_bottom()` for StackBase, which is the end
            // of the underlying mwemu map and sits 0x2000 above the initial RSP.
            // ntdll's exception dispatcher uses NtTib.StackBase as the upper bound
            // for stack scratch buffers; if it's past the real stack top, those
            // writes spill past the map and fault. Use the initial RSP instead.
            let stack_lo = self
                .maps
                .get_map_by_name("stack")
                .map(|s| s.get_base())
                .unwrap_or(self.cfg.stack_addr);
            let stack_hi = self.regs().rsp; // initial RSP = canonical NtTib.StackBase
            let teb_map = self.maps.get_mem_mut("teb");
            let teb_addr = teb_map.get_base();
            let mut teb = structures::TEB64::load_map(teb_addr, teb_map);
            teb.nt_tib.stack_base = stack_hi;
            teb.nt_tib.stack_limit = stack_lo;
            // NtTib.Self must point to the TEB itself (gs:[0x30] canonical value).
            teb.nt_tib.self_pointer = teb_addr;
            teb.save(teb_map);

            return Ok(());
        }

        // Update exe_name from the real EXE basename before PEB init writes it.
        peb64::refresh_exe_name_from_filename(self);
        peb64::init_peb(self);
        kuser_shared::init_kuser_shared_data(self);

        let mut metadata = Vec::new();
        let core_libraries = ["ntdll.dll", "kernel32.dll", "kernelbase.dll"];

        // Map all system libraries before binding imports between them.
        for dll in core_libraries {
            self.ensure_maps_dll(dll); // fetch from the symbol server if missing
            let filepath = self.cfg.get_maps_folder(dll);
            log::debug!("mapping base lib64: {}", filepath);
            metadata.push((dll.to_string(), self.map_dll_pe64(&filepath)?));
        }

        let mut dependencies: BTreeSet<String> = BTreeSet::new();
        for (_, dll) in &metadata {
            for mut dep in dll.image.get_dependencies() {
                dep = dep.to_lowercase();
                if !dep.ends_with(".dll") {
                    dep.push_str(".dll");
                }
                if !core_libraries.contains(&dep.as_str()) {
                    dependencies.insert(dep);
                }
            }
        }

        for dll in dependencies {
            self.ensure_maps_dll(&dll); // fetch from the symbol server if missing
            let filepath = self.cfg.get_maps_folder(&dll);
            log::debug!("mapping dependency {}", filepath);
            metadata.push((dll.to_string(), self.map_dll_pe64(&filepath)?));
        }

        for (name, dll) in &metadata {
            log::debug!("dynamic linking {}", name);
            peb64::dynamic_link_module(dll.base, dll.image.get_pe_off(), name, self);
        }

        // Relocations and exports are ready for every dependency.
        for (name, dll) in &mut metadata {
            log::debug!("iat binding {}", name);
            dll.image.iat_binding(&dll.raw, self, dll.base);
            dll.image.delay_load_binding(&dll.raw, self, dll.base);
        }
        log::debug!("win32 64bits base libs ok.");

        let ntdll_base = self.maps.get_mem("ntdll.pe").get_base();
        peb64::update_peb_image_base(self, ntdll_base);

        let (stack_base, stack_limit) = self
            .maps
            .get_map_by_name("stack")
            .map(|s| (s.get_base(), s.get_bottom()))
            .unwrap_or((self.cfg.stack_addr, self.cfg.stack_addr + 0x100000 + 0x2000));
        let teb_map = self.maps.get_mem_mut("teb");
        let mut teb = structures::TEB64::load_map(teb_map.get_base(), teb_map);
        teb.nt_tib.stack_base = stack_base;
        teb.nt_tib.stack_limit = stack_limit;
        teb.save(teb_map);

        let heap_sz = 0x885900 - 0x4b5000;
        self.heap_addr = 0x520000; // Hardcoded in PEB64
        self.maps
            .create_map(".heap", self.heap_addr, heap_sz, Permission::READ_WRITE)
            .map_err(|error| MwemuError::new(&format!("cannot create heap map: {error}")))?;

        if self.heap_arenas.is_empty() {
            let heap = O1Heap::new(self.heap_addr, heap_sz as u32).map_err(|error| {
                MwemuError::new(&format!("cannot initialize process heap: {error}"))
            })?;
            self.heap_arenas.push(Box::new(heap));

            // Reserve the first 0x800 bytes for the _HEAP header so the
            // allocator never hands them out and overwrites the struct fields.
            self.heap_arenas[0].allocate(0x800);

            let heap_handle = HeapHandle::new(0x0, heap_sz as usize, 0, self.heap_arenas.len() - 1);
            let key = self.handle_management.insert_heap_handle(heap_handle);
            self.handle_management
                .set_heap_base_addr(key, self.heap_addr);
            let process_key = self.handle_management.get_or_insert_process_heap();
            debug_assert_eq!(key, process_key);
        }

        // Populate the _HEAP structure via Heap64 (see docs/ANTI_EMU_HEAP.md).
        let heap_struct = structures::Heap64::new(self.heap_addr, heap_sz);
        let heap_map = self.maps.get_mem_mut(".heap");
        heap_struct.save(heap_map);
        Ok(())
    }
}
