use crate::utils::helpers::unlikely;
use crate::{
    console::Console, emu::Emu, exception::types::ExceptionType, winapi::winapi32,
    winapi::winapi64, windows::constants,
};

impl Emu {
    fn resolve_unix_x64_symbol(&self, addr: u64) -> String {
        let symbol = if self.os.is_macos() {
            self.macho64
                .as_ref()
                .and_then(|m| m.addr_to_symbol.get(&addr).cloned())
        } else if self.os.is_linux() {
            self.elf64
                .as_ref()
                .and_then(|elf| elf.addr_to_symbol.get(&addr).cloned())
                .or_else(|| {
                    self.elf64
                        .as_ref()
                        .map(|elf| elf.sym_get_name_from_addr(addr))
                        .filter(|name| !name.is_empty())
                })
        } else {
            None
        };

        symbol.unwrap_or_else(|| format!("unknown_0x{:x}", addr))
    }

    fn intercept_unix_x64_api_call(&mut self, addr: u64, section_name: &str) -> bool {
        if self.skip_apicall {
            self.its_apicall = Some(addr);
            return false;
        }

        let symbol = self.resolve_unix_x64_symbol(addr);

        log::info!(
            "{}** {} API call: {} (in {}) at 0x{:x} {}",
            self.colors.light_red,
            self.pos,
            symbol,
            section_name,
            addr,
            self.colors.nc
        );

        self.gateway_return = self.stack_pop64(false).unwrap_or(0);
        self.regs_mut().rip = self.gateway_return;

        if self.os.is_macos() {
            crate::macosapi::gateway(addr, section_name, &symbol, self);
        } else if self.os.is_linux() {
            crate::linuxapi::gateway(addr, section_name, &symbol, self);
        }

        self.force_break = true;
        true
    }

    #[inline]
    pub fn set_rip_without_check(&mut self, addr: u64) {
        /*
        This function is make so that the rip is forcing to set instead of going through a long range of checking
        This is to make the switching rip faster and also forcing the rip without
         */
        self.regs_mut().rip = addr;
    }

    ///TODO: reimplement set_eip and set_rip
    /// Redirect execution flow on 64bits.
    /// If the target address is a winapi, triggers it's implementation.
    pub fn set_rip_with_check(&mut self, addr: u64, is_branch: bool) -> bool {
        self.force_reload = true;

        // A driver lives entirely above the user-mode library range, so the
        // usual `addr < LIBS64_MIN` fast path cannot tell its own code from a
        // call into the kernel. Kernel mode answers that question first.
        if unlikely(self.kernel.as_ref().is_some_and(|k| !k.ledger_only)) {
            return self.kernel_set_pc(addr);
        }

        if unlikely(addr == constants::RETURN_THREAD as u64) {
            log::trace!("/!\\ Thread returned, continuing the main thread");
            self.regs_mut().rip = self.main_thread_cont;
            Console::spawn_console(self);
            self.force_break = true;
            return true;
        }

        let name = match self.maps.get_addr_name(addr) {
            Some(n) => n, //DON'T EVER MODIFY THIS AND ADD to_string IT MAKE THE COMPILER SO SO MUCH SLOWER
            None => {
                if self.os.is_linux() {
                    return false;
                }
                // No PE64 (e.g. a Mach-O guest): there is no IAT to resolve against.
                let Some(pe64) = self.pe64.as_ref() else {
                    return false;
                };
                let import = pe64.import_addr_to_dll_and_name(addr);
                return if !import.is_empty() {
                    let (dll, api) = import.split_once('!').unwrap_or(("", ""));

                    // The target is an unresolved import: its IAT slot still holds
                    // the on-disk thunk (an `.idata` RVA), which is not executable.
                    // This happens most often with api-set CRT imports (their stub
                    // DLLs are virtual name-forwarders we don't map).
                    //
                    // Legacy `msvcrt.dll` imports are special-cased before the
                    // gateway dispatcher: when the resolved target lands in the
                    // mapped `msvcrt.text`/`msvcrtfothk` section, execute the real
                    // bytes natively instead of running the now-removed stub.
                    // Other imports still flow through `gateway_by_import`, which
                    // sets `rax` for emulated APIs (api-ms-win-crt → wincrt, etc.).
                    if dll.trim().eq_ignore_ascii_case("msvcrt.dll") {
                        let resolved =
                            winapi64::kernel32::resolve_api_name_in_module(self, dll, api);
                        let native_target = if resolved != 0 {
                            self.maps
                                .get_addr_name(resolved)
                                .map(winapi64::msvcrt::is_native_section)
                                .unwrap_or(false)
                        } else {
                            false
                        };
                        if native_target {
                            // this `set_rip_with_check`; leave the stack intact and let
                            // the msvcrt gadget `ret` itself. ret.rs will pop the matching
                            // call-stack frame.
                            winapi64::msvcrt::log_native_call(self, resolved);
                            self.set_rip_without_check(resolved);
                            self.force_break = true;
                            return true;
                        }
                        // Either the import did not resolve to a mapped msvcrt
                        // section, or it did not resolve at all. Warn and resume at
                        // the caller as if the call had been skipped: same shape as
                        // any other unrecognized import.
                        log::warn!("unhandled import {}!{}", dll, api);
                        self.gateway_return = self.stack_pop64(false).unwrap_or(0);
                        self.regs_mut().rip = self.gateway_return;
                        self.call_stack_mut().pop();
                        self.force_break = true;
                        self.is_api_run = true;
                        return true;
                    }
                    self.gateway_return = self.stack_pop64(false).unwrap_or(0);
                    self.regs_mut().rip = self.gateway_return;
                    winapi64::gateway_by_import(self, dll, api, addr);
                    self.force_break = true;
                    self.is_api_run = true;
                    true
                } else if addr == 0 {
                    log::trace!(
                        "entry point returned to address 0x0 — treating as ExitProcess(rax=0x{:x})",
                        self.regs().rax,
                    );
                    self.stop();
                    true
                } else {
                    log::error!(
                        "/!\\ set_rip setting rip to non mapped addr 0x{:x} {}",
                        addr,
                        self.filename
                    );
                    self.exception(ExceptionType::SettingRipToNonMappedAddr);
                    false
                };
            }
        };

        let map_name = self.filename.as_str();

        /*
        if addr < constants::LIBS64_MIN
            || name == "code"
            || (!map_name.is_empty() && name.starts_with(&map_name))
            || name == "loader.text"*/
        // Non-syscall mode: ntdll is mapped at its original base (below
        // LIBS64_MIN), but cross-module calls into it should be intercepted
        // just like high-address libraries.
        let is_ntdll_cross_call = addr < constants::LIBS64_MIN
            && !self.cfg.emulate_winapi
            && self.os.is_windows()
            && name.starts_with("ntdll")
            && {
                let caller_name = self.maps.get_addr_name(self.regs().rip).unwrap_or("??");
                !caller_name.starts_with("ntdll")
            };

        if addr < constants::LIBS64_MIN && !is_ntdll_cross_call {
            // This is the normal case, no linux syscall or winapi to handle
            if self.cfg.verbose > 1 {
                let rip = self.regs().rip;
                let prev = self.maps.get_addr_name(rip).unwrap_or("??");
                if unlikely(prev != name) {
                    log::trace!("{}:0x{:x} map change  {} -> {}", self.pos, rip, prev, name);
                }
            }

            self.regs_mut().rip = addr;
            true
        } else if self.os.is_linux() && self.cfg.linux_real_libc {
            // Real-libc mode: execute the actual mapped libc/ld machine code.
            // Only `__libc_start_main` is still bootstrapped in Rust so we can
            // drive `main` without running glibc's full CSU init; everything
            // else (including libc->libc internal calls) runs for real and
            // reaches the `syscall` instruction, handled by the syscall layer.
            // When the real ld.so drives the bootstrap, nothing is hooked: the
            // real libc (including __libc_start_main) executes for real. Only in
            // the lighter "no ld.so" mode do we still bootstrap main in Rust.
            if !self.ld_bootstrap {
                let symbol = self.resolve_unix_x64_symbol(addr);
                if symbol == "__libc_start_main" {
                    let section_name = name.to_string();
                    return self.intercept_unix_x64_api_call(addr, &section_name);
                }
            }
            self.regs_mut().rip = addr;
            true
        } else if self.os.is_linux() || self.os.is_macos() {
            let section_name = name.to_string();
            self.intercept_unix_x64_api_call(addr, &section_name)
        } else {
            // Here we hit the winapi case

            std::hint::cold_path();
            if self.cfg.verbose >= 2 && !self.cfg.emulate_winapi {
                log::trace!("/!\\ changing RIP to {} ", name);
            }

            // SSDT mode: always run the real DLL machine code; user-level APIs
            // like `kernel32!LoadLibraryA` execute their full path down to the
            // `syscall` instruction (which the syscall handler intercepts).
            // No Rust winapi stubs are used here — the whole point of SSDT is
            // to emulate the real DLLs and only implement the kernel syscall
            // surface.
            if unlikely(self.cfg.emulate_winapi) {
                let api_name = winapi64::kernel32::guess_api_name(self, addr);
                if !api_name.is_empty() && self.cfg.verbose >= 1 {
                    log_red!(self, "emulating {}", api_name);
                }
                if !api_name.is_empty() && ssdt_fast_path(self, &api_name) {
                    return true;
                }
                self.regs_mut().rip = addr;
                return true;
            }

            // Anti-hook trampolines run an API's first instructions themselves
            // and jump to API+N. Undo that prologue and serve the export with
            // its winapi stub; library code is never executed for real here.
            let in_code_section = name.ends_with(".text");
            let addr = if in_code_section
                && winapi64::kernel32::guess_api_name(self, addr).is_empty()
            {
                match winapi64::stolen_prologue::unwind(self, addr) {
                    Some(entry) => {
                        log::trace!(
                            "jump to 0x{:x} skips the prologue of the API at 0x{:x}",
                            addr,
                            entry
                        );
                        entry
                    }
                    None => {
                        log::error!(
                            "jump into library code at 0x{:x} that no winapi stub can serve — stopping",
                            addr
                        );
                        self.stop();
                        return true;
                    }
                }
            } else {
                addr
            };

            if self.skip_apicall {
                self.its_apicall = Some(addr);
                return false;
            }

            self.gateway_return = self.stack_pop64(false).unwrap_or(0);
            self.regs_mut().rip = self.gateway_return;

            let handle_winapi: bool =
                if let Some(mut hook_fn) = self.hooks.hook_on_winapi_call.take() {
                    let rip = self.regs().rip;
                    let result = hook_fn(self, rip, addr);
                    self.hooks.hook_on_winapi_call = Some(hook_fn);
                    result
                } else {
                    true
                };

            if unlikely(handle_winapi) {
                let section_name = match self.maps.get_addr_name(addr) {
                    Some(name) => name,
                    None => {
                        log::error!("changing RIP to non mapped addr 0x{:x}", addr);
                        return false;
                    }
                };
                if winapi64::msvcrt::is_native_section(section_name) {
                    // Native execution: the legacy msvcrt gadget owns its own
                    // `ret`. The gateway model above popped the caller's
                    // architectural return address; restore it so native `ret`
                    // consumes the right value. The matching native `ret`
                    // (ret.rs) will pop the call-stack frame we leave here.
                    let _ = self.stack_push64(self.gateway_return);
                    winapi64::msvcrt::log_native_call(self, addr);
                    self.set_rip_without_check(addr);
                } else {
                    winapi64::gateway(addr, section_name.to_string().as_str(), self);
                }
            }
            self.force_break = true;
            true
        }
    }

    /// Redirect execution flow for AArch64 branches.
    /// If the target address is in a loaded library (dylib/so), intercept and
    /// dispatch to the appropriate API handler. Mirrors set_rip() for Windows.
    pub fn set_pc_aarch64(&mut self, addr: u64) -> bool {
        if unlikely(self.kernel.as_ref().is_some_and(|k| !k.ledger_only)) {
            return self.kernel_set_pc(addr);
        }

        // If in user code range, just set PC
        if addr < constants::LIBS64_MIN {
            self.regs_aarch64_mut().pc = addr;
            return true;
        }

        // Execution is entering a library — intercept
        let name = match self.maps.get_addr_name(addr) {
            Some(n) => n.to_string(),
            None => {
                log::error!("set_pc_aarch64: addr 0x{:x} not in any mapped region", addr);
                self.regs_aarch64_mut().pc = addr;
                return false;
            }
        };

        // Look up symbol name from address (check both Mach-O and ELF maps)
        let symbol = self
            .macho64
            .as_ref()
            .and_then(|m| m.addr_to_symbol.get(&addr).cloned())
            .or_else(|| {
                self.elf64
                    .as_ref()
                    .and_then(|e| e.addr_to_symbol.get(&addr).cloned())
            })
            .unwrap_or_else(|| format!("unknown_0x{:x}", addr));

        log::info!(
            "{}** {} API call: {} (in {}) at 0x{:x} {}",
            self.colors.light_red,
            self.pos,
            symbol,
            name,
            addr,
            self.colors.nc
        );

        // Dispatch to appropriate platform API handler
        if self.os.is_macos() {
            // Set PC to return address (already in LR from BL/BLR)
            self.regs_aarch64_mut().pc = self.regs_aarch64().x[30];
            crate::macosapi::gateway(addr, &name, &symbol, self);
        } else if self.os.is_linux() {
            // Set PC to return address (already in LR from BL/BLR)
            self.regs_aarch64_mut().pc = self.regs_aarch64().x[30];
            crate::linuxapi::gateway(addr, &name, &symbol, self);
        } else if self.os.is_windows() {
            // emulate winapi mode
            if self.cfg.emulate_winapi {
                let api_name = winapi64::kernel32::guess_api_name(self, addr);
                if !api_name.is_empty() && self.cfg.verbose >= 1 {
                    log_red!(self, "emulating {}", api_name);
                }
                self.regs_aarch64_mut().pc = addr;
                return true;
            }

            if self.skip_apicall {
                self.its_apicall = Some(addr);
                return false;
            }

            // Return via LR, not stack pop
            self.regs_aarch64_mut().pc = self.regs_aarch64().x[30];
            self.gateway_return = self.regs_aarch64().x[30];

            let handle_winapi: bool =
                if let Some(mut hook_fn) = self.hooks.hook_on_winapi_call.take() {
                    let pc = self.regs_aarch64().pc;
                    let result = hook_fn(self, pc, addr);
                    self.hooks.hook_on_winapi_call = Some(hook_fn);
                    result
                } else {
                    true
                };

            if handle_winapi {
                winapi64::gateway(addr, &name, self);
            }
        }

        self.force_break = true;
        true
    }

    /// Redirect execution flow on 32bits.
    /// If the target address is a winapi, triggers it's implementation.
    pub fn set_eip(&mut self, addr: u64, is_branch: bool) -> bool {
        self.force_reload = true;

        if addr == constants::RETURN_THREAD as u64 {
            log::trace!("/!\\ Thread returned, continuing the main thread");
            self.regs_mut().rip = self.main_thread_cont;
            Console::spawn_console(self);
            self.force_break = true;
            return true;
        }

        let name = match self.maps.get_addr_name(addr) {
            Some(n) => n,
            None => {
                if self.os.is_linux() {
                    return false;
                }
                // No PE32 (e.g. a Mach-O guest): there is no IAT to resolve against.
                let Some(pe32) = self.pe32.as_ref() else {
                    return false;
                };
                let api_name = pe32.import_addr_to_name(addr as u32);
                return if !api_name.is_empty() {
                    // winapi emulation case
                    if self.cfg.emulate_winapi {
                        let api_name = winapi32::kernel32::guess_api_name(self, addr as u32);
                        if self.cfg.verbose >= 1 {
                            log_red!(self, "emulating {}", api_name);
                        }
                        self.regs_mut().set_eip(addr);
                        return true;
                    }

                    self.gateway_return = self.stack_pop32(false).unwrap_or(0) as u64;
                    self.regs_mut().rip = self.gateway_return;
                    winapi32::gateway(addr as u32, "not_loaded", self);
                    self.force_break = true;
                    self.is_api_run = true;
                    true
                } else if addr == 0 {
                    log::trace!(
                        "entry point returned to address 0x0 — treating as ExitProcess(eax=0x{:x})",
                        self.regs().get_eax(),
                    );
                    self.stop();
                    true
                } else {
                    log::error!("/!\\ setting eip to non mapped addr 0x{:x}", addr);
                    self.exception(ExceptionType::SettingRipToNonMappedAddr);
                    false
                };
            }
        };

        let map_name = self.filename_to_mapname(&self.filename);

        // Non-syscall mode: ntdll is mapped at its original base (below
        // LIBS32_MIN), but cross-module calls into it should be intercepted
        // just like high-address libraries. Mirrors the 64-bit logic in set_rip.
        let is_ntdll_cross_call = addr < constants::LIBS32_MIN
            && !self.cfg.emulate_winapi
            && self.os.is_windows()
            && name.starts_with("ntdll")
            && {
                let caller_name = self
                    .maps
                    .get_addr_name(self.regs().get_eip())
                    .unwrap_or("??");
                !caller_name.starts_with("ntdll")
            };

        if addr < constants::LIBS32_MIN && !is_ntdll_cross_call {
            if self.cfg.verbose > 1 {
                let eip = self.regs().get_eip();
                let prev = self.maps.get_addr_name(eip).unwrap_or("??");
                if prev != name {
                    log::trace!("{}:0x{:x} map change  {} -> {}", self.pos, eip, prev, name);
                }
            }

            self.regs_mut().set_eip(addr);
        } else {
            if self.cfg.verbose >= 1 && !self.cfg.emulate_winapi {
                log::trace!("/!\\ changing EIP to {} 0x{:x}", name, addr);
            }

            // winapi emulation case
            if self.cfg.emulate_winapi {
                let api_name = winapi32::kernel32::guess_api_name(self, addr as u32);
                if !api_name.is_empty() && self.cfg.verbose >= 1 {
                    log_red!(self, "emulating {}", api_name);
                }
                self.regs_mut().set_eip(addr);
                return true;
            }

            if self.skip_apicall {
                self.its_apicall = Some(addr);
                return false;
            }

            self.gateway_return = self.stack_pop32(false).unwrap_or(0).into();
            let gateway_return = self.gateway_return;
            self.regs_mut().set_eip(gateway_return);

            let handle_winapi: bool =
                if let Some(mut hook_fn) = self.hooks.hook_on_winapi_call.take() {
                    let rip = self.regs().rip;
                    let result = hook_fn(self, rip, addr);
                    self.hooks.hook_on_winapi_call = Some(hook_fn);
                    result
                } else {
                    true
                };

            if handle_winapi {
                let name = match self.maps.get_addr_name(addr) {
                    Some(n) => n,
                    None => {
                        log::error!("changing EIP to non mapped addr 0x{:x}", addr);
                        return false;
                    }
                };
                winapi32::gateway(to32!(addr), name.to_string().as_str(), self);
            }
            self.force_break = true;
        }

        true
    }
}

/// SSDT fast-path: intercept CPU-hot ntdll string-conversion functions
/// and execute them in Rust instead of emulating ~190 instructions each.
/// Returns `true` when the function was handled (RIP set to return address,
/// registers filled with the result). The CALL instruction already pushed
/// the return address before `set_rip` is invoked.
fn ssdt_fast_path(emu: &mut Emu, api_name: &str) -> bool {
    match api_name {
        "ntdll!RtlAnsiCharToUnicodeChar" => ssdt_rtl_ansi_char_to_unicode_char(emu),
        "ntdll!RtlUTF8ToUnicodeN" => ssdt_rtl_utf8_to_unicode_n(emu),
        "ntdll!RtlUnicodeToUTF8N" => ssdt_rtl_unicode_to_utf8_n(emu),
        "ntdll!RtlUnicodeToMultiByteN" => ssdt_rtl_unicode_to_multibyte_n(emu),
        "ntdll!RtlAnsiStringToUnicodeString" => ssdt_rtl_ansi_string_to_unicode_string(emu),
        "ntdll!RtlMultiByteToUnicodeN" => ssdt_rtl_multibyte_to_unicode_n(emu),
        "ntdll!RtlEqualUnicodeString" => ssdt_rtl_equal_unicode_string(emu),
        _ => false,
    }
}

/// `WCHAR RtlAnsiCharToUnicodeChar(PUCHAR *SourceCharacter)`
/// RCX = pointer to pointer to source byte(s).
/// Returns: Unicode char in AX/RAX. Advances *SourceCharacter.
fn ssdt_rtl_ansi_char_to_unicode_char(emu: &mut Emu) -> bool {
    let pp_src = emu.regs().rcx;
    let p_src = match emu.maps.read_qword(pp_src) {
        Some(v) => v,
        None => return false,
    };
    let b0 = match emu.maps.read_byte(p_src) {
        Some(v) => v,
        None => return false,
    };
    let (wch, advance): (u16, u64) = if b0 < 0x80 {
        (b0 as u16, 1)
    } else if b0 & 0xE0 == 0xC0 {
        let b1 = emu.maps.read_byte(p_src + 1).unwrap_or(0);
        let cp = ((b0 as u32 & 0x1F) << 6) | (b1 as u32 & 0x3F);
        (cp as u16, 2)
    } else if b0 & 0xF0 == 0xE0 {
        let b1 = emu.maps.read_byte(p_src + 1).unwrap_or(0);
        let b2 = emu.maps.read_byte(p_src + 2).unwrap_or(0);
        let cp = ((b0 as u32 & 0x0F) << 12) | ((b1 as u32 & 0x3F) << 6) | (b2 as u32 & 0x3F);
        (cp as u16, 3)
    } else if b0 & 0xF8 == 0xF0 {
        let b1 = emu.maps.read_byte(p_src + 1).unwrap_or(0);
        let b2 = emu.maps.read_byte(p_src + 2).unwrap_or(0);
        let b3 = emu.maps.read_byte(p_src + 3).unwrap_or(0);
        let cp = ((b0 as u32 & 0x07) << 18)
            | ((b1 as u32 & 0x3F) << 12)
            | ((b2 as u32 & 0x3F) << 6)
            | (b3 as u32 & 0x3F);
        if cp > 0xFFFF {
            (0xFFFD, 4) // replacement char for surrogates
        } else {
            (cp as u16, 4)
        }
    } else {
        (0xFFFD, 1) // invalid lead byte
    };

    emu.maps.write_qword(pp_src, p_src + advance);
    emu.regs_mut().rax = wch as u64;
    let ret = emu.stack_pop64(false).unwrap_or(0);
    emu.regs_mut().rip = ret;
    true
}

/// `NTSTATUS RtlUTF8ToUnicodeN(PWCH Dest, ULONG MaxBytesDest, PULONG ActualBytes,
///                               PCCH Src, ULONG SrcBytes)`
/// x64: RCX=Dest, RDX=MaxBytesDest, R8=ActualBytes(out), R9=Src, [RSP+0x28]=SrcBytes
fn ssdt_rtl_utf8_to_unicode_n(emu: &mut Emu) -> bool {
    let dest = emu.regs().rcx;
    let max_bytes = emu.regs().rdx as usize;
    let actual_bytes_ptr = emu.regs().r8;
    let src = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let src_bytes = emu.maps.read_qword(rsp + 0x28).unwrap_or(0) as usize;

    let max_wchars = max_bytes / 2;
    let mut si: usize = 0;
    let mut di: usize = 0;

    while si < src_bytes && di < max_wchars {
        let b0 = emu.maps.read_byte(src + si as u64).unwrap_or(0);
        let (wch, advance): (u16, usize) = if b0 < 0x80 {
            (b0 as u16, 1)
        } else if b0 & 0xE0 == 0xC0 && si + 1 < src_bytes {
            let b1 = emu.maps.read_byte(src + si as u64 + 1).unwrap_or(0);
            (((b0 as u16 & 0x1F) << 6) | (b1 as u16 & 0x3F), 2)
        } else if b0 & 0xF0 == 0xE0 && si + 2 < src_bytes {
            let b1 = emu.maps.read_byte(src + si as u64 + 1).unwrap_or(0);
            let b2 = emu.maps.read_byte(src + si as u64 + 2).unwrap_or(0);
            (
                ((b0 as u16 & 0x0F) << 12) | ((b1 as u16 & 0x3F) << 6) | (b2 as u16 & 0x3F),
                3,
            )
        } else if b0 & 0xF8 == 0xF0 && si + 3 < src_bytes {
            (0xFFFD, 4) // 4-byte → replacement (BMP only)
        } else {
            (0xFFFD, 1)
        };
        emu.maps.write_word(dest + (di * 2) as u64, wch);
        si += advance;
        di += 1;
    }

    let written = (di * 2) as u32;
    if actual_bytes_ptr != 0 {
        emu.maps.write_dword(actual_bytes_ptr, written);
    }

    emu.regs_mut().rax = 0; // STATUS_SUCCESS
    let ret = emu.stack_pop64(false).unwrap_or(0);
    emu.regs_mut().rip = ret;
    true
}

/// `NTSTATUS RtlUnicodeToUTF8N(PCHAR Dest, ULONG MaxBytesDest, PULONG ActualBytes,
///                               PCWCH Src, ULONG SrcBytes)`
/// x64: RCX=Dest, RDX=MaxBytesDest, R8=ActualBytes(out), R9=Src, [RSP+0x28]=SrcBytes
fn ssdt_rtl_unicode_to_utf8_n(emu: &mut Emu) -> bool {
    let dest = emu.regs().rcx;
    let max_bytes = emu.regs().rdx as usize;
    let actual_bytes_ptr = emu.regs().r8;
    let src = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let src_bytes = emu.maps.read_qword(rsp + 0x28).unwrap_or(0) as usize;

    let wchar_count = src_bytes / 2;
    let mut si: usize = 0;
    let mut di: usize = 0;

    while si < wchar_count && di < max_bytes {
        let wch = emu.maps.read_word(src + (si * 2) as u64).unwrap_or(0);
        if wch < 0x80 {
            emu.maps.write_byte(dest + di as u64, wch as u8);
            di += 1;
        } else if wch < 0x800 {
            if di + 1 >= max_bytes {
                break;
            }
            emu.maps
                .write_byte(dest + di as u64, (0xC0 | (wch >> 6)) as u8);
            emu.maps
                .write_byte(dest + di as u64 + 1, (0x80 | (wch & 0x3F)) as u8);
            di += 2;
        } else {
            if di + 2 >= max_bytes {
                break;
            }
            emu.maps
                .write_byte(dest + di as u64, (0xE0 | (wch >> 12)) as u8);
            emu.maps
                .write_byte(dest + di as u64 + 1, (0x80 | ((wch >> 6) & 0x3F)) as u8);
            emu.maps
                .write_byte(dest + di as u64 + 2, (0x80 | (wch & 0x3F)) as u8);
            di += 3;
        }
        si += 1;
    }

    let written = di as u32;
    if actual_bytes_ptr != 0 {
        emu.maps.write_dword(actual_bytes_ptr, written);
    }

    emu.regs_mut().rax = 0; // STATUS_SUCCESS
    let ret = emu.stack_pop64(false).unwrap_or(0);
    emu.regs_mut().rip = ret;
    true
}

/// `NTSTATUS RtlUnicodeToMultiByteN(PCHAR Dest, ULONG MaxBytes, PULONG ActualBytes,
///                                    PCWCH Src, ULONG SrcBytes)`
fn ssdt_rtl_unicode_to_multibyte_n(emu: &mut Emu) -> bool {
    let dest = emu.regs().rcx;
    let max_bytes = emu.regs().rdx as usize;
    let actual_bytes_ptr = emu.regs().r8;
    let src = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let src_bytes = emu.maps.read_qword(rsp + 0x28).unwrap_or(0) as usize;

    let wchar_count = src_bytes / 2;
    let count = wchar_count.min(max_bytes);

    for i in 0..count {
        let wch = emu.maps.read_word(src + (i * 2) as u64).unwrap_or(0);
        let byte = if wch < 0x100 { wch as u8 } else { b'?' };
        emu.maps.write_byte(dest + i as u64, byte);
    }

    if actual_bytes_ptr != 0 {
        emu.maps.write_dword(actual_bytes_ptr, count as u32);
    }

    emu.regs_mut().rax = 0;
    let ret = emu.stack_pop64(false).unwrap_or(0);
    emu.regs_mut().rip = ret;
    true
}

fn utf8_decode_one(emu: &Emu, addr: u64) -> (u16, usize) {
    let b0 = emu.maps.read_byte(addr).unwrap_or(0);
    if b0 < 0x80 {
        (b0 as u16, 1)
    } else if b0 & 0xE0 == 0xC0 {
        let b1 = emu.maps.read_byte(addr + 1).unwrap_or(0);
        (((b0 as u16 & 0x1F) << 6) | (b1 as u16 & 0x3F), 2)
    } else if b0 & 0xF0 == 0xE0 {
        let b1 = emu.maps.read_byte(addr + 1).unwrap_or(0);
        let b2 = emu.maps.read_byte(addr + 2).unwrap_or(0);
        (
            ((b0 as u16 & 0x0F) << 12) | ((b1 as u16 & 0x3F) << 6) | (b2 as u16 & 0x3F),
            3,
        )
    } else if b0 & 0xF8 == 0xF0 {
        (0xFFFD, 4)
    } else {
        (0xFFFD, 1)
    }
}

/// `NTSTATUS RtlAnsiStringToUnicodeString(PUNICODE_STRING Dest, PCANSI_STRING Src, BOOLEAN Alloc)`
fn ssdt_rtl_ansi_string_to_unicode_string(emu: &mut Emu) -> bool {
    let dest_us = emu.regs().rcx;
    let src_as = emu.regs().rdx;
    let alloc = emu.regs().r8 & 0xFF;

    let src_len = emu.maps.read_word(src_as).unwrap_or(0) as usize;
    let src_buf = emu.maps.read_qword(src_as + 8).unwrap_or(0);
    if src_buf == 0 || src_len == 0 {
        emu.maps.write_word(dest_us, 0);
        emu.regs_mut().rax = 0;
        let ret = emu.stack_pop64(false).unwrap_or(0);
        emu.regs_mut().rip = ret;
        return true;
    }

    let mut wchars: Vec<u16> = Vec::with_capacity(src_len + 1);
    let mut si: usize = 0;
    while si < src_len {
        let (wch, advance) = utf8_decode_one(emu, src_buf + si as u64);
        wchars.push(wch);
        si += advance;
    }
    let uni_byte_len = (wchars.len() * 2) as u16;

    if alloc != 0 {
        let alloc_sz = uni_byte_len as u64 + 2;
        let buf = match emu.maps.lib64_alloc(alloc_sz) {
            Some(a) => a,
            None => {
                emu.regs_mut().rax = 0xC0000017;
                let ret = emu.stack_pop64(false).unwrap_or(0);
                emu.regs_mut().rip = ret;
                return true;
            }
        };
        let name = format!("ansi2uni_{:x}", buf);
        let _ = emu.maps.create_map(
            &name,
            buf,
            alloc_sz,
            crate::maps::mem64::Permission::READ_WRITE,
        );
        emu.maps.write_word(dest_us, uni_byte_len);
        emu.maps.write_word(dest_us + 2, uni_byte_len + 2);
        emu.maps.write_qword(dest_us + 8, buf);

        for (i, &wch) in wchars.iter().enumerate() {
            emu.maps.write_word(buf + (i * 2) as u64, wch);
        }
        emu.maps.write_word(buf + uni_byte_len as u64, 0);
    } else {
        let dst_max = emu.maps.read_word(dest_us + 2).unwrap_or(0) as usize;
        let dst_buf = emu.maps.read_qword(dest_us + 8).unwrap_or(0);
        if dst_buf == 0 {
            emu.regs_mut().rax = 0xC0000005;
            let ret = emu.stack_pop64(false).unwrap_or(0);
            emu.regs_mut().rip = ret;
            return true;
        }
        let write_count = wchars.len().min(dst_max / 2);
        for (i, _) in wchars.iter().enumerate().take(write_count) {
            emu.maps.write_word(dst_buf + (i * 2) as u64, wchars[i]);
        }
        let written = (write_count * 2) as u16;
        emu.maps.write_word(dest_us, written);
        if written + 2 <= dst_max as u16 {
            emu.maps.write_word(dst_buf + written as u64, 0);
        }
    }

    emu.regs_mut().rax = 0;
    let ret = emu.stack_pop64(false).unwrap_or(0);
    emu.regs_mut().rip = ret;
    true
}

/// `NTSTATUS RtlMultiByteToUnicodeN(PWCH Dest, ULONG MaxBytesDest, PULONG BytesWritten,
///                                    PCCH Src, ULONG SrcBytes)`
fn ssdt_rtl_multibyte_to_unicode_n(emu: &mut Emu) -> bool {
    let dest = emu.regs().rcx;
    let max_bytes = emu.regs().rdx as usize;
    let bytes_written_ptr = emu.regs().r8;
    let src = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let src_bytes = emu.maps.read_qword(rsp + 0x28).unwrap_or(0) as usize;

    let max_wchars = max_bytes / 2;
    let mut si: usize = 0;
    let mut di: usize = 0;

    while si < src_bytes && di < max_wchars {
        let (wch, advance) = utf8_decode_one(emu, src + si as u64);
        emu.maps.write_word(dest + (di * 2) as u64, wch);
        si += advance;
        di += 1;
    }

    let written = (di * 2) as u32;
    if bytes_written_ptr != 0 {
        emu.maps.write_dword(bytes_written_ptr, written);
    }

    emu.regs_mut().rax = 0;
    let ret = emu.stack_pop64(false).unwrap_or(0);
    emu.regs_mut().rip = ret;
    true
}

fn read_unicode_string(emu: &Emu, us_ptr: u64) -> String {
    if us_ptr == 0 {
        return String::new();
    }
    let len = emu.maps.read_word(us_ptr).unwrap_or(0) as usize;
    let buf = emu.maps.read_qword(us_ptr + 8).unwrap_or(0);
    if buf == 0 || len == 0 {
        return String::new();
    }
    let wchar_count = len / 2;
    let mut chars = Vec::with_capacity(wchar_count);
    for i in 0..wchar_count {
        let wch = emu.maps.read_word(buf + (i * 2) as u64).unwrap_or(0);
        if wch == 0 {
            break;
        }
        chars.push(wch);
    }
    String::from_utf16_lossy(&chars)
}

/// `BOOLEAN RtlEqualUnicodeString(PCUNICODE_STRING s1, PCUNICODE_STRING s2, BOOLEAN ci)`
fn ssdt_rtl_equal_unicode_string(emu: &mut Emu) -> bool {
    let s1_ptr = emu.regs().rcx;
    let s2_ptr = emu.regs().rdx;
    let case_insensitive = (emu.regs().r8 & 0xFF) != 0;

    let s1 = read_unicode_string(emu, s1_ptr);
    let s2 = read_unicode_string(emu, s2_ptr);

    let equal = if case_insensitive {
        s1.eq_ignore_ascii_case(&s2)
    } else {
        s1 == s2
    };

    if emu.cfg.verbose >= 1 {
        log_red!(
            emu,
            "RtlEqualUnicodeString: '{}' vs '{}' ci={} => {}",
            s1,
            s2,
            case_insensitive,
            equal
        );
    }

    emu.regs_mut().rax = if equal { 1 } else { 0 };
    let ret = emu.stack_pop64(false).unwrap_or(0);
    emu.regs_mut().rip = ret;
    true
}
