//! Loading a driver: the ET_REL path.
//!
//! A `.ko` is not a program. It has no entry point, no program headers and no
//! interpreter — it is an object file the kernel links into its own address
//! space. So loading one is a three-step job that has no counterpart in the
//! PE/ELF executable loaders:
//!
//! 1. resolve every import against the emulated kernel (a stub for a function,
//!    storage for a variable),
//! 2. place the `SHF_ALLOC` sections and apply the relocations,
//! 3. find the entry points the OS would call — `init_module`, `cleanup_module`
//!    and whatever file operations the module registered.
//!
//! Nothing executes as a side effect of loading. `init` is run explicitly by
//! [`Emu::run_module_init`], so an analyst can inspect the linked image, set
//! breakpoints, or drive an ioctl handler directly without ever running init.

use std::collections::HashMap;

use rs_header::elf::elf64::{EM_AARCH64, EM_X86_64, Elf64};

use crate::arch::Arch;
use crate::emu::Emu;
use crate::err::MwemuError;
use crate::kernel::KernelOs;

/// Offset of `name[]` inside `struct module` on 64-bit: `state` (4 + 4 pad)
/// followed by `struct list_head list` (16).
const MODULE_NAME_OFFSET: u64 = 24;

impl Emu {
    /// Load a Linux kernel module and link it against the emulated kernel.
    ///
    /// Returns the module's base address. The module's init function is *not*
    /// executed; call [`Emu::run_module_init`] for that.
    pub fn load_kernel_module(&mut self, filename: &str) -> Result<u64, MwemuError> {
        let raw = std::fs::read(filename)
            .map_err(|e| MwemuError::new(&format!("cannot read {}: {}", filename, e)))?;

        let mut elf = Elf64::parse(&raw)
            .map_err(|e| MwemuError::new(&format!("{} is not a valid ELF64: {}", filename, e)))?;
        if !elf.is_relocatable() {
            return Err(MwemuError::new(
                "not a kernel module: ELF type is not ET_REL",
            ));
        }

        match elf.elf_hdr.e_machine {
            EM_X86_64 => self.cfg.arch = Arch::X86_64,
            EM_AARCH64 => {
                self.cfg.arch = Arch::Aarch64;
                self.ensure_arch_state_aarch64();
            }
            other => {
                return Err(MwemuError::new(&format!(
                    "unsupported kernel module architecture e_machine=0x{:x}",
                    other
                )));
            }
        }
        self.maps.is_64bits = true;
        self.filename = filename.to_string();
        self.cfg.filename = filename.to_string();

        self.kernel_init(KernelOs::Linux);

        // --- resolve imports before touching memory --------------------------
        // The relocation pass needs `&mut Maps`, and resolving needs `&mut Emu`
        // to hand out stub and data addresses, so the two cannot be interleaved.
        // Resolving first is also more honest: the caller learns exactly which
        // kernel symbols this module needs, whether or not they are implemented.
        let symbols = elf
            .parse_symtab()
            .map_err(|e| MwemuError::new(&format!("cannot read module symbols: {}", e)))?;
        let mut imports: HashMap<String, u64> = HashMap::new();
        for sym in &symbols {
            if sym.st_shndx != 0 || sym.st_dynstr_name.is_empty() {
                continue;
            }
            if imports.contains_key(&sym.st_dynstr_name) {
                continue;
            }
            if let Some(addr) = self.kernel_resolve_import(&sym.st_dynstr_name) {
                imports.insert(sym.st_dynstr_name.clone(), addr);
            }
        }

        let base = self
            .kernel
            .as_ref()
            .expect("kernel env present")
            .layout
            .module_base;
        let stem = std::path::Path::new(filename)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "module".to_string());

        let obj = elf
            .load_relocatable(&mut self.maps, &stem, base, &mut |name| {
                imports.get(name).copied()
            })
            .map_err(|e| MwemuError::new(&format!("cannot link {}: {}", filename, e)))?;

        // --- record what was loaded ------------------------------------------
        let init = obj.symbol("init_module").or_else(|| {
            // A module built without the `module_init()` alias still has its
            // constructor as the only function in `.init.text`.
            obj.symbols
                .iter()
                .find(|s| {
                    s.is_func
                        && obj
                            .section(".init.text")
                            .is_some_and(|sec| s.addr >= sec.addr && s.addr < sec.addr + sec.size)
                })
                .map(|s| s.addr)
        });
        let exit = obj.symbol("cleanup_module").or_else(|| {
            obj.symbols
                .iter()
                .find(|s| {
                    s.is_func
                        && obj
                            .section(".exit.text")
                            .is_some_and(|sec| s.addr >= sec.addr && s.addr < sec.addr + sec.size)
                })
                .map(|s| s.addr)
        });

        // The module's own `struct module` carries its name, the same string
        // `lsmod` shows.
        let name = obj
            .section(".gnu.linkonce.this_module")
            .map(|sec| self.maps.read_string(sec.addr + MODULE_NAME_OFFSET))
            .filter(|n| !n.is_empty())
            .unwrap_or_else(|| stem.clone());

        log::info!(
            "loaded kernel module '{}' at 0x{:x} ({} bytes, {} sections, {} symbols)",
            name,
            obj.base,
            obj.size,
            obj.sections.len(),
            obj.symbols.len()
        );
        if !obj.unresolved.is_empty() {
            log::warn!(
                "module imports {} symbol(s) with no kernel implementation: {}",
                obj.unresolved.len(),
                obj.unresolved.join(", ")
            );
        }

        let module = crate::kernel::ModuleImage {
            name,
            base: obj.base,
            size: obj.size,
            init,
            exit,
            sections: obj.sections,
            symbols: obj.symbols,
            unresolved: obj.unresolved,
        };
        self.kernel.as_mut().expect("kernel env present").module = module;

        self.base = obj.base;
        self.elf64 = Some(elf);
        Ok(base)
    }

    /// Address of a symbol the loaded module defines — how a caller reaches a
    /// specific handler (an ioctl, a file operation, a work callback).
    pub fn module_symbol(&self, name: &str) -> Option<u64> {
        self.kernel.as_ref()?.module.symbol(name)
    }

    /// Run the module's init function, as `insmod` would. Returns its result
    /// (0 on success, a negative errno otherwise).
    pub fn run_module_init(&mut self) -> Result<u64, MwemuError> {
        let init = self
            .kernel
            .as_ref()
            .and_then(|k| k.module.init)
            .ok_or_else(|| MwemuError::new("module has no init function"))?;
        log::info!("calling module init at 0x{:x}", init);
        self.kernel_call(init, &[])
    }

    /// Run the module's exit function, as `rmmod` would.
    pub fn run_module_exit(&mut self) -> Result<u64, MwemuError> {
        let exit = self
            .kernel
            .as_ref()
            .and_then(|k| k.module.exit)
            .ok_or_else(|| MwemuError::new("module has no exit function"))?;
        log::info!("calling module exit at 0x{:x}", exit);
        self.kernel_call(exit, &[])
    }

    /// Call one of the module's own functions by symbol name.
    pub fn call_module_symbol(&mut self, name: &str, args: &[u64]) -> Result<u64, MwemuError> {
        let addr = self
            .module_symbol(name)
            .ok_or_else(|| MwemuError::new(&format!("module has no symbol '{}'", name)))?;
        self.kernel_call(addr, args)
    }
}
