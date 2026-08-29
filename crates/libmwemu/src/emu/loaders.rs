use iced_x86::Register;

use crate::arch::Arch;
use crate::emu::Emu;
use crate::loaders::macho::macho64::Macho64;
use crate::maps::mem64::Permission;
use crate::winapi::winapi64;
use crate::windows::constants;
use crate::windows::peb::{peb32, peb64};
use rs_header::elf::elf32::Elf32;
use rs_header::elf::elf64::Elf64;
use rs_header::pe::{
    IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_ARM64, IMAGE_FILE_MACHINE_I386, pe_machine_type,
};

mod elf;
mod macho;
mod pe;

impl Emu {
    /// Classify a target the way `load_code` will, reporting whether it is a
    /// non-Windows guest (ELF or Mach-O) that needs no Windows system DLLs.
    /// Shellcode is, by definition, "none of the known formats" — it takes the
    /// Windows path, so this returns false for it (and for PE). Uses the very
    /// same detectors as `load_code`, so the two never drift. `force_shellcode`
    /// mirrors `--is_shellcode`: it forces the Windows shellcode path.
    pub fn is_non_windows_file(filename: &str, force_shellcode: bool) -> bool {
        if force_shellcode {
            return false;
        }
        let raw = std::fs::read(filename).unwrap_or_default();
        Elf32::is_elf32(&raw)
            || Elf64::is_elf64_x64(&raw)
            || Elf64::is_elf64_aarch64(&raw)
            || Macho64::is_macho64_aarch64(filename)
            || Macho64::is_macho64_x64(filename)
    }

    /// Detect the PE COFF `Machine` field of an input file without initializing
    /// the emulator or touching any maps. Returns:
    /// * `Some(IMAGE_FILE_MACHINE_I386)`  for PE32 (x86)
    /// * `Some(IMAGE_FILE_MACHINE_AMD64)` for PE32+ (x86_64)
    /// * `Some(IMAGE_FILE_MACHINE_ARM64)` for PE32+ (ARM64)
    /// * `None` for ELF, Mach-O, shellcode, malformed, or unreadable inputs.
    ///
    /// This is the CLI's pre-load check: it lets us reject `-6` (x86_64) combined
    /// with a PE32 (x86) input *before* any `--winver`/symbol-server download,
    /// while leaving shellcode, ELF, and Mach-O on their existing paths. The
    /// detector is `rs_header::pe::pe_machine_type`, the same one `load_code`
    /// uses for its dispatch, so this never disagrees with the loader.
    pub fn detect_pe_arch(filename: &str) -> Option<u16> {
        let raw = std::fs::read(filename).unwrap_or_default();
        pe_machine_type(&raw)
    }

    /// Pure decision helper for the CLI: does the requested architecture
    /// conflict with the input file's PE architecture? Returns the user-facing
    /// error message when so, `None` when the combination is fine or the input
    /// is not a PE (shellcode, ELF, Mach-O, garbage all fall through — the
    /// CLI's later handlers deal with those).
    ///
    /// Kept as a free-standing pure function (no `self`, no maps, no I/O beyond
    /// a single `std::fs::read`) so it can be unit-tested without bringing up
    /// `load_code` or the Windows simulator.
    pub fn pe32_x64_mismatch_error(filename: &str, x64_requested: bool) -> Option<&'static str> {
        if x64_requested && Self::detect_pe_arch(filename) == Some(IMAGE_FILE_MACHINE_I386) {
            Some("input binary is PE32/x86, but -6 requests x86_64 emulation")
        } else {
            None
        }
    }

    /// Load a sample. It can be PE32, PE64, ELF32, ELF64 or shellcode.
    /// If its a shellcode cannot be known if is for windows or linux, it triggers also init() to
    /// setup windows simulator.
    /// For now mwemu also don't know if shellcode is for 32bits or 64bits, in commandline -6 has
    /// to be selected for indicating 64bits, and from python or rust the emu32() or emu64()
    /// construtor dtermines the engine.
    pub fn load_code(&mut self, filename: &str) {
        self.filename = filename.to_string();
        self.cfg.filename = self.filename.clone();

        // Read the file once for the rs-header byte-based format detectors.
        // (Mach-O / PE detection below still take a path; this is just I/O.)
        // A missing/unreadable file must fail loudly here: swallowing the error
        // into empty bytes gets it misdetected as shellcode and panics later
        // creating the code map. Report clearly and bail instead.
        let raw = match std::fs::read(filename) {
            Ok(bytes) => bytes,
            Err(e) => {
                log::error!("cannot read '{}': {}", filename, e);
                eprintln!(
                    "[mwemu] cannot open '{}': {} — check the -f path",
                    filename, e
                );
                return;
            }
        };

        // ELF32
        if Elf32::is_elf32(&raw) && !self.cfg.shellcode {
            self.os = crate::arch::OperatingSystem::Linux;
            self.cfg.arch = Arch::X86;

            log::trace!("elf32 detected.");
            let mut elf32 = match Elf32::parse(&raw) {
                Ok(e) => e,
                Err(err) => {
                    log::error!("elf32 parse failed: {err}");
                    self.os = crate::arch::OperatingSystem::Linux;
                    self.cfg.arch = Arch::X86;
                    return;
                }
            };
            elf32.load(&mut self.maps);
            let stack_sz = 0x30000;
            let stack = self.alloc("stack", stack_sz, Permission::READ_WRITE);
            self.regs_mut().rsp = stack + (stack_sz / 2);

            // Set RIP from the ELF32 entry point. Mirrors the ELF64 path in
            // `load_elf64` (which uses `rebase_vaddr(e_entry)` then `set_pc`).
            // Without this branch `load_code` returns with `rip == 0` and
            // downstream tests like `hello_linux_x86` fail at
            // `assert!(entry != 0, …)`. `Elf32::load` already rebased each
            // `PT_LOAD` segment with the same `base` (ELF32_DYN_BASE for
            // dynamic/PIE binaries, 0 for ET_EXEC), so apply that base to
            // `e_entry` here for parity. The `cfg.entry_point` override
            // branch matches the convention used everywhere else.
            let entry_raw = elf32.elf_hdr.e_entry as u64;
            let base = elf32.base();
            let resolved_entry = if entry_raw != 0 && entry_raw < base {
                entry_raw + base
            } else {
                entry_raw
            };
            if self.cfg.entry_point != constants::CFG_DEFAULT_BASE {
                self.regs_mut().rip = self.cfg.entry_point;
            } else {
                self.regs_mut().rip = resolved_entry;
            }

            self.elf32 = Some(elf32);


        // ELF64 AArch64
        } else if Elf64::is_elf64_aarch64(&raw) && !self.cfg.shellcode {
            self.os = crate::arch::OperatingSystem::Linux;
            self.cfg.arch = Arch::Aarch64;
            self.maps.is_64bits = true;
            self.maps.clear();

            log::trace!("elf64 aarch64 detected.");
            // load_elf64 handles thread conversion (via init_linux64_aarch64)
            // and sets PC via set_pc()
            self.load_elf64(filename);

        // Mach-O AArch64
        } else if Macho64::is_macho64_aarch64(filename) && !self.cfg.shellcode {
            self.cfg.arch = Arch::Aarch64;
            self.maps.is_64bits = true;
            self.maps.clear();
            // CLI may have built the emu with x86 defaults; switch the
            // decode state machinery to AArch64 so the run loop doesn't
            // hit `unreachable!()` decoding ARM bytes against
            // `ArchState::X86`.
            self.ensure_arch_state_aarch64();

            // Switch to the macOS dylib folder for arm64. The CLI defaults
            // `cfg.maps_folder` to `maps/windows/...` before knowing the
            // binary is a Mach-O, so override when we still see a Windows
            // path or no path at all.
            let cur = self.cfg.maps_folder.as_str();
            if cur.is_empty() || cur.contains("windows") {
                if std::path::Path::new("maps/macos/aarch64").exists() {
                    self.cfg.maps_folder = "maps/macos/aarch64/".to_string();
                } else if std::path::Path::new("../../maps/macos/aarch64").exists() {
                    self.cfg.maps_folder = "../../maps/macos/aarch64/".to_string();
                }
            }

            log::trace!("macho64 aarch64 detected.");
            self.load_macho64(filename);

        // Mach-O x86_64
        } else if Macho64::is_macho64_x64(filename) && !self.cfg.shellcode {
            self.cfg.arch = Arch::X86_64;
            self.maps.is_64bits = true;
            self.maps.clear();

            // Set maps folder for macOS dylibs (try repo root, then relative from crate)
            if self.cfg.maps_folder.is_empty() {
                if std::path::Path::new("maps/macos/x86_64").exists() {
                    self.cfg.maps_folder = "maps/macos/x86_64/".to_string();
                } else if std::path::Path::new("../../maps/macos/x86_64").exists() {
                    self.cfg.maps_folder = "../../maps/macos/x86_64/".to_string();
                }
            }

            log::trace!("macho64 x86_64 detected.");
            self.load_macho64(filename);

        // ELF64 x86_64
        } else if Elf64::is_elf64_x64(&raw) && !self.cfg.shellcode {
            self.os = crate::arch::OperatingSystem::Linux;
            self.cfg.arch = Arch::X86_64;
            self.maps.clear();

            log::trace!("elf64 x86_64 detected.");
            self.load_elf64(filename);

        // PE: use COFF Machine field to distinguish x86 / x86_64 / ARM64
        } else if !self.cfg.shellcode && pe_machine_type(&raw) == Some(IMAGE_FILE_MACHINE_I386) {
            log::trace!(
                "PE32 x86 header detected (Machine=0x{:04x}).",
                IMAGE_FILE_MACHINE_I386
            );
            let clear_registers = false; // TODO: this needs to be more dynamic, like if we have a register set via args or not
            let clear_flags = false; // TODO: this needs to be more dynamic, like if we have a flag set via args or not
            self.cfg.arch = Arch::X86;
            self.os = crate::arch::OperatingSystem::Windows;

            // Set maps folder for Windows DLLs (try repo root, then relative from crate)
            if self.cfg.maps_folder.is_empty() {
                if std::path::Path::new("maps/windows/x86").exists() {
                    self.cfg.maps_folder = "maps/windows/x86/".to_string();
                } else if std::path::Path::new("../../maps/windows/x86").exists() {
                    self.cfg.maps_folder = "../../maps/windows/x86/".to_string();
                }
            }

            self.init_win32(clear_registers, clear_flags);
            let (base, _pe_off) = self.load_pe32(filename, true, 0);
            let ep = self.regs().rip;
            // emulating tls callbacks

            /*
            for i in 0..self.tls_callbacks.len() {
                self.regs_mut().rip = self.tls_callbacks[i];
                log::trace!("emulating tls_callback {} at 0x{:x}", i + 1, self.regs().rip);
                self.stack_push32(base);
                self.run(Some(base as u64));
            }*/

            // start loading dll
            // For a DLL's entry point, the OS calls DllMain with stdcall:
            //   BOOL __stdcall DllMain(HINSTANCE hinstDLL, DWORD fdwReason, LPVOID lpvReserved);
            // which on x86 means the stack must contain a return address followed
            // by the three arguments. We push the arguments in *reverse* order
            // (caller-convention right-to-left) so the callee sees them at the
            // expected offsets after it pops the return address with ret 12.
            match self.pe32 {
                Some(ref pe32) => {
                    if pe32.is_dll() {
                        log::trace!("emulating DllMain x86 base=0x{:x}", base);
                        self.stack_push32(0); // lpvReserved
                        self.stack_push32(1); // fdwReason = DLL_PROCESS_ATTACH
                        self.stack_push32(base); // hinstDLL
                        // fake return address so the entry's `ret` doesn't crash
                        // before DllMain executes its prolog (base is mapped RW).
                        self.stack_push32(base);
                    }
                }
                _ => {
                    log::error!("No Pe32 found inside self");
                }
            }

            self.regs_mut().rip = ep;

        // PE64 ARM64
        } else if !self.cfg.shellcode && pe_machine_type(&raw) == Some(IMAGE_FILE_MACHINE_ARM64) {
            log::trace!(
                "PE64 ARM64 header detected (Machine=0x{:04x}). Windows AArch64 PE recognized.",
                IMAGE_FILE_MACHINE_ARM64
            );
            self.cfg.arch = Arch::Aarch64;
            self.os = crate::arch::OperatingSystem::Windows;
            self.maps.is_64bits = true;

            // Set maps folder for Windows ARM64 DLLs
            if self.cfg.maps_folder.is_empty() {
                if std::path::Path::new("maps/windows/aarch64").exists() {
                    self.cfg.maps_folder = "maps/windows/aarch64/".to_string();
                } else if std::path::Path::new("../../maps/windows/aarch64").exists() {
                    self.cfg.maps_folder = "../../maps/windows/aarch64/".to_string();
                }
            }

            let clear_registers = false;
            let clear_flags = false;
            self.init_win32(clear_registers, clear_flags);
            let (base, _pe_off) = self.load_pe64(filename, true, 0);
            let ep = self.pc();

            match self.pe64 {
                Some(ref pe64) => {
                    if pe64.is_dll() {
                        let regs = self.regs_aarch64_mut();
                        regs.x[0] = base; // hinstDLL
                        regs.x[1] = 1; // fdwReason = DLL_PROCESS_ATTACH
                        regs.x[2] = 0; // lpvReserved
                    }
                }
                _ => {
                    log::error!("No Pe64 found inside self");
                }
            }

            self.set_pc(ep);

        // PE64 x86_64
        } else if !self.cfg.shellcode && pe_machine_type(&raw) == Some(IMAGE_FILE_MACHINE_AMD64) {
            log::trace!(
                "PE64 x86_64 header detected (Machine=0x{:04x}).",
                IMAGE_FILE_MACHINE_AMD64
            );
            let clear_registers = false; // TODO: this needs to be more dynamic, like if we have a register set via args or not
            let clear_flags = false; // TODO: this needs to be more dynamic, like if we have a flag set via args or not
            self.cfg.arch = Arch::X86_64;
            self.os = crate::arch::OperatingSystem::Windows;

            // Set maps folder for Windows DLLs (try repo root, then relative from crate)
            if self.cfg.maps_folder.is_empty() {
                if std::path::Path::new("maps/windows/x86_64").exists() {
                    self.cfg.maps_folder = "maps/windows/x86_64/".to_string();
                } else if std::path::Path::new("../../maps/windows/x86_64").exists() {
                    self.cfg.maps_folder = "../../maps/windows/x86_64/".to_string();
                }
            }

            self.init_win32(clear_registers, clear_flags);
            let (base, _pe_off) = self.load_pe64(filename, true, 0);
            let ep = self.regs().rip;

            match self.pe64 {
                Some(ref pe64) => {
                    // start loading dll
                    if pe64.is_dll() {
                        self.regs_mut().set_reg(Register::RCX, base);
                        self.regs_mut().set_reg(Register::RDX, 1);
                        self.regs_mut().set_reg(Register::R8L, 0);
                    }
                }
                _ => {
                    log::error!("No Pe64 found inside self");
                }
            }
            // Optional SSDT loader bootstrap: call ntdll!LdrInitializeThunk to perform loader init.
            if self.cfg.emulate_winapi {
                let ldr_init = winapi64::kernel32::resolve_api_name_in_module(
                    self,
                    "ntdll.dll",
                    "LdrInitializeThunk",
                );
                if ldr_init != 0 {
                    // Arrange return to entrypoint so execution continues normally.
                    self.regs_mut().rip = ep;
                    log::trace!("Initializing win32 64bits emulating ntdll!LdrInitializeThunk");
                    // LdrInitializeThunk(PCONTEXT Context, PVOID NtdllBase, PVOID Unused)
                    // The second argument must be ntdll's image base so the loader
                    // can parse its own PE headers during init.
                    let ntdll_base = self.maps.get_mem("ntdll.pe").get_base();

                    // Build a minimal x64 CONTEXT structure so LdrInitializeThunk does not
                    // null-deref when it reads CONTEXT.Rip to find the process entry point.
                    // x64 CONTEXT size is 0x4D0 bytes; key offsets:
                    //   +0x30  ContextFlags   (DWORD)
                    //   +0x98  Rsp            (QWORD)
                    //   +0xF8  Rip            (QWORD)
                    const CTX_SIZE: u64 = 0x4D0;
                    const CONTEXT_FULL: u32 = 0x10_007F;
                    let ctx_addr = self
                        .maps
                        .lib64_alloc(CTX_SIZE)
                        .expect("cannot alloc CONTEXT for LdrInitializeThunk");
                    self.maps
                        .create_map("ldr_context", ctx_addr, CTX_SIZE, Permission::READ_WRITE)
                        .expect("cannot create ldr_context map");
                    // ContextFlags
                    let _ = self.maps.write_dword(ctx_addr + 0x30, CONTEXT_FULL);
                    // Rsp: current stack pointer
                    let _ = self.maps.write_qword(ctx_addr + 0x98, self.regs().rsp);
                    // Rip: entry point — NtContinue will redirect execution here
                    let _ = self.maps.write_qword(ctx_addr + 0xF8, ep);

                    let _ = self.call64(ldr_init, &[ctx_addr, ntdll_base, 0]);
                    self.ldr_init_done = true;
                    if self.process_terminated {
                        log::trace!(
                            "ntdll!LdrInitializeThunk DID NOT complete — bailed out mid-init (process_terminated set). pos={}",
                            self.pos,
                        );
                    } else if self.regs().rip == ep {
                        log::trace!(
                            "ntdll!LdrInitializeThunk emulated completely. pos={} rip=ep=0x{:x}",
                            self.pos,
                            ep,
                        );
                    } else {
                        log::trace!(
                            "ntdll!LdrInitializeThunk returned but rip=0x{:x} (expected ep=0x{:x}). pos={}",
                            self.regs().rip,
                            ep,
                            self.pos,
                        );
                    }

                    // Some ntdll versions (notably newer Win10/Win11/Server2022) reset
                    // PEB_LDR_DATA during LdrInitializeThunk and rely on an internal
                    // RB-tree (LdrpModuleBaseAddressIndex) for lookups, leaving the
                    // legacy `In{Load,Memory,Initialization}OrderModuleList` linked
                    // lists empty. PEB-walking shellcode still walks the linked list,
                    // so we re-populate it here from our `.pe` maps in the order
                    // expected on real Windows: EXE, ntdll, kernel32, kernelbase, ...
                    {
                        let peb_base = self.maps.get_mem("peb").get_base();
                        let ldr_addr = self.maps.read_qword(peb_base + 0x18).unwrap_or(0);
                        if ldr_addr != 0 {
                            let sentinel_mem = ldr_addr + 0x20;
                            let first = self.maps.read_qword(sentinel_mem).unwrap_or(0);
                            if first == 0 || first == sentinel_mem {
                                log::trace!(
                                    "LDR InMemoryOrder list empty post-LdrInit — repopulating"
                                );
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
                                    let pe_off = self.maps.read_dword(base + 0x3c).unwrap_or(0);
                                    crate::windows::peb::peb64::dynamic_link_module(
                                        base, pe_off, &name, self,
                                    );
                                    log::trace!(
                                        "  repopulated LDR entry {} base=0x{:x}",
                                        name,
                                        base
                                    );
                                }
                            }
                        }
                    }

                    // Some ntdll versions allocate the EXE module's name buffer
                    // during LdrInitializeThunk but never copy the path into it
                    // under emulation, leaving FullDllName/BaseDllName pointing at
                    // uninitialized (later freed) heap. Patch it with a stable
                    // mwemu-owned buffer so PEB-walking code reads the real name.
                    crate::windows::peb::peb64::fix_exe_module_name(self);

                    // DEBUG: dump InMemoryOrder chain so we can verify a PEB-walking
                    // shellcode finds the expected DllBase at the expected index.
                    {
                        let peb_base = self.maps.get_mem("peb").get_base();
                        let ldr = self.maps.read_qword(peb_base + 0x18).unwrap_or(0);
                        log::trace!("DEBUG ldr_chain: PEB=0x{:x} Ldr=0x{:x}", peb_base, ldr);
                        // Dump first 64 bytes of the PEB_LDR_DATA so we can see if
                        // ntdll restructured the lists.
                        let mut hex = String::new();
                        for j in 0..64u64 {
                            let b = self.maps.read_byte(ldr + j).unwrap_or(0);
                            hex.push_str(&format!("{:02x} ", b));
                        }
                        log::trace!("DEBUG ldr_dump[0..64]: {}", hex);
                        let sentinel = ldr + 0x20;
                        let mut cur = self.maps.read_qword(sentinel).unwrap_or(0);
                        log::trace!("DEBUG sentinel=0x{:x} first_flink=0x{:x}", sentinel, cur);
                        let mut i = 0;
                        while cur != 0 && cur != sentinel && i < 16 {
                            // cur points to &entry.InMemoryOrderLinks (offset 0x10 in LDR_DATA_TABLE_ENTRY)
                            let entry = cur.wrapping_sub(0x10);
                            let dll_base = self.maps.read_qword(entry + 0x30).unwrap_or(0);
                            // BaseDllName UNICODE_STRING is at offset 0x58: Length(W), MaxLen(W), pad(D), Buffer(Q)
                            let name_len = self.maps.read_word(entry + 0x58).unwrap_or(0) as u64;
                            let name_buf = self.maps.read_qword(entry + 0x58 + 8).unwrap_or(0);
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
                } else if self.cfg.verbose >= 1 {
                    log::trace!("ssdt: could not resolve ntdll!LdrInitializeThunk");
                }
            }
            // Run the PE's TLS callbacks before the entry point. Win64 ABI:
            //   rcx = hModule (image base), rdx = Reason = DLL_PROCESS_ATTACH (1),
            //   r8  = Reserved (0).
            // Trampoline: push the image base as the return address and run until
            // the callback rets back to it. A callback that faults is logged and
            // the rest are skipped rather than aborting the load.
            //
            // TLS callbacks are auto-injected CRT init that runs *before* the
            // real entry point, so run them best-effort: unimplemented APIs are
            // skipped (not fatal) during callbacks, then the caller's crash-on-gap
            // policy is restored for the actual program.
            if !self.tls_callbacks.is_empty() {
                let prev_skip = self.cfg.skip_unimplemented;
                self.cfg.skip_unimplemented = true;
                for i in 0..self.tls_callbacks.len() {
                    let cb = self.tls_callbacks[i];
                    log::trace!("emulating TLS callback {} at 0x{:x}", i + 1, cb);
                    self.regs_mut().rcx = base;
                    self.regs_mut().rdx = 1;
                    self.regs_mut().r8 = 0;
                    self.stack_push64(base);
                    self.regs_mut().rip = cb;
                    if let Err(e) = self.run(Some(base)) {
                        log::warn!("TLS callback {} at 0x{:x} failed: {}", i + 1, cb, e);
                        break;
                    }
                }
                self.cfg.skip_unimplemented = prev_skip;
            }

            // If LdrInitializeThunk bailed via NtTerminateProcess, do not
            // continue with the EXE entry point: ntdll's loader state is
            // partially initialised and any further execution will crash
            // somewhere unrelated. Leave RIP at the syscall site so the
            // operator sees the actual termination point.
            if self.process_terminated {
                log::error!(
                    "ntdll!LdrInitializeThunk terminated the process during init. \
                     Skipping EXE entry. Last rip=0x{:x}",
                    self.regs().rip,
                );
            } else {
                self.regs_mut().rip = ep;
            }

        // Shellcode
        } else {
            log::trace!("shellcode detected.");
            let clear_registers = false; // TODO: this needs to be more dynamic, like if we have a register set via args or not
            let clear_flags = false; // TODO: this needs to be more dynamic, like if we have a flag set via args or not
            self.init_win32(clear_registers, clear_flags);
            let exe_name = self.cfg.exe_name.clone();
            if self.cfg.is_x64() {
                let (base, _pe_off) =
                    self.load_pe64(&format!("{}/{}", self.cfg.maps_folder, exe_name), false, 0);
                peb64::update_ldr_entry_base(&exe_name, base, self);
            } else {
                let (base, _pe_off) =
                    self.load_pe32(&format!("{}/{}", self.cfg.maps_folder, exe_name), false, 0);
                peb32::update_ldr_entry_base(&exe_name, base as u64, self);
            }

            if !self
                .maps
                .create_map(
                    "code",
                    self.cfg.code_base_addr,
                    0,
                    Permission::READ_WRITE_EXECUTE,
                )
                .expect("cannot create code map")
                .load(filename)
            {
                log::trace!("shellcode not found, select the file with -f");
                return;
            }
            let code = self.maps.get_mem_mut("code");
            code.extend(0xffff); // this could overlap an existing map
        }

        if self.cfg.entry_point != constants::CFG_DEFAULT_BASE {
            self.regs_mut().rip = self.cfg.entry_point;
        }

        /*if self.cfg.code_base_addr != constants::CFG_DEFAULT_BASE {
            let code = self.maps.get_mem("code");
            code.update_base(self.cfg.code_base_addr);
            code.update_bottom(self.cfg.code_base_addr + code.size() as u64);
        }*/
    }

    /// Load a shellcode from a variable.
    /// This assumes that there is no headers like PE/ELF and it's direclty code.
    /// Any OS simulation is triggered, but init() could be called by the user
    pub fn load_code_bytes(&mut self, bytes: &[u8]) {
        if self.cfg.verbose >= 1 {
            log::trace!("Loading shellcode from bytes");
        }

        self.init_cpu();

        let code = self
            .maps
            .create_map(
                "code",
                self.cfg.code_base_addr,
                bytes.len() as u64,
                Permission::READ_WRITE_EXECUTE,
            )
            .expect("cannot create code map");
        let base = code.get_base();
        code.write_bytes(base, bytes);
        self.set_pc(base);
    }
}
