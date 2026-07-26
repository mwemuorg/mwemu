use crate::debug::console::Console;
use crate::emu::Emu;
use crate::engine;
use crate::err::MwemuError;
use crate::windows::constants;

/// AArch64 cache fill. Performs the cache hit/miss check, reads a block
/// of code from the maps, and inserts the fixed-width decoded instructions
/// into the AArch64 instruction cache. Panics if the configured
/// architecture is not AArch64.
pub(crate) fn ensure_instruction_cache_populated_aarch64(
    emu: &mut Emu,
    pc: u64,
    block: &mut Vec<u8>,
) -> Result<(), MwemuError> {
    super::super::assert_aarch64_arch(emu, "ensure_instruction_cache_populated_aarch64");

    let cache_hit = emu.aarch64_instruction_cache().lookup_entry(pc, 0);
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

    emu.aarch64_instruction_cache()
        .insert_aarch64_from_block(block, pc);

    Ok(())
}

/// AArch64 variant of `decode_and_execute`. Panics if the configured
/// architecture is not AArch64.
pub(crate) fn decode_and_execute_aarch64(emu: &mut Emu) -> (usize, bool) {
    super::super::assert_aarch64_arch(emu, "decode_and_execute_aarch64");
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

    let block = code.read_bytes(pc, 4);
    if block.len() < 4 {
        log::warn!("aarch64: cannot read 4 bytes at 0x{:x}", pc);
        return (0, false);
    }

    let decoder = yaxpeax_arm::armv8::a64::InstDecoder::default();
    let mut reader = yaxpeax_arch::U8Reader::new(block);
    let ins = match yaxpeax_arch::Decoder::decode(&decoder, &mut reader) {
        Ok(ins) => ins,
        Err(e) => {
            log::warn!("aarch64: decode error at 0x{:x}: {:?}", pc, e);
            return (0, false);
        }
    };

    if emu.cfg.verbose >= 2 {
        log::trace!("{} 0x{:x}: {}", emu.pos, pc, ins);
    }

    let decoded = emu.last_decoded_aarch64(pc, ins);

    // Pre-instruction hook
    if let Some(mut hook_fn) = emu.hooks.hook_on_pre_instruction.take() {
        let skip = !hook_fn(emu, pc, &decoded, 4);
        emu.hooks.hook_on_pre_instruction = Some(hook_fn);
        if skip {
            return (4, true); // skip instruction emulation but report as successful
        }
    }

    let result_ok = engine::aarch64::emulate_instruction(emu, &ins);
    emu.last_instruction_size = 4;

    // Post-instruction hook
    if let Some(mut hook_fn) = emu.hooks.hook_on_post_instruction.take() {
        hook_fn(emu, pc, &decoded, 4, result_ok);
        emu.hooks.hook_on_post_instruction = Some(hook_fn);
    }

    (4, result_ok)
}

/// AArch64 variant of `advance_pc`. Respects `force_reload` and otherwise
/// advances PC by `sz` bytes (normal decoders always pass 4). Panics on x86.
#[inline]
pub(crate) fn advance_pc_aarch64(emu: &mut Emu, sz: usize) {
    super::super::assert_aarch64_arch(emu, "advance_pc_aarch64");
    if emu.force_reload {
        emu.force_reload = false;
    } else {
        emu.regs_aarch64_mut().pc += sz as u64;
    }
}
