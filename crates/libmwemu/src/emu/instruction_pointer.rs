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

    ///TODO: reimplement set_eip and set_rip
    /// Redirect execution flow on 64bits.
    /// If the target address is a winapi, triggers it's implementation.
    pub fn set_rip(&mut self, addr: u64, is_branch: bool) -> bool {
        self.force_reload = true;

        if addr == constants::RETURN_THREAD as u64 {
            log::trace!("/!\\ Thread returned, continuing the main thread");
            self.regs_mut().rip = self.main_thread_cont;
            Console::spawn_console(self);
            self.force_break = true;
            return true;
        }

        let name = match self.maps.get_addr_name(addr) {
            Some(n) => n, //DON"T EVER MODIFY THIS AND ADD to_string IT MAKE THE COMPILER SO SO MUCH SLOWER
            None => {
                if self.os.is_linux() {
                    return false;
                }
                let import = self
                    .pe64
                    .as_ref()
                    .unwrap()
                    .import_addr_to_dll_and_name(addr);
                if !import.is_empty() {
                    let (dll, api) = import.split_once('!').unwrap_or(("", ""));

                    // In SSDT mode (`emulate_winapi`), we usually execute the real mapped DLL code.
                    // But api-set CRT imports are virtual; if they weren't bound, the IAT entry can
                    // still point into `.idata` (RVA), which is not executable. Handle them
                    // virtually by name instead of jumping to `addr`.
                    if self.cfg.emulate_winapi {
                        self.gateway_return = self.stack_pop64(false).unwrap_or(0);
                        self.regs_mut().rip = self.gateway_return;
                        winapi64::gateway_by_import(dll, api, self);
                        self.force_break = true;
                        self.is_api_run = true;
                        return true;
                    }

                    self.gateway_return = self.stack_pop64(false).unwrap_or(0);
                    self.regs_mut().rip = self.gateway_return;
                    winapi64::gateway(addr, "not_loaded", self);
                    self.force_break = true;
                    self.is_api_run = true;
                    return true;
                } else {
                    log::error!(
                        "/!\\ set_rip setting rip to non mapped addr 0x{:x} {}",
                        addr,
                        self.filename
                    );
                    self.exception(ExceptionType::SettingRipToNonMappedAddr);
                    return false;
                }
            }
        };

        let map_name = self.filename.as_str();

        /*
        if addr < constants::LIBS64_MIN
            || name == "code"
            || (!map_name.is_empty() && name.starts_with(&map_name))
            || name == "loader.text"*/
        if addr < constants::LIBS64_MIN {
            if self.cfg.verbose > 1 {
                let rip = self.regs().rip;
                let prev = self.maps.get_addr_name(rip).unwrap_or("??");
                if prev != name {
                    log::trace!("{}:0x{:x} map change  {} -> {}", self.pos, rip, prev, name);
                }
            }

            self.regs_mut().rip = addr;
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
            return true;
        } else if self.os.is_linux() || self.os.is_macos() {
            let section_name = name.to_string();
            return self.intercept_unix_x64_api_call(addr, &section_name);
        } else {
            if self.cfg.verbose >= 2 && !self.cfg.emulate_winapi {
                log::trace!("/!\\ changing RIP to {} ", name);
            }

            // SSDT mode: always run the real DLL machine code; user-level APIs
            // like `kernel32!LoadLibraryA` execute their full path down to the
            // `syscall` instruction (which the syscall handler intercepts).
            // No Rust winapi stubs are used here — the whole point of SSDT is
            // to emulate the real DLLs and only implement the kernel syscall
            // surface.
            if self.cfg.emulate_winapi {
                let api_name = winapi64::kernel32::guess_api_name(self, addr);
                if !api_name.is_empty()
                    && self.cfg.verbose >= 1 {
                        log_red!(self, "emulating {}", api_name);
                    }
                self.regs_mut().rip = addr;
                return true;
            }

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

            if handle_winapi {
                let name = self
                    .maps
                    .get_addr_name(addr)
                    .expect("/!\\ changing RIP to non mapped addr 0x");
                winapi64::gateway(addr, name.to_string().as_str(), self);
            }
            self.force_break = true;
        }

        true
    }

    /// Redirect execution flow for AArch64 branches.
    /// If the target address is in a loaded library (dylib/so), intercept and
    /// dispatch to the appropriate API handler. Mirrors set_rip() for Windows.
    pub fn set_pc_aarch64(&mut self, addr: u64) -> bool {
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
                if !api_name.is_empty()
                    && self.cfg.verbose >= 1 {
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
                let api_name = self.pe32.as_ref().unwrap().import_addr_to_name(addr as u32);
                if !api_name.is_empty() {
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
                    return true;
                } else {
                    log::error!("/!\\ setting eip to non mapped addr 0x{:x}", addr);
                    self.exception(ExceptionType::SettingRipToNonMappedAddr);
                    return false;
                }
            }
        };

        let map_name = self.filename_to_mapname(&self.filename);

        /*
        if name == "code"
            || addr < constants::LIBS32_MIN
            || (!map_name.is_empty() && name.starts_with(&map_name))
            || name == "loader.text"*/
        if addr < constants::LIBS32_MIN {
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
                if !api_name.is_empty()
                    && self.cfg.verbose >= 1 {
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
                let name = self.maps.get_addr_name(addr).unwrap();
                winapi32::gateway(to32!(addr), name.to_string().as_str(), self);
            }
            self.force_break = true;
        }

        true
    }
}
