use crate::emu::decoded_instruction::DecodedInstruction;
use crate::emu::Emu;
use iced_x86::{Decoder, DecoderOptions, Instruction};
// about 10 mb should be on l3 cache
// 8192 cache lines,
// 64 instructions for each one,
// 40 for the struct (I think we can make it smaller)
const INSTRUCTION_ARRAY_SIZE: usize = 8192 * 64;

// we want the cache size to be store in L1 cache or L2 cache which is lower than 40kb
const CACHE_SIZE: usize = 2048 * MAX_CACHE_PER_LINE;
const CACHE_MASK: usize = CACHE_SIZE - 1; // Assumes power of 2
const MAX_CACHE_PER_LINE: usize = 32;

// we need INVALID_KEY and INVALID_LEN to be the same as INVALID_LPF_ADDR to optimize for memset
pub const INVALID_LPF_ADDR: u64 = 0xffffffffffffffff;
pub const INVALID_KEY: usize = 0xffffffffffffffff;
pub const INVALID_LEN: usize = 0xffffffffffffffff;

pub fn LPF_OF(addr: u64) -> u64 {
    // Implementation of LPF_OF macro/function
    addr & 0xfffffffffffff000
}

#[derive(Clone)]
struct CachedInstruction {
    pub lpf: u64,
    pub instruction_key: usize,
    pub instruction_len: usize,
}

impl Default for CachedInstruction {
    fn default() -> Self {
        CachedInstruction {
            lpf: INVALID_LPF_ADDR,
            instruction_key: INVALID_KEY,
            instruction_len: INVALID_LEN,
        }
    }
}

impl CachedInstruction {
    pub fn is_valid(&self) -> bool {
        self.lpf == INVALID_LPF_ADDR
    }
}

/// Decoded-instruction cache, generic over the native instruction type.
///
/// `T` is `iced_x86::Instruction` for x86 or
/// `yaxpeax_arm::armv8::a64::Instruction` for aarch64.  The cache itself
/// (address→slot mapping, linear probing, flush logic) is entirely
/// type-agnostic; only the insert helpers are arch-specific.
#[derive(Clone)]
pub struct InstructionCache<T: Copy + Default> {
    cache_entries: Vec<CachedInstruction>,
    instructions: Vec<T>,
    next_instruction_slot: usize,
    pub current_instruction_slot: usize,
    current_decode_len: usize,
    current_decode_idx: usize,
}

impl<T: Copy + Default> Default for InstructionCache<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy + Default> InstructionCache<T> {
    pub fn new() -> Self {
        InstructionCache {
            cache_entries: vec![CachedInstruction::default(); CACHE_SIZE],
            instructions: vec![T::default(); INSTRUCTION_ARRAY_SIZE],
            next_instruction_slot: 0,
            current_decode_len: 0,
            current_instruction_slot: 0,
            current_decode_idx: 0,
        }
    }

    #[inline(always)]
    pub fn get_index_of(&self, lpf: u64, len: u64) -> usize {
        const TLB_MASK: u32 = ((CACHE_SIZE - 1) << 12) as u32;
        (((lpf + len) & (TLB_MASK as u64)) >> 12) as usize
    }

    #[inline]
    pub fn flush_cache_line(&mut self, idx: usize) {
        for i in 0..MAX_CACHE_PER_LINE {
            self.cache_entries[idx + i].lpf = INVALID_LPF_ADDR;
        }
    }

    pub fn lookup_entry(&mut self, addr: u64, len: u64) -> bool {
        let lpf = crate::maps::tlb::LPF_OF(addr);
        let idx = self.get_index_of(lpf, len);

        // do a linear probing for each cache line
        for i in 0..MAX_CACHE_PER_LINE {
            if self.cache_entries[idx + i].lpf == INVALID_LPF_ADDR {
                return false;
            }
            // found the instruction now do initialization and return true
            if self.cache_entries[idx + i].lpf == addr {
                let key = self.cache_entries[idx + i].instruction_key;
                self.current_instruction_slot = key;
                self.current_decode_len = self.cache_entries[idx + i].instruction_len;
                self.current_decode_idx = 0;
                return true;
            }
        }

        // the cache_line is full now we flush all the cache line
        self.flush_cache_line(idx);
        false
    }

    #[inline(always)]
    fn flush_cache(&mut self) {
        self.cache_entries.iter_mut().for_each(|entry| {
            entry.lpf = INVALID_LPF_ADDR;
            entry.instruction_key = INVALID_KEY;
            entry.instruction_len = INVALID_LEN;
        });
        self.next_instruction_slot = 0;
    }

    /// Copy the next cached instruction into caller-owned storage.
    ///
    /// Hot-loop callers should use this plus the ISA-specific
    /// `decode_out_x86_into` / `decode_out_aarch64_into` to avoid
    /// touching the `DecodedInstruction` envelope.
    #[inline(always)]
    pub fn decode_out_into(&mut self, out: &mut T) {
        *out = self.instructions[self.current_instruction_slot + self.current_decode_idx];
        self.current_decode_idx += 1;
    }

    /// Generic return-by-value decode. Kept for non-hot callers;
    /// the typed-*_into helpers are preferred on the execution hot path.
    #[inline(always)]
    pub fn decode_out(&mut self) -> T {
        let mut value = T::default();
        self.decode_out_into(&mut value);
        value
    }

    pub fn can_decode(&self) -> bool {
        self.current_decode_idx < self.current_decode_len
    }

    /// Insert a single instruction at the given address.
    /// Used by the aarch64 path which decodes one instruction at a time.
    pub fn insert_single(&mut self, addr: u64, instruction: T) {
        let lpf = crate::maps::tlb::LPF_OF(addr);
        let idx = self.get_index_of(lpf, 0);

        let slot = self.next_instruction_slot;
        if slot + 1 > INSTRUCTION_ARRAY_SIZE {
            self.flush_cache();
        }

        self.instructions[self.next_instruction_slot] = instruction;
        self.next_instruction_slot += 1;

        for i in 0..MAX_CACHE_PER_LINE {
            if self.cache_entries[idx + i].lpf == INVALID_LPF_ADDR {
                self.cache_entries[idx + i].instruction_key = slot;
                self.cache_entries[idx + i].lpf = addr;
                self.cache_entries[idx + i].instruction_len = 1;
                break;
            }
        }
    }
}

// --- x86 and aarch64-specific insert methods on the unified cache ---

impl InstructionCache<DecodedInstruction> {
    /// Decode a basic block from an iced-x86 decoder and insert each
    /// instruction as `DecodedInstruction::X86(_)`.
    pub fn insert_x86_from_decoder(
        &mut self,
        decoder: &mut Decoder<'_>,
        addition: usize,
        rip_addr: u64,
    ) {
        let lpf = crate::maps::tlb::LPF_OF(rip_addr);
        let idx = self.get_index_of(lpf, 0);
        let slot = self.next_instruction_slot;
        let mut count: usize = 0;
        let max_position = decoder.max_position();
        if max_position + self.next_instruction_slot > INSTRUCTION_ARRAY_SIZE {
            self.flush_cache();
        }
        while decoder.can_decode() && decoder.position() + addition <= max_position {
            let mut native = Instruction::default();
            decoder.decode_out(&mut native);
            let ins = DecodedInstruction::X86(native);
            self.instructions[slot + count] = ins;
            let temp = native;
            if temp.is_jmp_short_or_near()
                || temp.is_jmp_near_indirect()
                || temp.is_jmp_far()
                || temp.is_jmp_far_indirect()
                || temp.is_jcc_short_or_near()
                || temp.is_call_near_indirect()
                || temp.is_call_near()
                || temp.is_call_far_indirect()
                || temp.is_call_far()
            {
                count += 1;
                break;
            }
            count += 1;
        }
        self.next_instruction_slot += count;
        for i in 0..MAX_CACHE_PER_LINE {
            if self.cache_entries[idx + i].lpf == INVALID_LPF_ADDR {
                self.cache_entries[idx + i].instruction_key = slot;
                self.cache_entries[idx + i].lpf = rip_addr;
                self.cache_entries[idx + i].instruction_len = count;
                break;
            }
        }
        let inserted = self.lookup_entry(rip_addr, 0);
        debug_assert!(
            inserted,
            "x86 cache insertion failed at 0x{:x}",
            rip_addr
        );
    }

    /// Decode a basic block of aarch64 instructions from the given byte
    /// slice and insert them as `DecodedInstruction::AArch64(_)`.
    pub fn insert_aarch64_from_block(&mut self, block: &[u8], pc_addr: u64) {
        let lpf = crate::maps::tlb::LPF_OF(pc_addr);
        let idx = self.get_index_of(lpf, 0);
        let slot = self.next_instruction_slot;
        let mut count: usize = 0;
        let decoder = yaxpeax_arm::armv8::a64::InstDecoder::default();
        let mut offset: usize = 0;
        while offset + 4 <= block.len() {
            if slot + count >= INSTRUCTION_ARRAY_SIZE {
                self.flush_cache();
                return;
            }
            let chunk = &block[offset..offset + 4];
            let mut reader = yaxpeax_arch::U8Reader::new(chunk);
            let native = match yaxpeax_arch::Decoder::decode(&decoder, &mut reader) {
                Ok(ins) => ins,
                Err(_) => break,
            };
            self.instructions[slot + count] = DecodedInstruction::AArch64(native);
            count += 1;
            offset += 4;
            use yaxpeax_arm::armv8::a64::Opcode;
            match native.opcode {
                Opcode::RET
                | Opcode::B
                | Opcode::BR
                | Opcode::BL
                | Opcode::BLR
                | Opcode::Bcc(_)
                | Opcode::CBZ
                | Opcode::CBNZ
                | Opcode::TBZ
                | Opcode::TBNZ => break,
                _ => {}
            }
        }
        if count == 0 {
            return;
        }
        self.next_instruction_slot += count;
        for i in 0..MAX_CACHE_PER_LINE {
            if self.cache_entries[idx + i].lpf == INVALID_LPF_ADDR {
                self.cache_entries[idx + i].instruction_key = slot;
                self.cache_entries[idx + i].lpf = pc_addr;
                self.cache_entries[idx + i].instruction_len = count;
                break;
            }
        }
        let inserted = self.lookup_entry(pc_addr, 0);
        debug_assert!(inserted, "aarch64 cache insertion failed at 0x{:x}", pc_addr);
    }

    /// Copy the next cached x86 instruction into caller-owned storage.
    ///
    /// The cache slot must hold a `DecodedInstruction::X86`. Used by the hot
    /// x86 execution loop, which owns a reusable `iced_x86::Instruction`
    /// local. Avoids the intermediate `DecodedInstruction` value the
    /// generic `decode_out` returns.
    #[inline(always)]
    pub fn decode_out_x86_into(&mut self, out: &mut iced_x86::Instruction) {
        let index = self.current_instruction_slot + self.current_decode_idx;
        match self.instructions[index] {
            DecodedInstruction::X86(ins) => *out = ins,
            DecodedInstruction::AArch64(_) => {
                unreachable!("x86 cache slot contains an AArch64 instruction")
            }
        }
        self.current_decode_idx += 1;
    }

    /// Copy the next cached AArch64 instruction into caller-owned storage.
    ///
    /// The cache slot must hold a `DecodedInstruction::AArch64`. Used by
    /// the hot AArch64 execution loop, which owns a reusable
    /// `yaxpeax_arm::armv8::a64::Instruction` local.
    #[inline(always)]
    pub fn decode_out_aarch64_into(
        &mut self,
        out: &mut yaxpeax_arm::armv8::a64::Instruction,
    ) {
        let index = self.current_instruction_slot + self.current_decode_idx;
        match self.instructions[index] {
            DecodedInstruction::AArch64(ins) => *out = ins,
            DecodedInstruction::X86(_) => {
                unreachable!("AArch64 cache slot contains an x86 instruction")
            }
        }
        self.current_decode_idx += 1;
    }
}
impl Emu {
    /// Disassemble instructions at the given address.
    /// Works for both x86 and aarch64.
    pub fn disassemble(&mut self, addr: u64, amount: u32) -> String {
        if self.cfg.arch.is_aarch64() {
            return self.disassemble_aarch64(addr, amount);
        }

        let mut out = String::new();
        let code = self.maps.get_mem_by_addr(addr).expect("address not mapped");
        let block = code.read_from(addr).to_vec();

        let bits: u32 = if self.cfg.is_x64() { 64 } else { 32 };
        let mut decoder = Decoder::with_ip(bits, &block, addr, DecoderOptions::NONE);
        let mut instruction = Instruction::default();
        let mut count: u32 = 1;
        while decoder.can_decode() {
            decoder.decode_out(&mut instruction);
            let output = self.x86_format_instruction(&instruction);
            if self.cfg.is_x64() {
                out.push_str(&format!("0x{:x}: {}\n", instruction.ip(), output));
            } else {
                out.push_str(&format!("0x{:x}: {}\n", instruction.ip32(), output));
            }
            count += 1;
            if count == amount {
                break;
            }
        }
        out
    }

    /// Disassemble aarch64 instructions at the given address.
    fn disassemble_aarch64(&self, addr: u64, amount: u32) -> String {
        let mut out = String::new();
        let code = self.maps.get_mem_by_addr(addr).expect("address not mapped");
        let block = code.read_from(addr).to_vec();

        let decoder = yaxpeax_arm::armv8::a64::InstDecoder::default();
        let mut pc = addr;
        let mut count: u32 = 0;
        let mut offset: usize = 0;

        while offset + 4 <= block.len() && count < amount {
            let chunk = &block[offset..offset + 4];
            let mut reader = yaxpeax_arch::U8Reader::new(chunk);
            match yaxpeax_arch::Decoder::decode(&decoder, &mut reader) {
                Ok(ins) => {
                    out.push_str(&format!("0x{:x}: {}\n", pc, ins));
                }
                Err(e) => {
                    out.push_str(&format!("0x{:x}: <decode error: {:?}>\n", pc, e));
                }
            }
            pc += 4;
            offset += 4;
            count += 1;
        }
        out
    }
}
