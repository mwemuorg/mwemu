use crate::{emu::Emu, windows::constants, windows::structures::MemoryOperation};

impl Emu {
    /// This is not used on the emulation.
    /// It's part of a feature like  reading or wirtting like it was asm "dword ptr [rax + 0x123]"
    /// NOTE: x86 operand syntax only. Will panic on aarch64.
    pub fn memory_operand_to_address(&mut self, operand: &str) -> u64 {
        if self.cfg.arch.is_aarch64() {
            panic!("memory_operand_to_address() uses x86 operand syntax, not available on aarch64");
        }
        let spl: Vec<&str> = operand.split('[').collect::<Vec<&str>>()[1]
            .split(']')
            .collect::<Vec<&str>>()[0]
            .split(' ')
            .collect();

        if operand.contains("fs:[") || operand.contains("gs:[") {
            let mem = operand.split(':').collect::<Vec<&str>>()[1];
            let value = self.memory_operand_to_address(mem);

            /*
            fs:[0x30]

            FS:[0x00] : Current SEH Frame
            FS:[0x18] : TEB (Thread Environment Block)
            FS:[0x20] : PID
            FS:[0x24] : TID
            FS:[0x30] : PEB (Process Environment Block)
            FS:[0x34] : Last Error Value
            */

            //let inm = self.get_inmediate(spl[0]);
            if self.cfg.verbose >= 1 {
                log::trace!("FS ACCESS TO 0x{:x}", value);
            }

            if value == 0x30 {
                // PEB
                if self.cfg.verbose >= 1 {
                    log::trace!("ACCESS TO PEB");
                }
                let peb = self.maps.get_mem("peb");
                return peb.get_base();
            }

            if value == 0x18 {
                if self.cfg.verbose >= 1 {
                    log::trace!("ACCESS TO TEB");
                }
                let teb = self.maps.get_mem("teb");
                return teb.get_base();
            }

            if value == 0x2c {
                if self.cfg.verbose >= 1 {
                    log::trace!("ACCESS TO CURRENT LOCALE");
                }
                return constants::EN_US_LOCALE as u64;
            }

            if value == 0xc0 {
                if self.cfg.verbose >= 1 {
                    log::trace!("CHECKING IF ITS 32bits (ISWOW64)");
                }

                if self.cfg.is_x64() {
                    return 0;
                }

                return 1;
            }

            panic!("not implemented: {}", operand);
        }

        if spl.len() == 3 {
            //ie eax + 0xc
            let sign = spl[1];

            // weird case: [esi + eax*4]
            if spl[2].contains('*') {
                let spl2: Vec<&str> = spl[2].split('*').collect();
                if spl2.len() != 2 {
                    panic!(
                        "case ie [esi + eax*4] bad parsed the *  operand:{}",
                        operand
                    );
                }

                let reg1_val = self.regs().get_by_name(spl[0]);
                let reg2_val = self.regs().get_by_name(spl2[0]);
                let num = u64::from_str_radix(spl2[1].trim_start_matches("0x"), 16)
                    .expect("bad num conversion");

                if sign != "+" && sign != "-" {
                    panic!("weird sign2 {}", sign);
                }

                if sign == "+" {
                    return reg1_val + (reg2_val * num);
                }

                if sign == "-" {
                    return reg1_val - (reg2_val * num);
                }

                unimplemented!();
            }

            let reg = spl[0];
            let sign = spl[1];
            //log::trace!("disp --> {}  operand:{}", spl[2], operand);

            let disp: u64 = if self.regs().is_reg(spl[2]) {
                self.regs().get_by_name(spl[2])
            } else {
                u64::from_str_radix(spl[2].trim_start_matches("0x"), 16).expect("bad disp")
            };

            if sign != "+" && sign != "-" {
                panic!("weird sign {}", sign);
            }

            if sign == "+" {
                let r: u64 = self.regs().get_by_name(reg) + disp;
                return r & 0xffffffff;
            } else {
                return self.regs().get_by_name(reg) - disp;
            }
        }

        if spl.len() == 1 {
            //ie [eax]
            let reg = spl[0];

            if reg.contains("0x") {
                let addr: u64 =
                    u64::from_str_radix(reg.trim_start_matches("0x"), 16).expect("bad disp2");
                return addr;
                // weird but could be a hardcoded address [0x11223344]
            }

            let reg_val = self.regs().get_by_name(reg);
            return reg_val;
        }

        0
    }

    /// This is not used on the emulation.
    /// It's a feature to read memory based on an string like "dword ptr [rax + 0x1234]"
    /// Unperfect but cool feautre, don't alow all the combinations possible.
    /// Not sure if this features will be removed.
    /// The emulator uses much more eficient ways to decode the operands than this.
    pub fn memory_read(&mut self, operand: &str) -> Option<u64> {
        if operand.contains("fs:[0]") {
            if self.cfg.verbose >= 1 {
                log::trace!("{} Reading SEH fs:[0] 0x{:x}", self.pos, self.seh());
            }
            return Some(self.seh());
        }

        let addr: u64 = self.memory_operand_to_address(operand);

        if operand.contains("fs:[") || operand.contains("gs:[") {
            return Some(addr);
        }

        let bits = self.get_size(operand);
        // check integrity of eip, esp and ebp registers

        let stack = self.maps.get_mem("stack");

        // could be normal using part of code as stack
        if !stack.inside(self.regs().get_esp()) {
            //hack: redirect stack
            let esp = stack.get_base() + 0x1ff;
            self.regs_mut().set_esp(esp);
            panic!("/!\\ fixing stack.")
        }

        match bits {
            64 => match self.maps.read_qword(addr) {
                Some(v) => {
                    if self.cfg.trace_mem {
                        let name = self.maps.get_addr_name(addr).unwrap_or("not mapped");
                        let memory_operation = MemoryOperation {
                            pos: self.pos,
                            rip: self.pc(),
                            op: "read".to_string(),
                            bits: 64,
                            address: addr,
                            old_value: 0, // not needed for read?
                            new_value: v,
                            name: name.to_string(),
                        };
                        self.memory_operations.push(memory_operation);
                        log::trace!(
                            "\tmem_trace: pos = {} rip = {:x} op = read bits = {} address = 0x{:x} value = 0x{:x} name = '{}'",
                            self.pos,
                            self.pc(),
                            64,
                            addr,
                            v,
                            name
                        );
                    }
                    Some(v)
                }
                None => None,
            },
            32 => match self.maps.read_dword(addr) {
                Some(v) => {
                    if self.cfg.trace_mem {
                        let name = self
                            .maps
                            .get_addr_name(addr)
                            .unwrap_or("not mapped");
                        let memory_operation = MemoryOperation {
                            pos: self.pos,
                            rip: self.pc(),
                            op: "read".to_string(),
                            bits: 32,
                            address: addr,
                            old_value: 0, // not needed for read?
                            new_value: v as u64,
                            name: name.to_string(),
                        };
                        self.memory_operations.push(memory_operation);
                        log::trace!(
                            "\tmem_trace: pos = {} rip = {:x} op = read bits = {} address = 0x{:x} value = 0x{:x} name = '{}'",
                            self.pos,
                            self.pc(),
                            32,
                            addr,
                            v,
                            name
                        );
                    }
                    Some(v.into())
                }
                None => None,
            },
            16 => match self.maps.read_word(addr) {
                Some(v) => {
                    if self.cfg.trace_mem {
                        let name = self
                            .maps
                            .get_addr_name(addr)
                            .unwrap_or("not mapped");
                        let memory_operation = MemoryOperation {
                            pos: self.pos,
                            rip: self.pc(),
                            op: "read".to_string(),
                            bits: 16,
                            address: addr,
                            old_value: 0, // not needed for read?
                            new_value: v as u64,
                            name: name.to_string(),
                        };
                        self.memory_operations.push(memory_operation);
                        log::trace!(
                            "\tmem_trace: pos = {} rip = {:x} op = read bits = {} address = 0x{:x} value = 0x{:x} name = '{}'",
                            self.pos,
                            self.pc(),
                            16,
                            addr,
                            v,
                            name
                        );
                    }
                    Some(v.into())
                }
                None => None,
            },
            8 => match self.maps.read_byte(addr) {
                Some(v) => {
                    if self.cfg.trace_mem {
                        let name = self
                            .maps
                            .get_addr_name(addr)
                            .unwrap_or("not mapped");
                        let memory_operation = MemoryOperation {
                            pos: self.pos,
                            rip: self.pc(),
                            op: "read".to_string(),
                            bits: 8,
                            address: addr,
                            old_value: 0, // not needed for read?
                            new_value: v as u64,
                            name: name.to_string(),
                        };
                        self.memory_operations.push(memory_operation);
                        log::trace!(
                            "\tmem_trace: pos = {} rip = {:x} op = read bits = {} address = 0x{:x} value = 0x{:x} name = '{}'",
                            self.pos,
                            self.pc(),
                            8,
                            addr,
                            v,
                            name
                        );
                    }
                    Some(v.into())
                }
                None => None,
            },
            _ => panic!("weird size: {}", operand),
        }
    }

    /// This is not used on the emulation.
    /// It's a feature to write memory based on an string like "dword ptr [rax + 0x1234]"
    /// Unperfect but cool feautre, don't alow all the combinations possible.
    /// Not sure if this features will be removed.
    /// The emulator uses much more eficient ways to decode the operands than this.
    pub fn memory_write(&mut self, operand: &str, value: u64) -> bool {
        if operand.contains("fs:[0]") {
            log::trace!("Setting SEH fs:[0]  0x{:x}", value);
            self.set_seh(value);
            return true;
        }

        let addr: u64 = self.memory_operand_to_address(operand);

        /*if !self.maps.is_mapped(addr) {
        panic!("writting in non mapped memory");
        }*/

        let name = self.maps.get_addr_name(addr).unwrap_or("error");

        if name == "code" {
            if self.cfg.verbose >= 1 {
                log::trace!("/!\\ polymorfic code, write at 0x{:x}", addr);
            }
            self.force_break = true;
        }

        let bits = self.get_size(operand);

        if self.cfg.trace_mem {
            let memory_operation = MemoryOperation {
                pos: self.pos,
                rip: self.pc(),
                op: "write".to_string(),
                bits: bits as u32,
                address: addr,
                old_value: match bits {
                    64 => self.maps.read_qword(addr).unwrap_or(0),
                    32 => self.maps.read_dword(addr).unwrap_or(0) as u64,
                    16 => self.maps.read_word(addr).unwrap_or(0) as u64,
                    8 => self.maps.read_byte(addr).unwrap_or(0) as u64,
                    _ => unreachable!("weird size: {}", operand),
                },
                new_value: value,
                name: name.to_string(),
            };
            self.memory_operations.push(memory_operation);
            log::trace!(
                "\tmem_trace: pos = {} rip = {:x} op = write bits = {} address = 0x{:x} value = 0x{:x} name = '{}'",
                self.pos,
                self.pc(),
                32,
                addr,
                value,
                name
            );
        }

        match bits {
            64 => self.maps.write_qword(addr, value),
            32 => self.maps.write_dword(addr, (value & 0xffffffff) as u32),
            16 => self.maps.write_word(addr, (value & 0x0000ffff) as u16),
            8 => self.maps.write_byte(addr, (value & 0x000000ff) as u8),
            _ => unreachable!("weird size: {}", operand),
        }
    }

    /// This is not used on the emulation.
    /// It's just for a memory reading feature.
    /// The emulation uses much more efficient ways to decode
    pub fn get_size(&self, operand: &str) -> u8 {
        if operand.contains("byte ptr") {
            return 8;
        } else if operand.contains("dword ptr") {
            return 32;
        } else if operand.contains("qword ptr") {
            return 64;
        } else if operand.contains("word ptr") {
            return 16;
        }

        let c: Vec<char> = operand.chars().collect();

        if operand.len() == 3 {
            if c[0] == 'e' {
                return 32;
            }
        } else if operand.len() == 2 {
            if c[1] == 'x' {
                return 16;
            }

            if c[1] == 'h' || c[1] == 'l' {
                return 8;
            }

            if c[1] == 'i' {
                return 16;
            }
        }

        panic!("weird size: {}", operand);
    }

    /// Attempt to auto-extend the thread stack to cover `fault_addr`.
    ///
    /// On real Windows the stack starts as a small committed region with a
    /// PAGE_GUARD page at the bottom; touching the guard page raises
    /// STATUS_GUARD_PAGE_VIOLATION, which the kernel transparently handles by
    /// committing a new page and moving the guard. We don't model guard pages,
    /// so we approximate the same effect here: if a memory fault lands inside
    /// (or just under) the current "stack" map and within a sensible distance
    /// of RSP, we map a new RW region covering the faulting page so the
    /// instruction can be retried.
    ///
    /// Returns `true` if a new region was mapped (caller should retry the
    /// access); `false` if the fault is unrelated to stack growth.
    pub fn try_grow_stack(&mut self, fault_addr: u64) -> bool {
        // Tunables. Real Windows reserves 1 MiB by default and lets it grow up
        // to the reserve limit; we keep the same upper bound on the growth we
        // are willing to do per fault — but we cap how far below RSP we grow,
        // since wild pointers are not stack overflows.
        const MAX_DIST_BELOW_RSP: u64 = 0x100000; // 1 MiB
        const GROW_PAGE_SIZE: u64 = 0x1000; // 4 KiB

        if !self.cfg.is_x64() {
            return false;
        }

        let rsp = self.regs().rsp;
        // The fault must be reasonably close to the current stack pointer.
        // Below RSP is the typical case (`_chkstk` / large frame); slightly
        // above RSP also happens with red-zone-like writes.
        if fault_addr >= rsp {
            // Above RSP: only allow a small overshoot (8 KB) — anything more
            // is a wild pointer rather than a stack reference.
            if fault_addr.saturating_sub(rsp) > 0x2000 {
                return false;
            }
        } else if rsp.saturating_sub(fault_addr) > MAX_DIST_BELOW_RSP {
            return false;
        }

        // Page-align the faulting address.
        let page_base = fault_addr & !(GROW_PAGE_SIZE - 1);

        // If the address is already mapped, nothing to do here.
        if self.maps.is_mapped(page_base) {
            return false;
        }

        // Prefer extending the existing "stack" map downward by creating a new
        // adjacent map for the page. We give it a stable name so subsequent
        // faults extend with separate maps rather than fail to allocate.
        let name = format!("stack_grow_{:x}", page_base);
        match self.maps.create_map(
            &name,
            page_base,
            GROW_PAGE_SIZE,
            crate::maps::mem64::Permission::READ_WRITE,
        ) {
            Ok(_) => {
                log::trace!(
                    "stack auto-grow: mapped {} at 0x{:x} (rsp=0x{:x}, fault=0x{:x})",
                    name,
                    page_base,
                    rsp,
                    fault_addr
                );
                true
            }
            Err(e) => {
                log::trace!("stack auto-grow failed for 0x{:x}: {:?}", page_base, e);
                false
            }
        }
    }
}
