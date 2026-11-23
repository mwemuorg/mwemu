//! Loading binaries into the emulator.
//!
//! [`Emu::load_code`] sniffs the input format and hands the file to the
//! loader for that format. Every format owns a module under `loaders/`, so
//! adding one is a new module plus a dispatch arm — the dispatcher itself
//! never has to change.

use rs_header::pe::{IMAGE_FILE_MACHINE_I386, pe_machine_type};

use crate::api::abi::windows::{X64_SHADOW_SPACE, X64_STACK_ALIGNMENT};
use crate::emu::Emu;
use crate::err::MwemuError;
use crate::maps::mem64::Permission;
use crate::windows::constants::process::DLL_PROCESS_ATTACH;

mod detect;
mod elf;
mod elf32;
mod format;
pub(crate) mod guest_dll;
mod kext;
mod ko;
mod ldr_init;
mod macho;
mod pe;
mod pe_process;
mod shellcode;
mod sys;
pub use pe::{MappedPe64, PeLoadInfo};
mod pe_stub;
pub(crate) mod pe_stub_exports;

use detect::BinaryFormat;
use format::{FormatLoader, LoadInput};

impl Emu {
    /// Execute initialization under the same API policy as the entry point.
    pub(crate) fn run_tls_callbacks(&mut self, base: u64) -> Result<(), MwemuError> {
        self.check_api_stop()?;
        for i in 0..self.tls_callbacks.len() {
            self.check_api_stop()?;
            let callback = self.tls_callbacks[i];
            let saved_sp = self.sp();
            log::trace!("emulating TLS callback {} at 0x{:x}", i + 1, callback);
            let frame_written = if self.cfg.arch.is_64bits() {
                let frame = (saved_sp & !(X64_STACK_ALIGNMENT - 1))
                    .checked_sub(X64_SHADOW_SPACE)
                    .ok_or_else(|| MwemuError::new("invalid TLS callback stack"))?;
                self.set_sp(frame);
                self.regs_mut().rcx = base;
                self.regs_mut().rdx = DLL_PROCESS_ATTACH as u64;
                self.regs_mut().r8 = 0;
                self.stack_push64(base)
            } else {
                [0, DLL_PROCESS_ATTACH, base as u32, base as u32]
                    .into_iter()
                    .all(|value| self.stack_push32(value))
            };
            if !frame_written {
                return Err(MwemuError::new("cannot write TLS callback frame"));
            }
            self.set_pc(callback);
            let returned = self.run(Some(base))?;
            self.check_api_stop()?;
            if self.process_terminated || returned != base {
                return Err(MwemuError::new(&format!(
                    "TLS callback {} at 0x{:x} did not return",
                    i + 1,
                    callback
                )));
            }
            self.set_sp(saved_sp);
        }
        Ok(())
    }

    /// Classify a target the way `load_code` will, reporting whether it is a
    /// non-Windows guest (ELF or Mach-O) that needs no Windows system DLLs.
    /// Raw files have no OS marker and default to Windows unless the caller selects an OS.
    /// `force_shellcode` skips header detection.
    pub fn is_non_windows_file(filename: &str, force_shellcode: bool) -> bool {
        if force_shellcode {
            return false;
        }
        let raw = std::fs::read(filename).unwrap_or_default();
        BinaryFormat::detect(&raw, filename, false).is_non_windows()
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

    /// Load a binary file (PE, ELF, Mach-O, or raw shellcode) into the
    /// emulator.
    ///
    /// The format is auto-detected from the file header and handed to the
    /// loader for that format. For raw files, choose the architecture at construction
    /// and call `set_os` before loading a non-Windows guest.
    pub fn load_code(&mut self, filename: &str) -> Result<(), MwemuError> {
        self.filename = filename.to_string();
        self.cfg.filename = self.filename.clone();

        // A missing/unreadable file must fail loudly here: swallowing the
        // error into empty bytes gets it misdetected as shellcode and panics
        // later creating the code map. Report clearly and bail instead.
        let bytes = match std::fs::read(filename) {
            Ok(bytes) => bytes,
            Err(e) => {
                log::error!("cannot read '{}': {}", filename, e);
                eprintln!(
                    "[mwemu] cannot open '{}': {} — check the -f path",
                    filename, e
                );
                return Err(MwemuError::new(&format!(
                    "cannot read '{}': {}",
                    filename, e
                )));
            }
        };

        let format = BinaryFormat::detect(&bytes, filename, self.cfg.shellcode);
        let input = LoadInput {
            path: filename,
            bytes: &bytes,
        };
        dispatch(format, self, &input)
    }

    /// Load a shellcode from a variable.
    /// This assumes that there is no headers like PE/ELF and it's direclty code.
    /// Any OS simulation is triggered, but init() could be called by the user
    pub fn load_code_bytes(&mut self, bytes: &[u8]) -> Result<(), MwemuError> {
        if self.cfg.verbose >= 1 {
            log::trace!("Loading shellcode from bytes");
        }

        self.init_cpu();

        let code = match self.maps.create_map(
            "code",
            self.cfg.code_base_addr,
            bytes.len() as u64,
            Permission::READ_WRITE_EXECUTE,
        ) {
            Ok(m) => m,
            Err(_) => {
                log::error!("cannot create code map for load_code_bytes");
                return Err(MwemuError::new(
                    "cannot create code map for load_code_bytes",
                ));
            }
        };
        let base = code.get_base();
        code.write_bytes(base, bytes);
        self.set_pc(base);
        Ok(())
    }
}

/// Hand the file to the loader for its format.
fn dispatch(format: BinaryFormat, emu: &mut Emu, input: &LoadInput<'_>) -> Result<(), MwemuError> {
    match format {
        BinaryFormat::Elf32 => elf32::Elf32.load(emu, input),
        BinaryFormat::Elf64X64 => elf::Elf64X64.load(emu, input),
        BinaryFormat::Elf64Aarch64 => elf::Elf64Aarch64.load(emu, input),
        BinaryFormat::Elf64Relocatable => ko::Elf64Relocatable.load(emu, input),
        BinaryFormat::MachoKext => macho::MachoKext.load(emu, input),
        BinaryFormat::MachoAarch64 => macho::MachoAarch64.load(emu, input),
        BinaryFormat::MachoX64 => macho::MachoX64.load(emu, input),
        BinaryFormat::Pe32 => pe_process::Pe32.load(emu, input),
        BinaryFormat::Pe64X64 => pe_process::Pe64X64.load(emu, input),
        BinaryFormat::Pe64Arm64 => pe_process::Pe64Arm64.load(emu, input),
        BinaryFormat::PeSysDriver => sys::PeSysDriver.load(emu, input),
        BinaryFormat::Shellcode => shellcode::Shellcode.load(emu, input),
    }
}
