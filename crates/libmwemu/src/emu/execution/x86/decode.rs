use iced_x86::{Decoder, DecoderOptions};

use crate::debug::console::Console;
use crate::emu::Emu;
use crate::engine;
use crate::err::MwemuError;
use crate::windows::constants;

use super::assert_x86_arch;

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
    if cache_hit {
        return Ok(());
    }

    if block.is_empty() {
        return Err(MwemuError::new("cannot read code block, weird address."));
    }

    let code = match emu.maps.get_mem_by_addr(pc) {
        Some(code) => code,
        None => {
            log::trace!("code flow to unmapped address 0x{:x}", pc);
            Console::spawn_console(emu);
            return Err(MwemuError::new("cannot read program counter"));
        }
    };

    let block_slice = code.read_bytes(pc, constants::BLOCK_LEN);
    if block_slice.len() != block.len() {
        block.resize(block_slice.len(), 0);
    }
    block.clone_from_slice(block_slice);

    let mut decoder = Decoder::with_ip(arch_bits, block, pc, DecoderOptions::NONE);
    emu.rep = None;
    let addition = block.len().min(16);
    emu.x86_instruction_cache()
        .insert_x86_from_decoder(&mut decoder, addition, pc);

    Ok(())
}

impl Emu {
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
            cache.invalidate_range(addr, len, constants::BLOCK_LEN as u64);
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
    assert_x86_arch(emu, "decode_and_execute_x86");
    let pc = emu.pc();

    // Fetch code
    let code = match emu.maps.get_mem_by_addr(pc) {
        Some(c) => c,
        None => {
            log::trace!("code flow to unmapped address 0x{:x}", pc);
            Console::spawn_console(emu);
            return (0, false);
        }
    };

    emu.memory_operations.clear();

    let block = code.read_from(pc).to_vec();
    let mut decoder = if emu.cfg.is_x64() {
        Decoder::with_ip(64, &block, pc, DecoderOptions::NONE)
    } else {
        Decoder::with_ip(32, &block, pc, DecoderOptions::NONE)
    };

    let ins = decoder.decode();
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
