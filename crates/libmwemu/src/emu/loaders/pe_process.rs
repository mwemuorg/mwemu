//! Windows PE process images: PE32 (x86), PE32+ (x86_64) and PE32+ (ARM64).
//!
//! These are the loaders for a PE that runs as a user-mode process or DLL.
//! They set up the Windows environment, map the image, and hand control to
//! the entry point the way the Windows loader would — including the
//! `DllMain` calling convention and the `ntdll!LdrInitializeThunk`
//! bootstrap.

use iced_x86::Register;
use rs_header::pe::{IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_ARM64, IMAGE_FILE_MACHINE_I386};

use crate::arch::{Arch, OperatingSystem};
use crate::emu::Emu;
use crate::err::MwemuError;
use crate::winapi::winapi64;

use super::format::{FormatLoader, LoadInput, apply_entry_point_override, set_maps_folder};

/// Loader for a Windows PE32 (x86) process image or DLL.
pub(crate) struct Pe32;

impl FormatLoader for Pe32 {
    fn load(&self, emu: &mut Emu, input: &LoadInput<'_>) -> Result<(), MwemuError> {
        log::trace!(
            "PE32 x86 header detected (Machine=0x{:04x}).",
            IMAGE_FILE_MACHINE_I386
        );
        emu.cfg.arch = Arch::X86;
        emu.os = OperatingSystem::Windows;
        set_maps_folder(emu, "windows", "x86", false);

        // TODO: the register/flag reset should follow the caller's request
        // instead of being hardcoded off.
        emu.init_windows_environment(false, false)?;
        // Claim the image before the PEB, loader lists and command line choose addresses.
        let mapped = emu.map_pe32_image(input.path, true, 0)?;
        emu.init_win32_process_maps();
        let base = emu.bind_pe32_image(input.path, true, mapped).base as u32;
        let image = emu
            .pe32
            .as_ref()
            .ok_or_else(|| MwemuError::new("missing PE32 process image"))?;
        let is_dll = image.is_dll();
        let (reserve, commit) = if is_dll {
            (0, 0)
        } else {
            (
                image.opt.size_of_stack_reserve as u64,
                image.opt.size_of_stack_commit as u64,
            )
        };
        emu.init_pe_stack32(reserve, commit)?;
        let ep = emu.regs().rip;
        emu.run_tls_callbacks(base as u64)?;

        if !is_dll {
            let peb = emu
                .maps
                .get_map_by_name("peb")
                .ok_or_else(|| MwemuError::new("missing PEB at x86 process entry"))?
                .get_base();
            emu.regs_mut().set_ebx(peb);
        }
        emu.regs_mut().rip = pe32_entry_point(emu, base, ep);
        apply_entry_point_override(emu);
        Ok(())
    }
}

/// The address a PE32 image starts at, and the stack it expects.
///
/// For a DLL the OS calls `DllMain` with stdcall:
///   `BOOL __stdcall DllMain(HINSTANCE hinstDLL, DWORD fdwReason, LPVOID lpvReserved);`
/// so the stack must contain a return address followed by the three
/// arguments. We push the arguments in *reverse* order (caller-convention
/// right-to-left) so the callee sees them at the expected offsets after it
/// pops the return address with `ret 12`.
///
/// Reflective-loader payloads (Cobalt Strike beacons, Meterpreter, …) ship
/// with an encrypted body and a cleartext `ReflectiveLoader` export. The PE
/// entry point lands inside the still-encrypted body, so we must start at
/// that export instead: it resolves its own imports, maps/decrypts the image
/// and finally calls the real `DllMain`. Its signature is stdcall
/// `ReflectiveLoader(LPVOID)`.
fn pe32_entry_point(emu: &mut Emu, base: u32, entry: u64) -> u64 {
    let reflective = emu.export_indexes.resolve_reflective_loader();

    // Read what the fix-up needs from the parsed image first: its borrow has
    // to end before the stack is written.
    let (is_dll, flat_start) = match emu.pe32.as_ref() {
        Some(pe32) => {
            let flat_start = match (reflective.as_ref(), emu.pe32_flat_loaded) {
                (Some((_, addr)), true) => {
                    let rva = (*addr - base as u64) as u32;
                    let foff = pe32
                        .sect_hdr
                        .iter()
                        .find(|s| {
                            rva >= s.virtual_address && rva < s.virtual_address + s.virtual_size
                        })
                        .map(|s| rva - s.virtual_address + s.pointer_to_raw_data)
                        .unwrap_or(rva);
                    Some(base as u64 + foff as u64)
                }
                _ => None,
            };
            (pe32.is_dll(), flat_start)
        }
        None => {
            log::error!("No Pe32 found inside self");
            return entry;
        }
    };

    if let Some((module, rl_addr)) = reflective {
        emu.cfg.soft_exception_recovery = true;
        let rl_ep = flat_start.unwrap_or(rl_addr);
        log::trace!(
            "emulating ReflectiveLoader of {} at 0x{:x} (base=0x{:x})",
            module,
            rl_ep,
            base
        );
        emu.stack_push32(0); // lpParameter
        emu.stack_push32(base); // fake return address
        return rl_ep;
    }

    if is_dll {
        log::trace!("emulating DllMain x86 base=0x{:x}", base);
        emu.stack_push32(0); // lpvReserved
        emu.stack_push32(1); // fdwReason = DLL_PROCESS_ATTACH
        emu.stack_push32(base); // hinstDLL
        // fake return address so the entry's `ret` doesn't crash
        // before DllMain executes its prolog (base is mapped RW).
        emu.stack_push32(base);
    }
    entry
}

/// Loader for a Windows PE32+ (x86_64) process image or DLL.
pub(crate) struct Pe64X64;

impl FormatLoader for Pe64X64 {
    fn load(&self, emu: &mut Emu, input: &LoadInput<'_>) -> Result<(), MwemuError> {
        log::trace!(
            "PE64 x86_64 header detected (Machine=0x{:04x}).",
            IMAGE_FILE_MACHINE_AMD64
        );
        emu.cfg.arch = Arch::X86_64;
        emu.os = OperatingSystem::Windows;
        set_maps_folder(emu, "windows", "x86_64", false);

        // TODO: the register/flag reset should follow the caller's request
        // instead of being hardcoded off.
        emu.init_win32(false, false)?;
        let base = emu.load_pe64(input.path, true, 0)?.base;
        let ep = emu.regs().rip;

        // A DLL starts in DllMain, which the OS calls with the module handle,
        // the attach reason and a reserved pointer.
        match emu.pe64.as_ref().map(|pe64| pe64.is_dll()) {
            Some(true) => {
                emu.regs_mut().set_reg(Register::RCX, base);
                emu.regs_mut().set_reg(Register::RDX, 1);
                emu.regs_mut().set_reg(Register::R8L, 0);
            }
            None => log::error!("No Pe64 found inside self"),
            Some(false) => {}
        }

        // Optional loader bootstrap: call ntdll!LdrInitializeThunk to perform
        // the loader init a real Windows process gets.
        if emu.cfg.emulate_winapi {
            emu.bootstrap_loader_init(ep)?;
        }
        emu.run_tls_callbacks(base)?;

        // If LdrInitializeThunk bailed via NtTerminateProcess, do not
        // continue with the EXE entry point: ntdll's loader state is
        // partially initialised and any further execution will crash
        // somewhere unrelated. Leave RIP at the syscall site so the
        // operator sees the actual termination point.
        if emu.process_terminated {
            log::error!(
                "ntdll!LdrInitializeThunk terminated the process during init. \
                 Skipping EXE entry. Last rip=0x{:x}",
                emu.regs().rip,
            );
        } else {
            // msvcrt's DllMain normally initialises _wcmdln and _acmdln
            // by calling GetCommandLine{W,A}. mwemu does not execute
            // DllMain for system DLLs, so populate these data exports
            // here so CRT startup code that reads them finds a valid
            // command line.
            emu.init_msvcrt_cmdln_globals();

            emu.regs_mut().rip = ep;
        }

        apply_entry_point_override(emu);
        Ok(())
    }
}

/// Loader for a Windows PE32+ (ARM64) process image or DLL.
pub(crate) struct Pe64Arm64;

impl FormatLoader for Pe64Arm64 {
    fn load(&self, emu: &mut Emu, input: &LoadInput<'_>) -> Result<(), MwemuError> {
        log::trace!(
            "PE64 ARM64 header detected (Machine=0x{:04x}). Windows AArch64 PE recognized.",
            IMAGE_FILE_MACHINE_ARM64
        );
        emu.cfg.arch = Arch::Aarch64;
        emu.os = OperatingSystem::Windows;
        emu.maps.is_64bits = true;
        set_maps_folder(emu, "windows", "aarch64", false);

        emu.init_win32(false, false)?;
        let base = emu.load_pe64(input.path, true, 0)?.base;
        let ep = emu.pc();

        // A DLL starts in DllMain, which the OS calls with the module handle,
        // the attach reason and a reserved pointer.
        match emu.pe64.as_ref().map(|pe64| pe64.is_dll()) {
            Some(true) => {
                let regs = emu.regs_aarch64_mut();
                regs.x[0] = base; // hinstDLL
                regs.x[1] = 1; // fdwReason = DLL_PROCESS_ATTACH
                regs.x[2] = 0; // lpvReserved
            }
            None => log::error!("No Pe64 found inside self"),
            Some(false) => {}
        }

        emu.set_pc(ep);
        apply_entry_point_override(emu);
        Ok(())
    }
}

impl Emu {
    /// Populate msvcrt's `_wcmdln`/`_acmdln` data exports with the emulated
    /// command line.
    fn init_msvcrt_cmdln_globals(&mut self) {
        let wcmdln_addr =
            winapi64::kernel32::resolve_api_name_in_module(self, "msvcrt.dll", "_wcmdln");
        let acmdln_addr =
            winapi64::kernel32::resolve_api_name_in_module(self, "msvcrt.dll", "_acmdln");

        if wcmdln_addr == 0 && acmdln_addr == 0 {
            return;
        }

        let exe_name = self.cfg.exe_name.clone();

        if wcmdln_addr != 0 {
            let Some(buf) = self.heap_mut().allocate(2048) else {
                log::error!("cannot allocate _wcmdln buffer");
                return;
            };
            self.maps.write_wide_string(buf, &exe_name);
            self.maps.write_qword(wcmdln_addr, buf);
            log::trace!(
                "msvcrt!_wcmdln at 0x{:x} → 0x{:x} (L\"{}\")",
                wcmdln_addr,
                buf,
                exe_name
            );
        }

        if acmdln_addr != 0 {
            let Some(buf) = self.heap_mut().allocate(1024) else {
                log::error!("cannot allocate _acmdln buffer");
                return;
            };
            self.maps.write_string(buf, &exe_name);
            self.maps.write_qword(acmdln_addr, buf);
            log::trace!(
                "msvcrt!_acmdln at 0x{:x} → 0x{:x} (\"{}\")",
                acmdln_addr,
                buf,
                exe_name
            );
        }
    }
}
