use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;

impl Emu {
    /// display 32bits main registers
    pub fn featured_regs32(&self) {
        self.regs().show_eax(&self.maps, 0);
        self.regs().show_ebx(&self.maps, 0);
        self.regs().show_ecx(&self.maps, 0);
        self.regs().show_edx(&self.maps, 0);
        self.regs().show_esi(&self.maps, 0);
        self.regs().show_edi(&self.maps, 0);
        log::trace!("\tesp: 0x{:x}", self.regs().get_esp() as u32);
        log::trace!("\tebp: 0x{:x}", self.regs().get_ebp() as u32);
        log::trace!("\teip: 0x{:x}", self.regs().get_eip() as u32);
    }

    /// display 64bits main registers
    pub fn featured_regs64(&self) {
        self.regs().show_rax(&self.maps, 0);
        self.regs().show_rbx(&self.maps, 0);
        self.regs().show_rcx(&self.maps, 0);
        self.regs().show_rdx(&self.maps, 0);
        self.regs().show_rsi(&self.maps, 0);
        self.regs().show_rdi(&self.maps, 0);
        log::trace!("\trsp: 0x{:x}", self.regs().rsp);
        log::trace!("\trbp: 0x{:x}", self.regs().rbp);
        log::trace!("\trip: 0x{:x}", self.regs().rip);
        self.regs().show_r8(&self.maps, 0);
        self.regs().show_r9(&self.maps, 0);
        self.regs().show_r10(&self.maps, 0);
        self.regs().show_r11(&self.maps, 0);
        self.regs().show_r12(&self.maps, 0);
        self.regs().show_r13(&self.maps, 0);
        self.regs().show_r14(&self.maps, 0);
        self.regs().show_r15(&self.maps, 0);
    }

    /// display aarch64 main registers
    pub fn featured_regs_aarch64(&self) {
        let regs = self.regs_aarch64();
        for i in (0..31).step_by(2) {
            if i + 1 < 31 {
                log::trace!(
                    "\tx{:<2}: 0x{:016x}  x{:<2}: 0x{:016x}",
                    i,
                    regs.x[i],
                    i + 1,
                    regs.x[i + 1]
                );
            } else {
                log::trace!("\tx{:<2}: 0x{:016x}", i, regs.x[i]);
            }
        }
        log::trace!("\tsp:  0x{:016x}", regs.sp);
        log::trace!("\tpc:  0x{:016x}", regs.pc);
        log::trace!("\tlr:  0x{:016x} (x30)", regs.x[30]);
        log::trace!("\tfp:  0x{:016x} (x29)", regs.x[29]);
        log::trace!(
            "\tNZCV: N={} Z={} C={} V={}",
            regs.nzcv.n,
            regs.nzcv.z,
            regs.nzcv.c,
            regs.nzcv.v
        );
    }

    #[inline]
    pub fn show_instruction_comment(
        &mut self,
        color: &str,
        ins: &DecodedInstruction,
        comment: &str,
    ) {
        if self.cfg.verbose < 2 {
            return;
        }
        let out = self.format_instruction(ins);
        let addr = if ins.is_x86() {
            ins.address()
        } else {
            self.pc()
        };
        if self.cfg.verbose >= 2 {
            if self.cfg.nocolors {
                log::trace!("{} 0x{:x}: {} ; {}", self.pos, addr, out, comment);
            } else {
                log::trace!(
                    "{}{} 0x{:x}: {} ; {}{}",
                    color,
                    self.pos,
                    addr,
                    out,
                    comment,
                    self.colors.nc
                );
            }
            self.show_definition();
        }
    }

    #[inline]
    pub fn show_instruction(&mut self, color: &str, ins: &DecodedInstruction) {
        if self.cfg.verbose < 2 {
            return;
        }
        let out = self.format_instruction(ins);
        let addr = if ins.is_x86() {
            ins.address()
        } else {
            self.pc()
        };
        if self.cfg.verbose >= 2 {
            if self.cfg.nocolors {
                log::trace!("{} 0x{:x}: {}", self.pos, addr, out);
            } else {
                log::trace!(
                    "{}{} 0x{:x}: {}{}",
                    color,
                    self.pos,
                    addr,
                    out,
                    self.colors.nc
                );
            }
            self.show_definition();
        }
    }

    #[inline]
    pub fn show_instruction_ret(&mut self, color: &str, ins: &DecodedInstruction, ret_addr: u64) {
        if self.cfg.verbose < 2 {
            return;
        }
        let out = self.format_instruction(ins);
        let addr = if ins.is_x86() {
            ins.address()
        } else {
            self.pc()
        };
        if self.cfg.verbose >= 2 {
            if self.cfg.nocolors {
                log::trace!(
                    "{} 0x{:x}: {} ; ret-addr: 0x{:x} ret-value: 0x{:x}",
                    self.pos,
                    addr,
                    out,
                    ret_addr,
                    self.regs().rax
                );
            } else {
                log::trace!(
                    "{}{} 0x{:x}: {} ; ret-addr: 0x{:x} ret-value: 0x{:x} {}",
                    color,
                    self.pos,
                    addr,
                    out,
                    ret_addr,
                    self.regs().rax,
                    self.colors.nc
                );
            }
            self.show_definition();
        }
    }

    #[inline]
    pub fn show_instruction_pushpop(&mut self, color: &str, ins: &DecodedInstruction, value: u64) {
        if self.cfg.verbose < 2 {
            return;
        }
        let out = self.format_instruction(ins);
        let addr = if ins.is_x86() {
            ins.address()
        } else {
            self.pc()
        };
        if self.cfg.verbose >= 2 {
            if self.cfg.nocolors {
                log::trace!("{} 0x{:x}: {} ;0x{:x}", self.pos, addr, out, value);
            } else {
                log::trace!(
                    "{}{} 0x{:x}: {} ;0x{:x} {}",
                    color,
                    self.pos,
                    addr,
                    out,
                    value,
                    self.colors.nc
                );
            }
            self.show_definition();
        }
    }

    #[inline]
    pub fn show_instruction_taken(&mut self, color: &str, ins: &DecodedInstruction) {
        if self.cfg.verbose < 2 {
            return;
        }
        let out = self.format_instruction(ins);
        let addr = if ins.is_x86() {
            ins.address()
        } else {
            self.pc()
        };
        if self.cfg.verbose >= 2 {
            if self.cfg.nocolors {
                log::trace!("{} 0x{:x}: {} taken", self.pos, addr, out);
            } else {
                log::trace!(
                    "{}{} 0x{:x}: {} taken {}",
                    color,
                    self.pos,
                    addr,
                    out,
                    self.colors.nc
                );
            }
            self.show_definition();
        }
    }

    pub fn show_instruction_not_taken(&mut self, color: &str, ins: &DecodedInstruction) {
        if self.cfg.verbose < 2 {
            return;
        }
        let out = self.format_instruction(ins);
        let addr = if ins.is_x86() {
            ins.address()
        } else {
            self.pc()
        };
        if self.cfg.verbose >= 2 {
            if self.cfg.nocolors {
                log::trace!("{} 0x{:x}: {} not taken", self.pos, addr, out);
            } else {
                log::trace!(
                    "{}{} 0x{:x}: {} not taken {}",
                    color,
                    self.pos,
                    addr,
                    out,
                    self.colors.nc
                );
            }
            self.show_definition();
        }
    }
}
