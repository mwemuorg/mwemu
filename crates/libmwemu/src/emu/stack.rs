use crate::{emu::Emu, windows::structures::MemoryOperation};

impl Emu {
    /// Push a dword to the stack and dec the esp
    /// This will return false if stack pointer is pointing to non allocated place.
    pub fn stack_push32(&mut self, value: u32) -> bool {
        if self.cfg.stack_trace {
            log::trace!("--- stack push32 ---");
            self.maps.dump_dwords(self.regs().get_esp(), 5);
        }

        if self.cfg.trace_mem {
            let name = self
                .maps
                .get_addr_name(self.regs().get_esp())
                .unwrap_or("not mapped");
            let memory_operation = MemoryOperation {
                pos: self.pos,
                rip: self.regs().rip,
                op: "write".to_string(),
                bits: 32,
                address: self.regs().get_esp() - 4,
                old_value: self.maps.read_dword(self.regs().get_esp()).unwrap_or(0) as u64,
                new_value: value as u64,
                name: name.to_string(),
            };
            self.memory_operations.push(memory_operation);
            log::trace!(
                "\tmem_trace: pos = {} rip = {:x} op = write bits = {} address = 0x{:x} value = 0x{:x} name = '{}'",
                self.pos,
                self.regs().rip,
                32,
                self.regs().get_esp(),
                value,
                name
            );
        }

        let esp = self.regs().get_esp() - 4;
        self.regs_mut().set_esp(esp);
        //self.stack_lvl[self.stack_lvl_idx] += 1;
        //log::trace!("push32 stack level is {} deep {}", self.stack_lvl[self.stack_lvl_idx], self.stack_lvl_idx);

        /*
        let stack = self.maps.get_mem("stack");
        if stack.inside(self.regs().get_esp()) {
            if !self.maps.write_dword(self.regs().get_esp(), value) {
                //if !stack.write_dword(self.regs().get_esp(), value) {
                return false;
            }
        } else {
            let mem = match self.maps.get_mem_by_addr(self.regs().get_esp()) {
                Some(m) => m,
                None => {
                    log::trace!(
                        "/!\\ pushing stack outside maps esp: 0x{:x}",
                        self.regs().get_esp()
                    );
                    Console::spawn_console(self);
                    return false;
                }
            };
            if !self.maps.write_dword(self.regs().get_esp(), value) {
                //if !mem.write_dword(self.regs().get_esp(), value) {
                return false;
            }
        }*/

        if self.maps.write_dword(self.regs().get_esp(), value) {
            true
        } else {
            log::trace!(
                "/!\\ pushing in non mapped mem 0x{:x}",
                self.regs().get_esp()
            );
            false
        }
    }

    /// Push a qword to the stack and dec the stack pointer.
    /// Works for both x86_64 (rsp) and aarch64 (sp).
    /// This will return false if stack pointer is pointing to non allocated place.
    pub fn stack_push64(&mut self, value: u64) -> bool {
        let sp = self.sp();

        if self.cfg.stack_trace {
            log::trace!("--- stack push64  ---");
            self.maps.dump_qwords(sp, 5);
        }

        if self.cfg.trace_mem {
            let pc = self.pc();
            let name = self.maps.get_addr_name(sp).unwrap_or("not mapped");
            let memory_operation = MemoryOperation {
                pos: self.pos,
                rip: pc,
                op: "write".to_string(),
                bits: 64,
                address: sp - 8,
                old_value: self.maps.read_qword(sp).unwrap_or(0),
                new_value: value,
                name: name.to_string(),
            };
            self.memory_operations.push(memory_operation);
            log::trace!(
                "\tmem_trace: pos = {} rip = {:x} op = write bits = {} address = 0x{:x} value = 0x{:x} name = '{}'",
                self.pos,
                pc,
                64,
                sp,
                value,
                name
            );
        }

        let new_sp = sp - 8;
        self.set_sp(new_sp);

        if self.maps.write_qword(new_sp, value) {
            return true;
        }
        // Auto-grow the stack on a fresh push that lands in a yet-uncommitted
        // page (Windows would have hit a PAGE_GUARD fault and committed it).
        if self.try_grow_stack(new_sp) && self.maps.write_qword(new_sp, value) {
            return true;
        }
        log::trace!("/!\\ pushing in non mapped mem 0x{:x}", new_sp);
        false
    }

    /// Pop a dword from stack and return it, None if esp points to unmapped zone.
    pub fn stack_pop32(&mut self, pop_instruction: bool) -> Option<u32> {
        if self.cfg.stack_trace {
            log::trace!("--- stack pop32 ---");
            self.maps.dump_dwords(self.regs().get_esp(), 5);
        }

        /*
        let stack = self.maps.get_mem("stack");
        if stack.inside(self.regs().get_esp()) {
            //let value = stack.read_dword(self.regs().get_esp());
            let value = match self.maps.read_dword(self.regs().get_esp()) {
                Some(v) => v,
                None => {
                    log::trace!("esp out of stack");
                    return None;
                }
            };
            if self.cfg.verbose >= 1
                && pop_instruction
                && self.maps.get_mem("code").inside(value.into())
            {
                log::trace!("/!\\ poping a code address 0x{:x}", value);
            }
            let esp = self.regs().get_esp() + 4;
            self.regs_mut().set_esp(esp);
            return Some(value);
        }

        let mem = match self.maps.get_mem_by_addr(self.regs().get_esp()) {
            Some(m) => m,
            None => {
                log::trace!(
                    "poping stack outside map  esp: 0x{:x}",
                    self.regs().get_esp() as u32
                );
                Console::spawn_console(self);
                return None;
            }
        };*/

        let value = match self.maps.read_dword(self.regs().get_esp()) {
            Some(v) => v,
            None => {
                log::trace!("esp point to non mapped mem");
                return None;
            }
        };

        /*  walking mems in very pop is slow, and now we are not using "code" map
        if self.cfg.verbose >= 1
            && pop_instruction
            && self.maps.get_mem("code").inside(value.into())
        {
            log::trace!("/!\\ poping a code address 0x{:x}", value);
        }
        */

        if self.cfg.trace_mem {
            // Record the read from stack memory
            let name = self
                .maps
                .get_addr_name(self.regs().get_esp())
                .unwrap_or("not mapped");
            let read_operation = MemoryOperation {
                pos: self.pos,
                rip: self.regs().rip,
                op: "read".to_string(),
                bits: 32,
                address: self.regs().get_esp(),
                old_value: 0, // not needed for read
                new_value: value as u64,
                name: name.to_string(),
            };
            self.memory_operations.push(read_operation);
            log::trace!(
                "\tmem_trace: pos = {} rip = {:x} op = read bits = {} address = 0x{:x} value = 0x{:x} name = '{}'",
                self.pos,
                self.regs().rip,
                32,
                self.regs().get_esp(),
                value,
                name
            );

            // Record the write to register
            let write_operation = MemoryOperation {
                pos: self.pos,
                rip: self.regs().rip,
                op: "write".to_string(),
                bits: 32,
                address: self.regs().get_esp(),
                old_value: self.maps.read_dword(self.regs().get_esp()).unwrap_or(0) as u64,
                new_value: value as u64, // new value being written
                name: "register".to_string(),
            };
            self.memory_operations.push(write_operation);
            log::trace!(
                "\tmem_trace: pos = {} rip = {:x} op = write bits = {} address = 0x{:x} value = 0x{:x} name = 'register'",
                self.pos,
                self.regs().rip,
                32,
                self.regs().get_esp(),
                value
            );
        }

        let esp = self.regs().get_esp() + 4;
        self.regs_mut().set_esp(esp);
        //self.stack_lvl[self.stack_lvl_idx] -= 1;
        //log::trace!("pop32 stack level is {} deep {}", self.stack_lvl[self.stack_lvl_idx], self.stack_lvl_idx);
        Some(value)
    }

    /// Pop a qword from stack, return None if cannot read the stack pointer address.
    /// Works for both x86_64 (rsp) and aarch64 (sp).
    pub fn stack_pop64(&mut self, pop_instruction: bool) -> Option<u64> {
        let sp = self.sp();

        if self.cfg.stack_trace {
            log::trace!("--- stack pop64 ---");
            self.maps.dump_qwords(sp, 5);
        }

        let value = match self.maps.read_qword(sp) {
            Some(v) => v,
            None => {
                log::trace!("stack pointer points to non mapped mem");
                return None;
            }
        };

        if self.cfg.trace_mem {
            let pc = self.pc();
            let name = self.maps.get_addr_name(sp).unwrap_or("not mapped");
            let read_operation = MemoryOperation {
                pos: self.pos,
                rip: pc,
                op: "read".to_string(),
                bits: 64,
                address: sp,
                old_value: 0,
                new_value: value,
                name: name.to_string(),
            };
            self.memory_operations.push(read_operation);
            log::trace!(
                "\tmem_trace: pos = {} rip = {:x} op = read bits = {} address = 0x{:x} value = 0x{:x} name = '{}'",
                self.pos,
                pc,
                64,
                sp,
                value,
                name
            );

            let write_operation = MemoryOperation {
                pos: self.pos,
                rip: pc,
                op: "write".to_string(),
                bits: 64,
                address: sp,
                old_value: self.maps.read_qword(sp).unwrap_or(0),
                new_value: value,
                name: "register".to_string(),
            };
            self.memory_operations.push(write_operation);
            log::trace!(
                "\tmem_trace: pos = {} rip = {:x} op = write bits = {} address = 0x{:x} value = 0x{:x} name = 'register'",
                self.pos,
                pc,
                64,
                sp,
                value
            );
        }

        self.set_sp(sp + 8);
        Some(value)
    }
}
