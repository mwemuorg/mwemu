use ahash::AHashMap;
use minidump::format::MemoryProtection;
use minidump::*;
use slab::Slab;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::error::Error;
use std::ops::Deref;

use crate::arch::aarch64::regs::RegsAarch64;
use crate::arch::{Arch, OperatingSystem};
use crate::flags::Flags;
use crate::maps::Maps;
use crate::maps::mem64::{Mem64, Permission};
use crate::maps::tlb::TLB;
use crate::regs64::Regs64;
use crate::serialization::emu::SerializableEmu;
use crate::serialization::maps::SerializableMaps;
use crate::serialization::pe32::SerializablePE32;
use crate::serialization::pe64::SerializablePE64;

enum ExtractedContext {
    X86 { regs: Regs64, flags: Flags },
    AArch64 { regs: RegsAarch64 },
}

pub struct MinidumpReader;

impl MinidumpReader {
    fn get_pe_offset(data: &[u8]) -> Option<usize> {
        if data.len() < 0x3C + 4 {
            return None;
        }
        let offset = u32::from_le_bytes([data[0x3C], data[0x3D], data[0x3E], data[0x3F]]) as usize;
        if offset < data.len() {
            Some(offset)
        } else {
            None
        }
    }

    fn is_pe64(data: &[u8], pe_offset: usize) -> bool {
        if pe_offset + 24 >= data.len() {
            return false;
        }
        // Check machine type in PE header - 0x8664 = x64
        let machine = u16::from_le_bytes([data[pe_offset + 4], data[pe_offset + 5]]);
        machine == 0x8664 || machine == 0xAA64
    }

    fn extract_pe_modules<T: Deref<Target = [u8]>>(
        dump: &minidump::Minidump<'static, T>,
    ) -> Result<(Option<SerializablePE32>, Option<SerializablePE64>), Box<dyn Error>> {
        let mut pe32 = None;
        let mut pe64 = None;

        if let Ok(modules) = dump.get_stream::<MinidumpModuleList>() {
            let memory = dump.get_memory().unwrap_or_default();

            for module in modules.iter() {
                // Try to read the module from memory
                if let Some(mem_region) = memory.memory_at_address(module.base_address()) {
                    let raw_data = mem_region.bytes().to_vec();

                    // Basic PE detection - check for MZ header
                    if raw_data.len() > 0x40 && &raw_data[0..2] == b"MZ" {
                        // Read PE header to determine 32 vs 64 bit
                        if let Some(pe_offset) = Self::get_pe_offset(&raw_data) {
                            if Self::is_pe64(&raw_data, pe_offset) {
                                pe64 = Some(SerializablePE64 {
                                    filename: module.name.to_string(),
                                    raw: raw_data,
                                });
                            } else {
                                pe32 = Some(SerializablePE32 {
                                    filename: module.name.to_string(),
                                    raw: raw_data,
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok((pe32, pe64))
    }

    fn extract_memory_maps<T: Deref<Target = [u8]>>(
        dump: &minidump::Minidump<'static, T>,
    ) -> Result<SerializableMaps, Box<dyn Error>> {
        let mut mem_slab = Slab::new();
        let mut maps = BTreeMap::new();
        let mut name_map = AHashMap::new();
        let memory = dump.get_memory().unwrap_or_default();

        if let Ok(memory_info) = dump.get_stream::<MinidumpMemoryInfoList>() {
            for info in memory_info.iter() {
                let base_addr = info.raw.base_address;
                if info.raw.region_size == 0 {
                    continue;
                }

                let permission = match info.protection.bits() & MemoryProtection::ACCESS_MASK.bits()
                {
                    x if x == MemoryProtection::PAGE_NOACCESS.bits() => Permission::NONE,
                    x if x == MemoryProtection::PAGE_READONLY.bits() => Permission::READ,
                    x if x == MemoryProtection::PAGE_READWRITE.bits() => Permission::READ_WRITE,
                    x if x == MemoryProtection::PAGE_EXECUTE.bits() => Permission::EXECUTE,
                    x if x == MemoryProtection::PAGE_EXECUTE_READ.bits() => {
                        Permission::READ_EXECUTE
                    }
                    x if x == MemoryProtection::PAGE_EXECUTE_READWRITE.bits() => {
                        Permission::READ_WRITE_EXECUTE
                    }
                    _ => Permission::READ_WRITE_EXECUTE,
                };

                let Some(mem_region) = memory.memory_at_address(base_addr) else {
                    continue;
                };

                let mem_data = mem_region.bytes().to_vec();
                if mem_data.is_empty() {
                    continue;
                }

                let mem_entry = Mem64::new(
                    format!("mem_0x{:016x}", base_addr), // name
                    base_addr,                           // base_addr
                    base_addr + mem_data.len() as u64,   // bottom_addr (base + size)
                    mem_data,                            // mem data
                    permission,
                );

                let slab_key = mem_slab.insert(mem_entry);
                maps.insert(base_addr, slab_key);
            }
        } else {
            for mem_region in memory.by_addr() {
                if mem_region.bytes().is_empty() {
                    continue;
                }

                let base_addr = mem_region.base_address();
                let mem_data = mem_region.bytes().to_vec();
                let mem_entry = Mem64::new(
                    format!("mem_0x{:016x}", base_addr),
                    base_addr,
                    base_addr + mem_data.len() as u64,
                    mem_data,
                    Permission::READ_WRITE_EXECUTE,
                );
                let slab_key = mem_slab.insert(mem_entry);
                maps.insert(base_addr, slab_key);
            }
        }

        // Also add mapped modules to name_map
        if let Ok(modules) = dump.get_stream::<MinidumpModuleList>() {
            for module in modules.iter() {
                let module_base = module.base_address();

                // Find corresponding memory region that contains this module
                for (&addr, &slab_key) in &maps {
                    if addr <= module_base && module_base < addr + mem_slab[slab_key].size() as u64
                    {
                        name_map.insert(module.name.to_string(), slab_key);
                        break;
                    }
                }
            }
        }

        let system_info = dump.get_stream::<MinidumpSystemInfo>()?;
        let is_64bits = matches!(
            system_info.cpu,
            minidump::system_info::Cpu::X86_64 | minidump::system_info::Cpu::Arm64
        );

        let banzai = false;
        let tlb = RefCell::new(TLB::new());

        Ok(SerializableMaps::new(Maps::new(
            mem_slab, maps, name_map, is_64bits, banzai, tlb,
        )))
    }

    fn extract_thread_context<T: Deref<Target = [u8]>>(
        dump: &minidump::Minidump<'static, T>,
        system_info: &MinidumpSystemInfo,
    ) -> Result<ExtractedContext, Box<dyn Error>> {
        let threads = dump.get_stream::<MinidumpThreadList>()?;
        let thread = threads
            .threads
            .first()
            .ok_or("No threads found in minidump")?;
        let misc = dump.get_stream::<MinidumpMiscInfo>().ok();
        let context = thread
            .context(system_info, misc.as_ref())
            .ok_or("No thread context found in minidump")?;

        match &context.raw {
            MinidumpRawContext::X86(raw) => {
                let mut regs = Regs64::default();
                let mut flags = Flags::new();
                regs.dr0 = raw.dr0 as u64;
                regs.dr1 = raw.dr1 as u64;
                regs.dr2 = raw.dr2 as u64;
                regs.dr3 = raw.dr3 as u64;
                regs.dr6 = raw.dr6 as u64;
                regs.dr7 = raw.dr7 as u64;
                regs.set_eax(raw.eax as u64);
                regs.set_ebx(raw.ebx as u64);
                regs.set_ecx(raw.ecx as u64);
                regs.set_edx(raw.edx as u64);
                regs.set_esi(raw.esi as u64);
                regs.set_edi(raw.edi as u64);
                regs.set_ebp(raw.ebp as u64);
                regs.set_esp(raw.esp as u64);
                regs.set_eip(raw.eip as u64);
                regs.fs = raw.fs as u64;
                regs.gs = raw.gs as u64;
                flags.load(raw.eflags);
                Ok(ExtractedContext::X86 { regs, flags })
            }
            MinidumpRawContext::Amd64(raw) => {
                let mut regs = Regs64::default();
                let mut flags = Flags::new();
                regs.dr0 = raw.dr0;
                regs.dr1 = raw.dr1;
                regs.dr2 = raw.dr2;
                regs.dr3 = raw.dr3;
                regs.dr6 = raw.dr6;
                regs.dr7 = raw.dr7;
                regs.rax = raw.rax;
                regs.rbx = raw.rbx;
                regs.rcx = raw.rcx;
                regs.rdx = raw.rdx;
                regs.rsi = raw.rsi;
                regs.rdi = raw.rdi;
                regs.rbp = raw.rbp;
                regs.rsp = raw.rsp;
                regs.rip = raw.rip;
                regs.r8 = raw.r8;
                regs.r9 = raw.r9;
                regs.r10 = raw.r10;
                regs.r11 = raw.r11;
                regs.r12 = raw.r12;
                regs.r13 = raw.r13;
                regs.r14 = raw.r14;
                regs.r15 = raw.r15;
                regs.fs = raw.fs as u64;
                regs.gs = raw.gs as u64;
                flags.load(raw.eflags);
                Ok(ExtractedContext::X86 { regs, flags })
            }
            MinidumpRawContext::Arm64(raw) => {
                let mut regs = RegsAarch64::new();
                for i in 0..31 {
                    regs.x[i] = raw.iregs[i];
                }
                regs.sp = raw.sp;
                regs.pc = raw.pc;
                regs.nzcv.from_u64(raw.cpsr as u64);
                regs.fpcr = raw.fpcr as u64;
                regs.fpsr = raw.fpsr as u64;
                for i in 0..32 {
                    regs.v[i] = raw.float_regs[i];
                }
                Ok(ExtractedContext::AArch64 { regs })
            }
            MinidumpRawContext::OldArm64(raw) => {
                let mut regs = RegsAarch64::new();
                for i in 0..31 {
                    regs.x[i] = raw.iregs[i];
                }
                regs.sp = raw.sp;
                regs.pc = raw.pc;
                regs.nzcv.from_u64(raw.cpsr as u64);
                regs.fpcr = raw.fpcr as u64;
                regs.fpsr = raw.fpsr as u64;
                for i in 0..32 {
                    regs.v[i] = raw.float_regs[i];
                }
                Ok(ExtractedContext::AArch64 { regs })
            }
            _ => Err("Unsupported minidump CPU context".into()),
        }
    }

    pub fn from_minidump_file(path: &str) -> Result<SerializableEmu, Box<dyn Error>> {
        let dump = minidump::Minidump::read_path(path)?;

        // Get basic streams we need
        let system_info = dump.get_stream::<MinidumpSystemInfo>()?;

        let arch = match system_info.cpu {
            minidump::system_info::Cpu::X86 => Arch::X86,
            minidump::system_info::Cpu::X86_64 => Arch::X86_64,
            minidump::system_info::Cpu::Arm64 => Arch::Aarch64,
            _ => Arch::X86,
        };

        // Extract PE modules
        let (pe32, pe64) = Self::extract_pe_modules(&dump)?;

        // Extract memory maps
        let maps = Self::extract_memory_maps(&dump)?;

        // Extract thread context when present.
        let extracted = Self::extract_thread_context(&dump, &system_info)?;

        // Build serializable emu with the correct architecture
        let mut serializable_emu = SerializableEmu::default_for_arch(arch);
        serializable_emu.set_maps(maps);
        serializable_emu.set_pe32(pe32);
        serializable_emu.set_pe64(pe64);
        serializable_emu.cfg.arch = arch;

        match extracted {
            ExtractedContext::X86 { regs, flags } => {
                serializable_emu.set_regs(regs);
                serializable_emu.set_flags(flags);
            }
            ExtractedContext::AArch64 { regs } => {
                serializable_emu.set_regs_aarch64(regs);
            }
        }

        serializable_emu.os = match system_info.os {
            minidump::system_info::Os::Windows => OperatingSystem::Windows,
            minidump::system_info::Os::Linux => OperatingSystem::Linux,
            minidump::system_info::Os::MacOs => OperatingSystem::MacOS,
            _ => OperatingSystem::Windows,
        };
        if let Some(pe64) = &serializable_emu.pe64 {
            serializable_emu.filename = pe64.filename.clone();
        } else if let Some(pe32) = &serializable_emu.pe32 {
            serializable_emu.filename = pe32.filename.clone();
        }

        Ok(serializable_emu)
    }
}
