use iced_x86::{Code, Instruction, Mnemonic};

use super::Emu;

impl Emu {
    /// Bulk fast-path for REP-prefixed string instructions. Returns `true` when
    /// it fully executed the whole REP (already advanced `rip`, `pos`,
    /// `instruction_count`, and the string registers/flags); `false` to let the
    /// per-element path run unchanged.
    ///
    /// The per-element loop in `run_loop` pays the full interpreter overhead
    /// (state tracing, the SSDT heap-list fixup, hook/breakpoint checks, decode
    /// re-entry) for EVERY element of a REP — so a `rep stos` of 100 KB costs
    /// ~12 500 loop iterations. Real loader code does huge `rep stosq`/`repe
    /// scasd` (heap fills/scans), which dominated Win11 SSDT init. This path
    /// performs the whole operation with one bulk memory access instead.
    ///
    /// Semantics are identical to the per-element path; only the redundant
    /// per-element work is removed. It engages ONLY when nothing needs to
    /// observe individual elements (no `-vvv`, traces, hooks, or memory
    /// watchpoints), only for x64, only forward (DF=0), and only for REPs large
    /// enough to be worth it. Anything else returns `false` and falls back.
    pub(crate) fn try_fast_rep_string(&mut self, ins: &Instruction, size: usize) -> bool {
        use Mnemonic::*;

        if !(ins.has_rep_prefix() || ins.has_repe_prefix() || ins.has_repne_prefix()) {
            return false;
        }
        // Element size by mnemonic. CMPS is intentionally left to the per-element
        // path (rare, and its operand/flag order is easy to get subtly wrong).
        let esz: u64 = match ins.mnemonic() {
            Stosb | Movsb | Scasb | Lodsb => 1,
            Stosw | Movsw | Scasw | Lodsw => 2,
            Stosd | Movsd | Scasd | Lodsd => 4,
            Stosq | Movsq | Scasq | Lodsq => 8,
            _ => return false,
        };

        // x64 + forward (DF=0) only. 32-bit and DF=1 fall back.
        if !self.cfg.is_x64() || self.flags().f_df {
            return false;
        }

        // Only in pure-execution mode: anything that observes individual elements
        // disables the fast path so debugging keeps byte-for-byte fidelity.
        if self.cfg.verbose >= 3
            || self.cfg.trace_mem
            || self.cfg.trace_regs
            || self.cfg.trace_reg
            || self.cfg.trace_flags
            || self.cfg.trace_string
            || self.cfg.inspect
            || self.cfg.entropy
            || self.hooks.hook_on_pre_instruction.is_some()
            || self.hooks.hook_on_post_instruction.is_some()
            || self.hooks.hook_on_memory_read.is_some()
            || self.hooks.hook_on_memory_write.is_some()
            || !self.bp.mem_read_addr.is_empty()
            || !self.bp.mem_write_addr.is_empty()
        {
            return false;
        }

        let count = self.regs().rcx;
        // Tiny REPs aren't worth the setup; the normal path also handles rcx==0
        // (it advances rip), so let it.
        if count < 16 {
            return false;
        }

        // Don't bulk past an instruction-count breakpoint / console / exit
        // position that lands strictly inside this REP — fall back so it stops.
        let lo = self.pos;
        let hi = self.pos + count;
        if self.exp != u64::MAX && self.exp > lo && self.exp < hi {
            return false;
        }
        if self.cfg.exit_position != 0 && self.cfg.exit_position > lo && self.cfg.exit_position < hi
        {
            return false;
        }
        if self.bp.instruction.iter().any(|&b| b > lo && b < hi) {
            return false;
        }

        let mask: u64 = if esz == 8 {
            u64::MAX
        } else {
            (1u64 << (esz * 8)) - 1
        };
        let le = |b: &[u8]| -> u64 {
            let mut v = 0u64;
            for (i, &x) in b.iter().enumerate() {
                v |= (x as u64) << (8 * i);
            }
            v
        };
        let di = self.regs().rdi;
        let si = self.regs().rsi;
        let total = (count * esz) as usize;
        let processed: u64;

        match ins.mnemonic() {
            Stosb | Stosw | Stosd | Stosq => {
                let end = di + total as u64 - 1;
                let writable = self
                    .maps
                    .get_mem_by_addr(di)
                    .map(|mm| mm.inside(end) && mm.can_write())
                    .unwrap_or(false);
                if !writable {
                    return false;
                }
                let unit = self.regs().rax.to_le_bytes();
                let unit = &unit[..esz as usize];
                let mut buf = Vec::with_capacity(total);
                for _ in 0..count {
                    buf.extend_from_slice(unit);
                }
                if let Some(mm) = self.maps.get_mem_by_addr_mut(di) {
                    mm.write_bytes(di, &buf);
                }
                self.regs_mut().rdi = di + total as u64;
                self.regs_mut().rcx = 0;
                processed = count;
            }
            Movsb | Movsw | Movsd | Movsq => {
                let dend = di + total as u64 - 1;
                let send = si + total as u64 - 1;
                // Overlapping forward copy has propagation semantics a flat
                // memcpy wouldn't reproduce — leave those to the per-element path.
                if di <= send && si <= dend {
                    return false;
                }
                let readable = self
                    .maps
                    .get_mem_by_addr(si)
                    .map(|mm| mm.inside(send) && mm.can_read())
                    .unwrap_or(false);
                let writable = self
                    .maps
                    .get_mem_by_addr(di)
                    .map(|mm| mm.inside(dend) && mm.can_write())
                    .unwrap_or(false);
                if !readable || !writable {
                    return false;
                }
                let src = self.maps.read_bytes(si, total).to_vec();
                if let Some(mm) = self.maps.get_mem_by_addr_mut(di) {
                    mm.write_bytes(di, &src);
                }
                self.regs_mut().rsi = si + total as u64;
                self.regs_mut().rdi = di + total as u64;
                self.regs_mut().rcx = 0;
                processed = count;
            }
            Scasb | Scasw | Scasd | Scasq => {
                let end = di + total as u64 - 1;
                let readable = self
                    .maps
                    .get_mem_by_addr(di)
                    .map(|mm| mm.inside(end) && mm.can_read())
                    .unwrap_or(false);
                if !readable {
                    return false;
                }
                let bytes = self.maps.read_bytes(di, total).to_vec();
                let acc = self.regs().rax & mask;
                let repne = ins.has_repne_prefix();
                let mut k = 0u64;
                let term = loop {
                    let off = (k * esz) as usize;
                    let elem = le(&bytes[off..off + esz as usize]);
                    k += 1;
                    let equal = acc == elem;
                    // repe/rep: stop on first mismatch; repne: stop on first match.
                    let stop = if repne { equal } else { !equal };
                    if stop || k == count {
                        break elem;
                    }
                };
                self.set_sub_flags(esz, acc, term);
                self.regs_mut().rcx = count - k;
                self.regs_mut().rdi = di + k * esz;
                processed = k;
            }
            Lodsb | Lodsw | Lodsd | Lodsq => {
                let send = si + total as u64 - 1;
                let readable = self
                    .maps
                    .get_mem_by_addr(si)
                    .map(|mm| mm.inside(send) && mm.can_read())
                    .unwrap_or(false);
                if !readable {
                    return false;
                }
                let off = ((count - 1) * esz) as usize;
                let bytes = self.maps.read_bytes(si, total);
                let last = le(&bytes[off..off + esz as usize]);
                self.set_acc(esz, last);
                self.regs_mut().rsi = si + total as u64;
                self.regs_mut().rcx = 0;
                processed = count;
            }
            _ => return false,
        }

        self.rep = None;
        self.pos += processed;
        self.instruction_count += processed;
        self.regs_mut().rip += size as u64;
        true
    }

    #[inline]
    fn set_sub_flags(&mut self, esz: u64, a: u64, b: u64) {
        match esz {
            1 => {
                self.flags_mut().sub8(a, b);
            }
            2 => {
                self.flags_mut().sub16(a, b);
            }
            4 => {
                self.flags_mut().sub32(a, b);
            }
            _ => {
                self.flags_mut().sub64(a, b);
            }
        }
    }

    #[inline]
    fn set_acc(&mut self, esz: u64, v: u64) {
        match esz {
            1 => self.regs_mut().set_al(v),
            2 => self.regs_mut().set_ax(v),
            4 => self.regs_mut().set_eax(v),
            _ => self.regs_mut().rax = v,
        }
    }

    pub(crate) fn handle_x86_rep_pre_execution(
        &mut self,
        instruction: Instruction,
        size: usize,
    ) -> bool {
        let is_ret = matches!(instruction.code(), Code::Retnw | Code::Retnd | Code::Retnq);
        let has_rep_prefix = instruction.has_rep_prefix()
            || instruction.has_repe_prefix()
            || instruction.has_repne_prefix();

        if is_ret || !has_rep_prefix {
            return false;
        }

        if self.rep.is_none() {
            self.rep = Some(0);
        }

        if self.regs().rcx == 0 {
            self.rep = None;
            if self.cfg.is_x64() {
                self.regs_mut().rip += size as u64;
            } else {
                let new_eip = self.regs().get_eip() + size as u64;
                self.regs_mut().set_eip(new_eip);
            }
            return true;
        }

        false
    }

    pub(crate) fn update_x86_rep_state_after_execution(&mut self, instruction: Instruction) {
        let Some(rep_count) = self.rep else {
            return;
        };

        if self.cfg.verbose >= 3 {
            log::trace!("    rcx: {}", self.regs().rcx);
        }

        if self.regs().rcx > 0 {
            self.regs_mut().rcx -= 1;
            if self.regs().rcx == 0 {
                self.rep = None;
            } else {
                self.rep = Some(rep_count + 1);
            }
        }

        let is_string_movement = matches!(
            instruction.mnemonic(),
            Mnemonic::Movsb
                | Mnemonic::Movsw
                | Mnemonic::Movsd
                | Mnemonic::Movsq
                | Mnemonic::Stosb
                | Mnemonic::Stosw
                | Mnemonic::Stosd
                | Mnemonic::Stosq
                | Mnemonic::Lodsb
                | Mnemonic::Lodsw
                | Mnemonic::Lodsd
                | Mnemonic::Lodsq
        );
        let is_string_comparison = matches!(
            instruction.mnemonic(),
            Mnemonic::Cmpsb
                | Mnemonic::Cmpsw
                | Mnemonic::Cmpsd
                | Mnemonic::Cmpsq
                | Mnemonic::Scasb
                | Mnemonic::Scasw
                | Mnemonic::Scasd
                | Mnemonic::Scasq
        );

        if is_string_movement {
            return;
        }

        if is_string_comparison {
            if instruction.has_repe_prefix() && !self.flag_zf() {
                self.rep = None;
            }
            if instruction.has_repne_prefix() && self.flag_zf() {
                self.rep = None;
            }
            return;
        }

        self.rep = None;
    }
}
