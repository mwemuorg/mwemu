use iced_x86::{Decoder, DecoderError, DecoderOptions, Instruction};

use crate::emu::Emu;
use crate::engine;
use crate::err::MwemuError;
use crate::exception::types::{ExceptionType, MemoryAccess};
use crate::windows::constants::memory::BLOCK_LEN;

use super::assert_x86_arch;

const MAX_X86_INSTRUCTION_LEN: usize = 15;

/// x86-family cache fill. Performs the cache hit/miss check, reads a block
/// of code from the maps, and inserts the decoded instructions into the
/// x86 instruction cache. Also resets the REP prefix state on a miss.
/// Panics if the configured architecture is AArch64.
pub(crate) fn ensure_instruction_cache_populated_x86(
    emu: &mut Emu,
    pc: u64,
    block: &mut Vec<u8>,
    arch_bits: u32,
) -> Result<(), MwemuError> {
    // Self-modifying code: writes that hit executable memory since the last
    // drain (an API stub, a syscall, the console) must not be served stale.
    if emu.maps.has_code_writes() {
        let mut scratch = Vec::new();
        emu.drain_code_writes_x86(&mut scratch);
    }

    let cache_hit = emu.x86_instruction_cache().lookup_entry(pc, 0);
    if cache_hit
        && emu.maps.get_mem_by_addr(pc).is_some_and(|mem| {
            mem.read_code(pc, MAX_X86_INSTRUCTION_LEN).len() == MAX_X86_INSTRUCTION_LEN
        })
    {
        return Ok(());
    }

    // Near a map boundary, recheck continuation bytes even on a cache hit.
    if !emu.fetch_x86_code(pc, arch_bits, BLOCK_LEN, block) {
        return Err(MwemuError::new(
            "instruction fetch from inaccessible memory",
        ));
    }

    if cache_hit {
        return Ok(());
    }

    let mut decoder = Decoder::with_ip(arch_bits, block, pc, DecoderOptions::NONE);
    emu.rep = None;
    let addition = block.len().min(16);
    emu.x86_instruction_cache()
        .insert_x86_from_decoder(&mut decoder, addition, pc);

    Ok(())
}

impl Emu {
    /// Read a block from the current map. Only the first instruction may need
    /// bytes from the next map. Speculative lookahead must never raise a fault.
    fn fetch_x86_code(&mut self, pc: u64, bits: u32, limit: usize, bytes: &mut Vec<u8>) -> bool {
        bytes.clear();
        let mut cursor = pc;
        let mut remaining = limit;
        while let Some(mem) = self.maps.get_mem_by_addr(cursor) {
            let chunk = mem.read_code(cursor, remaining);
            if chunk.is_empty() {
                break;
            }
            bytes.extend_from_slice(&chunk);
            if bytes.len() >= MAX_X86_INSTRUCTION_LEN {
                return true;
            }
            remaining = MAX_X86_INSTRUCTION_LEN - bytes.len();
            cursor = pc.saturating_add(bytes.len() as u64);
        }

        // A short buffer can still contain a complete (possibly illegal) opcode.
        let mut decoder = Decoder::with_ip(bits, bytes, pc, DecoderOptions::NONE);
        let _ = decoder.decode();
        if decoder.last_error() != DecoderError::NoMoreBytes {
            return true;
        }

        self.memory_exception(
            ExceptionType::SettingRipToNonMappedAddr,
            cursor,
            1,
            MemoryAccess::Execute,
        );
        false
    }

    pub(super) fn fetch_x86_instruction(&mut self, pc: u64) -> Option<Instruction> {
        let bits = if self.cfg.is_x64() { 64 } else { 32 };
        let mut bytes = Vec::with_capacity(MAX_X86_INSTRUCTION_LEN);
        if !self.fetch_x86_code(pc, bits, MAX_X86_INSTRUCTION_LEN, &mut bytes) {
            return None;
        }
        Some(Decoder::with_ip(bits, &bytes, pc, DecoderOptions::NONE).decode())
    }

    /// Apply the executable-memory writes `Maps` recorded since the last drain
    /// to the decoded-instruction cache. Returns `true` when a write touched an
    /// instruction of the block currently being executed from the cache, so the
    /// caller must stop decoding from it and refetch from the current PC.
    pub(crate) fn drain_code_writes_x86(&mut self, scratch: &mut Vec<(u64, u64)>) -> bool {
        self.maps.drain_code_writes(scratch);
        let mut refetch = false;
        for &(addr, len) in scratch.iter() {
            if len == u64::MAX {
                self.x86_instruction_cache().invalidate_all();
                return true;
            }
            let end = addr.saturating_add(len);
            let cache = self.x86_instruction_cache();
            cache.invalidate_range(addr, len, BLOCK_LEN as u64);
            if cache.pending_x86_overlaps(addr, end) {
                refetch = true;
            }
        }
        refetch
    }

    /// Address of an ntdll export in the mapped ntdll, or 0 when it is not
    /// available (non-Windows guest, 32-bit, ntdll not mapped). Export RVAs are
    /// build-specific, so callers must never hardcode them.
    pub(crate) fn resolve_ntdll_export(&self, name: &str) -> u64 {
        if !self.cfg.is_x64() || !self.os.is_windows() {
            return 0;
        }
        self.export_indexes
            .resolve_name_in_module("ntdll.dll", name)
    }
}

/// x86-family variant of `decode_and_execute`. Panics if the configured
/// architecture is AArch64.
pub(crate) fn decode_and_execute_x86(emu: &mut Emu) -> (usize, bool) {
    if emu.api_stopped() {
        return (0, false);
    }
    assert_x86_arch(emu, "decode_and_execute_x86");
    let pc = emu.pc();

    let Some(ins) = emu.fetch_x86_instruction(pc) else {
        return (0, false);
    };
    emu.memory_operations.clear();
    let sz = ins.len();

    emu.set_x86_instruction(Some(ins));
    let decoded = emu.last_decoded_x86(pc, ins);

    // Pre-instruction hook
    if let Some(mut hook_fn) = emu.hooks.hook_on_pre_instruction.take() {
        let skip = !hook_fn(emu, pc, &decoded, sz);
        emu.hooks.hook_on_pre_instruction = Some(hook_fn);
        if skip {
            return (sz, true); // skip instruction emulation but report as successful
        }
    }

    let result_ok = if Emu::is_rep_string_instruction(&ins) {
        emu.execute_x86_rep_fully(&ins, sz)
    } else {
        engine::emulate_instruction(emu, &ins, sz, true)
    };
    emu.last_instruction_size = sz;

    // Post-instruction hook
    if let Some(mut hook_fn) = emu.hooks.hook_on_post_instruction.take() {
        hook_fn(emu, pc, &decoded, sz, result_ok);
        emu.hooks.hook_on_post_instruction = Some(hook_fn);
    }

    (sz, result_ok)
}

/// x86-family variant of `advance_pc`. Respects `force_reload` and then
/// advances RIP (64-bit) or EIP (32-bit) by `sz` bytes. Panics on AArch64.
#[inline]
pub(crate) fn advance_pc_x86(emu: &mut Emu, sz: usize) {
    if emu.api_stopped() {
        return;
    }
    assert_x86_arch(emu, "advance_pc_x86");
    if emu.force_reload {
        emu.force_reload = false;
    } else if emu.cfg.is_x64() {
        emu.regs_mut().rip += sz as u64;
    } else {
        let eip = emu.regs().get_eip() + sz as u64;
        emu.regs_mut().set_eip(eip);
    }
}
