use iced_x86::{Instruction, MemorySize, OpKind, Register};

use crate::maps::mem64::Permission;
use crate::{
    console::Console,
    emu::Emu,
    exception::types::ExceptionType,
    regs64,
    windows::constants,
    windows::structures::{self, MemoryOperation},
};

impl Emu {
    /// Decode the jump parameter
    pub fn get_jump_value(&mut self, ins: &Instruction, noperand: u32) -> Option<u64> {
        match ins.op_kind(noperand) {
            OpKind::NearBranch64 | OpKind::NearBranch32 | OpKind::NearBranch16 => {
                Some(ins.near_branch_target())
            }
            OpKind::FarBranch16 => Some(ins.far_branch16() as u64),
            OpKind::FarBranch32 => Some(ins.far_branch32() as u64),
            _ => self.get_operand_value(ins, 0, true),
        }
    }

    /// Instruction argument decoder.
    fn handle_memory_get_operand(
        &mut self,
        ins: &Instruction,
        noperand: u32,
        do_derref: bool,
    ) -> Option<u64> {
        let mem_seg = ins.memory_segment();
        let fs = mem_seg == Register::FS;
        let gs = mem_seg == Register::GS;
        let derref = if mem_seg == Register::FS || mem_seg == Register::GS {
            false
        } else {
            do_derref
        };
        let mem_base = ins.memory_base();
        let mem_index = ins.memory_index();

        /*if self.cfg.verbose >= 3 {
            log::debug!("handle_memory_get_operand: mem_seg={:?}, mem_base={:?}, mem_index={:?}, do_derref={}",
                       mem_seg, mem_base, mem_index, do_derref);
        }*/

        let mem_displace = if self.cfg.is_x64() {
            ins.memory_displacement64()
        } else {
            ins.memory_displacement32() as i32 as u64 // we need this for signed extension from 32bit to 64bi
        };

        /*if self.cfg.verbose >= 3 {
            log::debug!("  mem_displace=0x{:x} (is_64bits={})", mem_displace, self.cfg.is_x64());
        }*/

        let temp_displace = if mem_index == Register::None {
            mem_displace
        } else {
            let scale_index = ins.memory_index_scale();
            let index_val = self.regs().get_reg(mem_index);
            let scale_factor = index_val.wrapping_mul(scale_index as u64);
            let result = mem_displace.wrapping_add(scale_factor);
            /*if self.cfg.verbose >= 3 {
                log::debug!("  scale_index={}, index_val=0x{:x}, scale_factor=0x{:x}, temp_displace=0x{:x}",
                           scale_index, index_val, scale_factor, result);
            }*/
            result
        };

        // case when address is relative to rip then just return temp_displace
        let displace = if mem_base == Register::None {
            /*if self.cfg.verbose >= 3 {
                log::debug!("  mem_base is None, displace=temp_displace=0x{:x}", temp_displace);
            }*/
            temp_displace
        } else {
            let base_val = self.regs().get_reg(mem_base);
            let result = base_val.wrapping_add(temp_displace);
            /*if self.cfg.verbose >= 3 {
                log::debug!("  base_val=0x{:x}, displace=base+temp_displace=0x{:x}", base_val, result);
            }*/
            result
        };

        let displace_result = if !self.cfg.is_x64() {
            let masked = displace & 0xffffffff;
            /*if self.cfg.verbose >= 3 {
                log::debug!("  32-bit mode: displace_result=0x{:x} (masked from 0x{:x})", masked, displace);
            }*/
            masked
        } else {
            /*if self.cfg.verbose >= 3 {
                log::debug!("  64-bit mode: displace_result=0x{:x}", displace);
            }*/
            displace
        };

        // do this for cmov optimization
        let mem_addr = if mem_base == Register::RIP {
            /*if self.cfg.verbose >= 3 {
                log::debug!("  RIP-relative: mem_addr=temp_displace=0x{:x}", temp_displace);
            }*/
            temp_displace
        } else {
            /*if self.cfg.verbose >= 3 {
                log::debug!("  mem_addr=displace_result=0x{:x}", displace_result);
            }*/
            displace_result
        };

        if fs {
            if self.os.is_linux() {
                // Real TLS: once ld.so/libc install a thread pointer via
                // arch_prctl(ARCH_SET_FS), `fs:[disp]` addresses real memory at
                // fs_base+disp (the TCB, whose first qword is the self pointer,
                // the stack canary at +0x28, errno, etc.).
                let fs_base = self.regs().fs;
                if fs_base != 0 {
                    let real = fs_base.wrapping_add(mem_addr);
                    let v = self.maps.read_qword(real).unwrap_or(0);
                    return Some(v);
                }
                return if let Some(val) = self.fs().get(&mem_addr) {
                    if self.cfg.verbose > 0 {
                        log::trace!("reading FS[0x{:x}] -> 0x{:x}", mem_addr, *val);
                    }
                    if *val == 0 {
                        return Some(0); //0x7ffff7ff000);
                    }
                    Some(*val)
                } else {
                    if self.cfg.verbose > 0 {
                        log::trace!("reading FS[0x{:x}] -> 0", mem_addr);
                    }
                    Some(0) //0x7ffff7fff000);
                };
            }

            let value1: u64 = match mem_addr {
                0xc0 => {
                    if self.cfg.verbose >= 1 {
                        log::trace!("{} Reading ISWOW64 is 32bits on a 64bits system?", self.pos);
                    }
                    if self.cfg.is_x64() { 0 } else { 1 }
                }
                0x14 => {
                    let teb = self.maps.get_mem("teb");
                    let tib = teb.get_base(); // tib is first element.
                    if self.cfg.verbose >= 1 {
                        log::trace!("{} Reading NtTIB 0x{:x}", self.pos, tib);
                    }
                    tib
                }
                0x30 => {
                    let peb = self.maps.get_mem("peb");
                    if self.cfg.verbose >= 3 {
                        log::trace!("{} Reading PEB 0x{:x}", self.pos, peb.get_base());
                    }
                    peb.get_base()
                }
                0x20 => {
                    if self.cfg.verbose >= 1 {
                        log::trace!("{} Reading PID 0x{:x}", self.pos, 10);
                    }
                    10
                }
                0x24 => {
                    if self.cfg.verbose >= 1 {
                        log::trace!("{} Reading TID 0x{:x}", self.pos, 101);
                    }
                    101
                }
                0x34 => {
                    if self.cfg.verbose >= 1 {
                        log::trace!("{} Reading last error value 0", self.pos);
                    }
                    0
                }
                0x18 => {
                    let teb = self.maps.get_mem("teb");
                    if self.cfg.verbose >= 3 {
                        log::trace!("{} Reading TEB 0x{:x}", self.pos, teb.get_base());
                    }
                    teb.get_base()
                }
                0x00 => {
                    if self.cfg.verbose >= 1 {
                        log::trace!("Reading SEH 0x{:x}", self.seh());
                    }
                    self.seh()
                }
                0x28 => {
                    // TODO  linux TCB
                    0
                }
                0x2c => {
                    if self.cfg.verbose >= 1 {
                        log::trace!("Reading local ");
                    }
                    let locale = self.alloc("locale", 100, Permission::READ_WRITE);
                    self.maps.write_dword(locale, constants::EN_US_LOCALE);
                    //TODO: return a table of locales
                    /*
                    13071 0x41026e: mov   eax,[edx+eax*4]
                    =>r edx
                        edx: 0xc8 200 (locale)
                    =>r eax
                        eax: 0x409 1033
                    */

                    locale
                }
                _ => {
                    log::trace!("unimplemented fs:[{}]", mem_addr);
                    return None;
                }
            };
            return Some(value1);
        }

        if gs {
            let value1: u64 = match mem_addr {
                0x0 => {
                    // NtTib.ExceptionList — NULL in normal user threads (`cmp` / probes often use gs:[0]).
                    if self.cfg.verbose >= 1 {
                        log::trace!("{} gs:[0] ExceptionList -> 0", self.pos);
                    }
                    0
                }
                0x2 => {
                    // TEB+2: within the exception-list pointer bytes; typically 0 (see ntdll probes).
                    if self.cfg.verbose >= 1 {
                        log::trace!("{} gs:[2] TEB byte -> 0", self.pos);
                    }
                    0
                }
                0x60 => {
                    let peb = self.maps.get_mem("peb");
                    if self.cfg.verbose >= 3 {
                        log::trace!("{} Reading PEB 0x{:x}", self.pos, peb.get_base());
                    }
                    peb.get_base()
                }
                0x30 => {
                    let teb = self.maps.get_mem("teb");
                    if self.cfg.verbose >= 3 {
                        log::trace!("{} Reading TEB 0x{:x}", self.pos, teb.get_base());
                    }
                    teb.get_base()
                }
                0x40 => {
                    if self.cfg.verbose >= 1 {
                        log::trace!("{} Reading PID 0x{:x}", self.pos, 10);
                    }
                    10
                }
                0x48 => {
                    if self.cfg.verbose >= 1 {
                        log::trace!("{} Reading TID 0x{:x}", self.pos, 101);
                    }
                    101
                }
                0x10 => {
                    let stack = self.maps.get_mem("stack");
                    if self.cfg.verbose >= 1 {
                        log::trace!("{} Reading StackLimit 0x{:x}", self.pos, &stack.size());
                    }
                    stack.size() as u64
                }
                0x14 => {
                    unimplemented!("GS:[14]  get stack canary")
                }
                0x1488 => {
                    if self.cfg.verbose >= 1 {
                        log::trace!("Reading SEH 0x{:x}", self.seh());
                    }
                    self.seh()
                }
                0x8 => {
                    if self.cfg.verbose >= 1 {
                        log::trace!("Reading SEH 0x{:x}", self.seh());
                    }
                    if self.cfg.is_x64() {
                        self.maps.get_mem("peb").get_base()
                    } else {
                        let teb = self.maps.get_mem("teb");
                        let teb_struct = structures::TEB::new(teb.get_base() as u32);
                        teb_struct.thread_id as u64
                    }
                }
                0x58 => {
                    // Get or create static TLS array (for __declspec(thread) variables)
                    

                    match self.maps.get_mem2("static_tls_array") {
                        Some(mem) => mem.get_base(),
                        None => {
                            // This should be sized based on the number of modules with .tls sections
                            // For now, allocate space for a few module entries
                            let size = if self.cfg.is_x64() { 16 * 8 } else { 16 * 4 };
                            let tls_array =
                                self.alloc("static_tls_array", size, Permission::READ_WRITE);

                            // Initialize to null pointers
                            self.maps.write_bytes(tls_array, &vec![0; size as usize]);

                            tls_array
                        }
                    }
                }
                _ => {
                    // Fall back to reading from actual TEB memory so that real ntdll code
                    // can access arbitrary TEB fields without explicit special-casing.
                    let teb_base = self.maps.get_mem("teb").get_base();
                    let teb_addr = teb_base.wrapping_add(mem_addr);
                    let sz = self.get_operand_sz(ins, noperand);
                    let v = match sz {
                        8 => self.maps.read_byte(teb_addr).map(|x| x as u64).unwrap_or(0),
                        16 => self.maps.read_word(teb_addr).map(|x| x as u64).unwrap_or(0),
                        32 => self
                            .maps
                            .read_dword(teb_addr)
                            .map(|x| x as u64)
                            .unwrap_or(0),
                        _ => self.maps.read_qword(teb_addr).unwrap_or(0),
                    };
                    if self.cfg.verbose >= 1 {
                        log::trace!("gs:[0x{:x}] -> 0x{:x}  (teb+0x{:x})", mem_addr, v, mem_addr);
                    }
                    return Some(v);
                }
            };
            return Some(value1);
        }

        let value: u64;
        if derref {
            let sz = self.get_operand_sz(ins, noperand);
            /*if self.cfg.verbose >= 3 {
                log::debug!("  Dereferencing: mem_addr=0x{:x}, size={} bits", mem_addr, sz);
            }*/

            if let Some(mut hook_fn) = self.hooks.hook_on_memory_read.take() {
                let rip = self.regs().rip;
                hook_fn(self, rip, mem_addr, sz);
                self.hooks.hook_on_memory_read = Some(hook_fn);
            }

            value = match sz {
                64 => match self.maps.read_qword(mem_addr) {
                    Some(v) => v,
                    None => {
                        if self.try_grow_stack(mem_addr) {
                            self.maps.read_qword(mem_addr).unwrap_or(0)
                        } else {
                            log::trace!("/!\\ error dereferencing qword on 0x{:x}", mem_addr);
                            self.exception(ExceptionType::QWordDereferencing);
                            return None;
                        }
                    }
                },

                32 => match self.maps.read_dword(mem_addr) {
                    Some(v) => v.into(),
                    None => {
                        if self.try_grow_stack(mem_addr) {
                            self.maps.read_dword(mem_addr).unwrap_or(0) as u64
                        } else {
                            log::trace!("/!\\ error dereferencing dword on 0x{:x}", mem_addr);
                            self.exception(ExceptionType::DWordDereferencing);
                            return None;
                        }
                    }
                },

                16 => match self.maps.read_word(mem_addr) {
                    Some(v) => v.into(),
                    None => {
                        if self.try_grow_stack(mem_addr) {
                            self.maps.read_word(mem_addr).unwrap_or(0) as u64
                        } else {
                            log::trace!("/!\\ error dereferencing word on 0x{:x}", mem_addr);
                            self.exception(ExceptionType::WordDereferencing);
                            return None;
                        }
                    }
                },

                8 => match self.maps.read_byte(mem_addr) {
                    Some(v) => v.into(),
                    None => {
                        if self.try_grow_stack(mem_addr) {
                            self.maps.read_byte(mem_addr).unwrap_or(0) as u64
                        } else {
                            log::trace!("/!\\ error dereferencing byte on 0x{:x}", mem_addr);
                            self.exception(ExceptionType::ByteDereferencing);
                            return None;
                        }
                    }
                },

                _ => unimplemented!("weird size"),
            };

            if self.cfg.trace_mem {
                let name = self
                    .maps
                    .get_addr_name(mem_addr)
                    .unwrap_or("not mapped");
                let memory_operation = MemoryOperation {
                    pos: self.pos,
                    rip: self.regs().rip,
                    op: "read".to_string(),
                    bits: sz,
                    address: mem_addr,
                    old_value: 0, // not needed for read?
                    new_value: value,
                    name: name.to_string(),
                };
                self.memory_operations.push(memory_operation);
                log::trace!(
                    "\tmem_trace: pos = {} rip = {:x} op = read bits = {} address = 0x{:x} value = 0x{:x} name = '{}'",
                    self.pos,
                    self.regs().rip,
                    sz,
                    mem_addr,
                    value,
                    name
                );
            }

            if self.bp.is_bp_mem_read(mem_addr) {
                log::trace!("Memory breakpoint on read 0x{:x}", mem_addr);
                if self.running_script {
                    self.force_break = true;
                } else {
                    Console::spawn_console(self);
                }
            }
        } else {
            /*if self.cfg.verbose >= 3 {
                log::debug!("  Not dereferencing, returning mem_addr=0x{:x}", mem_addr);
            }*/
            value = mem_addr;
        }
        /*if self.cfg.verbose >= 3 {
            log::debug!("  Final return value: 0x{:x}", value);
        }*/
        Some(value)
    }

    /// Decode a selected operand and return its value (inmediate, register or memory)
    /// noperand: is (from 0 to n) and return
    /// do_derref: instructions like lea use memory, get the ref but dont derreference.
    pub fn get_operand_value(
        &mut self,
        ins: &Instruction,
        noperand: u32,
        do_derref: bool,
    ) -> Option<u64> {
        assert!(ins.op_count() > noperand);

        let value: u64 = match ins.op_kind(noperand) {
            OpKind::Immediate64 => ins.immediate64(),
            OpKind::Immediate8 => ins.immediate8() as u64,
            OpKind::Immediate16 => ins.immediate16() as u64,
            OpKind::Immediate32 => ins.immediate32() as u64,
            OpKind::Immediate8to64 => ins.immediate8to64() as u64,
            OpKind::Immediate32to64 => ins.immediate32to64() as u64,
            OpKind::Immediate8to32 => ins.immediate8to32() as u32 as u64,
            OpKind::Immediate8to16 => ins.immediate8to16() as u16 as u64,
            OpKind::Register => self.regs().get_reg(ins.op_register(noperand)),
            OpKind::Memory => match self.handle_memory_get_operand(ins, noperand, do_derref) {
                Some(v) => v,
                None => {
                    log::trace!(
                        "get_operand_value: unmapped memory access in {:?} op {}",
                        ins.mnemonic(),
                        noperand
                    );
                    return None;
                }
            },
            _ => unimplemented!("unimplemented operand type {:?}", ins.op_kind(noperand)),
        };
        Some(value)
    }

    /// Set a value to an operand, normally noperand=0
    /// If it's a register modify the register, it can be memory also.
    pub fn set_operand_value(&mut self, ins: &Instruction, noperand: u32, value: u64) -> bool {
        assert!(ins.op_count() > noperand);

        match ins.op_kind(noperand) {
            OpKind::Register => {
                if self.regs().is_fpu(ins.op_register(noperand)) {
                    self.fpu_mut()
                        .set_streg(ins.op_register(noperand), value as f64);
                } else {
                    self.regs_mut().set_reg(ins.op_register(noperand), value);
                }
            }

            OpKind::Memory => {
                let mem_base = ins.memory_base();
                let mem_index = ins.memory_index();
                let mem_displace = if self.cfg.is_x64() {
                    ins.memory_displacement64()
                } else {
                    ins.memory_displacement32() as i32 as u64 // we need this for signed extension from 32bit to 64bi
                };

                let mem_seg = ins.memory_segment();

                /*if self.cfg.verbose >= 3 {
                    log::debug!("set_operand_value Memory: mem_seg={:?}, mem_base={:?}, mem_index={:?}",
                               mem_seg, mem_base, mem_index);
                    log::debug!("  mem_displace=0x{:x}", mem_displace);
                }*/

                let temp_displace = if mem_index == Register::None {
                    mem_displace
                } else {
                    let scale_index = ins.memory_index_scale();
                    let index_val = self.regs().get_reg(mem_index);
                    let scale_factor = index_val.wrapping_mul(scale_index as u64);
                    let result = mem_displace.wrapping_add(scale_factor);
                    /*if self.cfg.verbose >= 3 {
                        log::debug!("  scale_index={}, index_val=0x{:x}, scale_factor=0x{:x}, temp_displace=0x{:x}",
                                   scale_index, index_val, scale_factor, result);
                    }*/
                    result
                };

                let gs_sz = self.get_operand_sz(ins, noperand);
                if mem_seg == Register::GS && self.cfg.is_x64() && !self.os.is_linux() {
                    // x64 Windows: GS is based at TEB.  Forward writes to teb_base + offset
                    // so that real ntdll code can update TEB fields without writing to address 0.
                    let teb_base = self.maps.get_mem("teb").get_base();
                    let teb_addr = teb_base.wrapping_add(temp_displace);
                    if self.cfg.verbose >= 1 {
                        log::trace!(
                            "gs:[0x{:x}] = 0x{:x}  (teb+0x{:x})",
                            temp_displace,
                            value,
                            temp_displace
                        );
                    }
                    match gs_sz {
                        8 => {
                            let _ = self.maps.write_byte(teb_addr, value as u8);
                        }
                        16 => {
                            let _ = self.maps.write_word(teb_addr, value as u16);
                        }
                        32 => {
                            let _ = self.maps.write_dword(teb_addr, value as u32);
                        }
                        64 => {
                            let _ = self.maps.write_qword(teb_addr, value);
                        }
                        _ => {
                            let _ = self.maps.write_qword(teb_addr, value);
                        }
                    }
                    return true;
                }

                if mem_seg == Register::FS || mem_base == Register::GS {
                    if self.os.is_linux() {
                        // Real TLS active: write through to memory at fs_base+offset.
                        // The offset must include any base register (e.g.
                        // `mov rax, fs:[rdx]` in __uselocale) — using only the
                        // displacement would smash the TCB self pointer at fs:[0].
                        let fs_base = self.regs().fs;
                        if fs_base != 0 {
                            let fs_off = if mem_base != Register::None
                                && mem_base != Register::RIP
                                && mem_base != Register::FS
                                && mem_base != Register::GS
                            {
                                self.regs().get_reg(mem_base).wrapping_add(temp_displace)
                            } else {
                                temp_displace
                            };
                            let real = fs_base.wrapping_add(fs_off);
                            let fs_sz = self.get_operand_sz(ins, noperand);
                            match fs_sz {
                                8 => {
                                    let _ = self.maps.write_byte(real, value as u8);
                                }
                                16 => {
                                    let _ = self.maps.write_word(real, value as u16);
                                }
                                32 => {
                                    let _ = self.maps.write_dword(real, value as u32);
                                }
                                _ => {
                                    let _ = self.maps.write_qword(real, value);
                                }
                            }
                            return true;
                        }
                        if self.cfg.verbose > 0 {
                            log::trace!("writting FS[0x{:x}] = 0x{:x}", temp_displace, value);
                        }
                        if value == 0x4b6c50 {
                            self.fs_mut().insert(0xffffffffffffffc8, 0x4b6c50);
                        }
                        self.fs_mut().insert(temp_displace, value);
                    } else {
                        if self.cfg.verbose >= 1 {
                            log::trace!("fs:{:x} setting SEH to 0x{:x}", temp_displace, value);
                        }
                        self.set_seh(value);
                    }

                    return true;
                }
                /* I don't think we can ever set fs and gs memory location and we have the faster method from above instead of calling virtual_address and switch statement
                let mem_addr = ins
                    .virtual_address(noperand, 0, |reg, idx, _sz| {
                        Some(self.regs().get_reg(reg))
                    })
                    .unwrap();

                if mem_addr != addr {
                    panic!("something wrong");
                }
                */
                // case when address is relative to rip then just return temp_displace
                let displace = if mem_base == Register::None {
                    /*if self.cfg.verbose >= 3 {
                        log::debug!("  mem_base is None, displace=temp_displace=0x{:x}", temp_displace);
                    }*/
                    temp_displace
                } else {
                    let base_val = self.regs().get_reg(mem_base);
                    let result = base_val.wrapping_add(temp_displace);
                    /*if self.cfg.verbose >= 3 {
                        log::debug!("  base_val=0x{:x}, displace=base+temp_displace=0x{:x}", base_val, result);
                    }*/
                    result
                };

                let displace_result = if !self.cfg.is_x64() {
                    let masked = displace & 0xffffffff;
                    /*if self.cfg.verbose >= 3 {
                        log::debug!("  32-bit mode: displace_result=0x{:x} (masked from 0x{:x})", masked, displace);
                    }*/
                    masked
                } else {
                    /*if self.cfg.verbose >= 3 {
                        log::debug!("  64-bit mode: displace_result=0x{:x}", displace);
                    }*/
                    displace
                };

                // do this for cmov optimization
                let mem_addr = if mem_base == Register::RIP {
                    /*if self.cfg.verbose >= 3 {
                        log::debug!("  RIP-relative: mem_addr=temp_displace=0x{:x}", temp_displace);
                    }*/
                    temp_displace
                } else {
                    /*if self.cfg.verbose >= 3 {
                        log::debug!("  Final mem_addr for write=0x{:x}", displace_result);
                    }*/
                    displace_result
                };

                let sz = self.get_operand_sz(ins, noperand);

                let value2 = if let Some(mut hook_fn) = self.hooks.hook_on_memory_write.take() {
                    let rip = self.regs().rip;
                    let result = hook_fn(self, rip, mem_addr, sz, value as u128) as u64;
                    self.hooks.hook_on_memory_write = Some(hook_fn);
                    result
                } else {
                    value
                };

                let old_value = if self.cfg.trace_mem {
                    match sz {
                        64 => self.maps.read_qword(mem_addr).unwrap_or(0),
                        32 => self.maps.read_dword(mem_addr).unwrap_or(0) as u64,
                        16 => self.maps.read_word(mem_addr).unwrap_or(0) as u64,
                        8 => self.maps.read_byte(mem_addr).unwrap_or(0) as u64,
                        _ => unreachable!("weird size: {}", sz),
                    }
                } else {
                    0
                };

                // now we flush the cacheline if it is written to executable memory and the cacheline exist
                let should_flush = self
                    .maps
                    .get_mem_by_addr(mem_addr)
                    .is_some_and(|mem1| mem1.can_execute());
                if should_flush {
                    let idx = self.x86_instruction_cache_ref().get_index_of(mem_addr, 0);
                    self.x86_instruction_cache().flush_cache_line(idx);
                }
                match sz {
                    64 => {
                        if !self.maps.write_qword(mem_addr, value2) {
                            if self.try_grow_stack(mem_addr)
                                && self.maps.write_qword(mem_addr, value2)
                            {
                                // grew + retry succeeded
                            } else {
                                return if self.cfg.skip_unimplemented {
                                    let map_name = format!("banzai_{:x}", mem_addr);
                                    let map = self
                                        .maps
                                        .create_map(
                                            &map_name,
                                            mem_addr,
                                            100,
                                            Permission::READ_WRITE_EXECUTE,
                                        )
                                        .expect("cannot create banzai map");
                                    map.write_qword(mem_addr, value2);
                                    true
                                } else {
                                    log::trace!(
                                        "/!\\ exception dereferencing bad address. 0x{:x}",
                                        mem_addr
                                    );
                                    self.exception(ExceptionType::BadAddressDereferencing);
                                    false
                                };
                            }
                        }
                    }
                    32 => {
                        if !self.maps.write_dword(mem_addr, to32!(value2)) {
                            if self.try_grow_stack(mem_addr)
                                && self.maps.write_dword(mem_addr, to32!(value2))
                            {
                                // grew + retry succeeded
                            } else {
                                return if self.cfg.skip_unimplemented {
                                    let map_name = format!("banzai_{:x}", mem_addr);
                                    let map = self
                                        .maps
                                        .create_map(
                                            &map_name,
                                            mem_addr,
                                            100,
                                            Permission::READ_WRITE_EXECUTE,
                                        )
                                        .expect("cannot create banzai map");
                                    map.write_dword(mem_addr, to32!(value2));
                                    true
                                } else {
                                    log::trace!(
                                        "/!\\ exception dereferencing bad address. 0x{:x}",
                                        mem_addr
                                    );
                                    self.exception(ExceptionType::BadAddressDereferencing);
                                    false
                                };
                            }
                        }
                    }
                    16 => {
                        if !self.maps.write_word(mem_addr, value2 as u16) {
                            if self.try_grow_stack(mem_addr)
                                && self.maps.write_word(mem_addr, value2 as u16)
                            {
                                // grew + retry succeeded
                            } else {
                                return if self.cfg.skip_unimplemented {
                                    let map_name = format!("banzai_{:x}", mem_addr);
                                    let map = self
                                        .maps
                                        .create_map(
                                            &map_name,
                                            mem_addr,
                                            100,
                                            Permission::READ_WRITE_EXECUTE,
                                        )
                                        .expect("cannot create banzai map");
                                    map.write_word(mem_addr, value2 as u16);
                                    true
                                } else {
                                    log::trace!(
                                        "/!\\ exception dereferencing bad address. 0x{:x}",
                                        mem_addr
                                    );
                                    self.exception(ExceptionType::BadAddressDereferencing);
                                    false
                                };
                            }
                        }
                    }
                    8 => {
                        if !self.maps.write_byte(mem_addr, value2 as u8) {
                            if self.try_grow_stack(mem_addr)
                                && self.maps.write_byte(mem_addr, value2 as u8)
                            {
                                // grew + retry succeeded
                            } else {
                                static FIRST_BYTE_OOB: std::sync::atomic::AtomicBool =
                                    std::sync::atomic::AtomicBool::new(false);
                                if (0x412000..=0x420000).contains(&mem_addr)
                                    && !FIRST_BYTE_OOB
                                        .swap(true, std::sync::atomic::Ordering::Relaxed)
                                {
                                    log::trace!(
                                        "DEBUG first OOB byte write at 0x{:x} from rip=0x{:x} pos={} rcx=0x{:x} rdi=0x{:x} rsi=0x{:x}",
                                        mem_addr,
                                        self.regs().rip,
                                        self.pos,
                                        self.regs().rcx,
                                        self.regs().rdi,
                                        self.regs().rsi,
                                    );
                                }
                                return if self.cfg.skip_unimplemented {
                                    let map_name = format!("banzai_{:x}", mem_addr);
                                    let map = self
                                        .maps
                                        .create_map(
                                            &map_name,
                                            mem_addr,
                                            100,
                                            Permission::READ_WRITE_EXECUTE,
                                        )
                                        .expect("cannot create banzai map");
                                    map.write_byte(mem_addr, value2 as u8);
                                    true
                                } else {
                                    log::trace!(
                                        "/!\\ exception dereferencing bad address. 0x{:x}",
                                        mem_addr
                                    );
                                    self.exception(ExceptionType::BadAddressDereferencing);
                                    false
                                };
                            }
                        }
                    }
                    _ => unimplemented!("weird size"),
                }

                if self.cfg.trace_mem {
                    let name = self
                        .maps
                        .get_addr_name(mem_addr)
                        .unwrap_or("not mapped");
                    let memory_operation = MemoryOperation {
                        pos: self.pos,
                        rip: self.regs().rip,
                        op: "write".to_string(),
                        bits: sz,
                        address: mem_addr,
                        old_value,
                        new_value: value2,
                        name: name.to_string(),
                    };
                    self.memory_operations.push(memory_operation);
                    log::trace!(
                        "\tmem_trace: pos = {} rip = {:x} op = write bits = {} address = 0x{:x} value = 0x{:x} name = '{}'",
                        self.pos,
                        self.regs().rip,
                        sz,
                        mem_addr,
                        value2,
                        name
                    );
                }

                /*
                let name = match self.maps.get_addr_name(mem_addr) {
                    Some(n) => n,
                    None => "not mapped".to_string(),
                };

                if name == "code" {
                    if self.cfg.verbose >= 1 {
                        log::trace!("/!\\ polymorfic code, addr 0x{:x}", mem_addr);
                    }
                    self.force_break = true;
                }*/

                if self.bp.is_bp_mem_write_addr(mem_addr) {
                    log::trace!("Memory breakpoint on write 0x{:x}", mem_addr);
                    if self.running_script {
                        self.force_break = true;
                    } else {
                        Console::spawn_console(self);
                    }
                }
            }

            _ => unimplemented!("unimplemented operand type {:?}", ins.op_kind(noperand)),
        };
        true
    }

    /// Get a 128bits operand ie for xmm instructions.
    pub fn get_operand_xmm_value_128(
        &mut self,
        ins: &Instruction,
        noperand: u32,
        do_derref: bool,
    ) -> Option<u128> {
        assert!(ins.op_count() > noperand);

        let value: u128 = match ins.op_kind(noperand) {
            OpKind::Register => self.regs().get_xmm_reg(ins.op_register(noperand)),

            OpKind::Immediate64 => ins.immediate64() as u128,
            OpKind::Immediate8 => ins.immediate8() as u128,
            OpKind::Immediate16 => ins.immediate16() as u128,
            OpKind::Immediate32 => ins.immediate32() as u128,
            OpKind::Immediate8to64 => ins.immediate8to64() as u128,
            OpKind::Immediate32to64 => ins.immediate32to64() as u128,
            OpKind::Immediate8to32 => ins.immediate8to32() as u32 as u128,
            OpKind::Immediate8to16 => ins.immediate8to16() as u16 as u128,

            OpKind::Memory => {
                let mem_addr = match ins
                    .virtual_address(noperand, 0, |reg, idx, _sz| Some(self.regs().get_reg(reg)))
                {
                    Some(addr) => addr,
                    None => {
                        log::trace!("/!\\ xmm exception reading operand");
                        self.exception(ExceptionType::SettingXmmOperand);
                        return None;
                    }
                };

                if do_derref {
                    if let Some(mut hook_fn) = self.hooks.hook_on_memory_read.take() {
                        let rip = self.regs().rip;
                        hook_fn(self, rip, mem_addr, 128);
                        self.hooks.hook_on_memory_read = Some(hook_fn);
                    }

                    let value: u128 = match self.maps.read_128bits_le(mem_addr) {
                        Some(v) => v,
                        None => {
                            log::trace!("/!\\ exception reading xmm operand at 0x{:x} ", mem_addr);
                            self.exception(ExceptionType::ReadingXmmOperand);
                            return None;
                        }
                    };
                    value
                } else {
                    mem_addr as u128
                }
            }
            _ => unimplemented!("unimplemented operand type {:?}", ins.op_kind(noperand)),
        };
        Some(value)
    }

    /// Set an operand of 128 bits, like xmm.
    pub fn set_operand_xmm_value_128(&mut self, ins: &Instruction, noperand: u32, value: u128) {
        assert!(ins.op_count() > noperand);

        match ins.op_kind(noperand) {
            OpKind::Register => self
                .regs_mut()
                .set_xmm_reg(ins.op_register(noperand), value),
            OpKind::Memory => {
                let mem_addr = match ins
                    .virtual_address(noperand, 0, |reg, idx, _sz| Some(self.regs().get_reg(reg)))
                {
                    Some(addr) => addr,
                    None => {
                        log::trace!("/!\\ exception setting xmm operand.");
                        self.exception(ExceptionType::SettingXmmOperand);
                        return;
                    }
                };

                let value2 = if let Some(mut hook_fn) = self.hooks.hook_on_memory_write.take() {
                    let rip = self.regs().rip;
                    let result = hook_fn(self, rip, mem_addr, 128, value);
                    self.hooks.hook_on_memory_write = Some(hook_fn);
                    result
                } else {
                    value
                };

                for (i, b) in value2.to_le_bytes().iter().enumerate() {
                    self.maps.write_byte(mem_addr + i as u64, *b);
                }
            }
            _ => unimplemented!("unimplemented operand type {:?}", ins.op_kind(noperand)),
        };
    }

    pub fn get_operand_ymm_value_256(
        &mut self,
        ins: &Instruction,
        noperand: u32,
        do_derref: bool,
    ) -> Option<regs64::U256> {
        assert!(ins.op_count() > noperand);

        let value: regs64::U256 = match ins.op_kind(noperand) {
            OpKind::Register => self.regs().get_ymm_reg(ins.op_register(noperand)),

            OpKind::Immediate64 => regs64::U256::from(ins.immediate64()),
            OpKind::Immediate8 => regs64::U256::from(ins.immediate8() as u64),
            OpKind::Immediate16 => regs64::U256::from(ins.immediate16() as u64),
            OpKind::Immediate32 => regs64::U256::from(ins.immediate32() as u64),
            OpKind::Immediate8to64 => regs64::U256::from(ins.immediate8to64() as u64),
            OpKind::Immediate32to64 => regs64::U256::from(ins.immediate32to64() as u64),
            OpKind::Immediate8to32 => regs64::U256::from(ins.immediate8to32() as u32 as u64),
            OpKind::Immediate8to16 => regs64::U256::from(ins.immediate8to16() as u16 as u64),

            OpKind::Memory => {
                let mem_addr = match ins
                    .virtual_address(noperand, 0, |reg, idx, _sz| Some(self.regs().get_reg(reg)))
                {
                    Some(addr) => addr,
                    None => {
                        log::trace!("/!\\ xmm exception reading operand");
                        self.exception(ExceptionType::ReadingXmmOperand);
                        return None;
                    }
                };

                if do_derref {
                    if let Some(mut hook_fn) = self.hooks.hook_on_memory_read.take() {
                        let rip = self.regs().rip;
                        hook_fn(self, rip, mem_addr, 256);
                        self.hooks.hook_on_memory_read = Some(hook_fn);
                    }

                    let bytes = self.maps.read_bytes_array::<32>(mem_addr);
                    

                    regs64::U256::from_little_endian(&bytes)
                } else {
                    regs64::U256::from(mem_addr)
                }
            }
            _ => unimplemented!("unimplemented operand type {:?}", ins.op_kind(noperand)),
        };
        Some(value)
    }

    /// Set a 256bits value to an operand, usually ymm instructions.
    pub fn set_operand_ymm_value_256(
        &mut self,
        ins: &Instruction,
        noperand: u32,
        value: regs64::U256,
    ) {
        assert!(ins.op_count() > noperand);

        match ins.op_kind(noperand) {
            OpKind::Register => self
                .regs_mut()
                .set_ymm_reg(ins.op_register(noperand), value),
            OpKind::Memory => {
                let mem_addr = match ins
                    .virtual_address(noperand, 0, |reg, idx, _sz| Some(self.regs().get_reg(reg)))
                {
                    Some(addr) => addr,
                    None => {
                        log::trace!("/!\\ exception setting xmm operand.");
                        self.exception(ExceptionType::SettingXmmOperand);
                        return;
                    }
                };

                // ymm dont support value modification from hook, for now
                let value_u128: u128 = ((value.0[1] as u128) << 64) | value.0[0] as u128;
                let value2 = if let Some(mut hook_fn) = self.hooks.hook_on_memory_write.take() {
                    let rip = self.regs().rip;
                    let result = hook_fn(self, rip, mem_addr, 256, value_u128);
                    self.hooks.hook_on_memory_write = Some(hook_fn);
                    result
                } else {
                    value_u128
                };

                let mut bytes: Vec<u8> = vec![0; 32];
                value.to_little_endian(&mut bytes);
                self.maps.write_bytes(mem_addr, &bytes);
            }
            _ => unimplemented!("unimplemented operand type {:?}", ins.op_kind(noperand)),
        };
    }

    /// Fetch the size in amount of bits of a specific operand (reg/mem/imm), if it's a memory operation it
    /// depend on the dword ptr, qword ptr etc.
    pub fn get_operand_sz(&self, ins: &Instruction, noperand: u32) -> u32 {
        let reg: Register = ins.op_register(noperand);
        if reg.is_xmm() {
            return 128;
        }
        if reg.is_ymm() {
            return 256;
        }

        match ins.op_kind(noperand) {
            //TODO: OpKind::Immediate8to64 could be 8
            OpKind::NearBranch64
            | OpKind::Immediate64
            | OpKind::Immediate32to64
            | OpKind::Immediate8to64
            | OpKind::MemoryESRDI
            | OpKind::MemorySegRSI => 64,
            OpKind::NearBranch32
            | OpKind::Immediate32
            | OpKind::Immediate8to32
            | OpKind::FarBranch32
            | OpKind::MemoryESEDI
            | OpKind::MemorySegESI => 32,
            OpKind::NearBranch16
            | OpKind::FarBranch16
            | OpKind::Immediate16
            | OpKind::Immediate8to16 => 16,
            OpKind::Immediate8 => 8,
            OpKind::Register => self.regs().get_size(ins.op_register(noperand)),

            OpKind::Memory => match ins.memory_size() {
                MemorySize::Float16
                | MemorySize::UInt16
                | MemorySize::Int16
                | MemorySize::WordOffset
                | MemorySize::Packed128_UInt16
                | MemorySize::Bound16_WordWord => 16,
                MemorySize::Float32
                | MemorySize::FpuEnv28
                | MemorySize::UInt32
                | MemorySize::Int32
                | MemorySize::DwordOffset
                | MemorySize::Packed128_UInt32
                | MemorySize::Bound32_DwordDword
                | MemorySize::Packed64_Float32
                | MemorySize::Packed256_UInt32
                | MemorySize::Packed128_Float32
                | MemorySize::Packed256_Float32
                | MemorySize::SegPtr32 => 32,
                MemorySize::Float64
                | MemorySize::UInt64
                | MemorySize::Int64
                | MemorySize::QwordOffset
                | MemorySize::Packed128_UInt64
                | MemorySize::Packed256_UInt64
                | MemorySize::Packed128_Float64
                | MemorySize::Packed256_Float64 => 64,
                MemorySize::UInt8 | MemorySize::Int8 => 8,
                MemorySize::Packed256_UInt128 => 128,
                _ => unimplemented!("memory size {:?}", ins.memory_size()),
            },
            _ => unimplemented!("unimplemented operand type {:?}", ins.op_kind(noperand)),
        }
    }
}
