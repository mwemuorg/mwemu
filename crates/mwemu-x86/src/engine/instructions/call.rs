use crate::color;
use crate::emu::Emu;
use crate::utils::helpers::unlikely;
use crate::windows::constants::memory::*;

use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Yellow"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let addr = match emu.get_jump_value(ins, 0) {
        Some(a) => a,
        None => return false,
    };

    if emu.cfg.trace_calls {
        let callee_sym = if emu.cfg.is_x64() {
            crate::layers::windows().guess_api_name(emu, addr, true)
        } else {
            crate::layers::windows().guess_api_name(emu, addr as u32 as u64, false)
        };
        let callee_note = if callee_sym.is_empty() {
            String::new()
        } else {
            format!(" {}", callee_sym)
        };
        log::trace!(
            "{} 0x{:x} CALL 0x{:x}{} (0x{:x}, 0x{:x}, 0x{:x})",
            emu.pos,
            emu.regs().rip,
            addr,
            callee_note,
            emu.regs().rcx,
            emu.regs().rdx,
            emu.regs().r8
        );
        // Keep a queryable log of named calls alongside the log::trace! line
        // above, so a caller that isn't reading the trace log (e.g. mwemu-mcp)
        // can still see which functions got called by name.
        if !callee_sym.is_empty() {
            let pos = emu.pos;
            let from = emu.regs().rip;
            emu.log_api_call(pos, from, addr, callee_sym);
        }
    }

    let rip = emu.regs().rip;
    emu.call_stack_mut().push((rip, addr));

    if emu.cfg.is_x64() {
        // SSDT shadow-space padding: real-Windows DLLs use the caller's 32-byte
        // home space (`[rsp+0x00..rsp+0x20]`) as scratch (e.g. KernelBase spills
        // `rsi` to `[rsp+0x10]`). Hand-written PE x64 binaries that store data
        // inside what would be their *callee's* home — like exe64win_msgbox,
        // which keeps a saved RSP at `[rbp-0x48]` directly overlapping the R8
        // home — get those locals trashed by the very first DLL call. Shift
        // the home-space window down by 0x20 only on PE→real-DLL transitions;
        // a matching RET (back to PE) unwinds the pad in `ret.rs`.
        let pad_call = emu.cfg.emulate_winapi
            && rip < LIBS64_MIN
            && (LIBS64_MIN..=LIBS64_MAX).contains(&addr)
            && emu.maps.is_mapped(addr);
        let stack_before = emu.sp();
        let pad_depth = emu.ssdt_pad_stack.len();
        if unlikely(pad_call) {
            emu.regs_mut().rsp = emu.regs().rsp.wrapping_sub(0x20);
            let expected_ra = rip + instruction_sz as u64;
            emu.ssdt_pad_stack.push(expected_ra);
        }
        if !emu.push_instruction(
            emu.regs().rip.wrapping_add(instruction_sz as u64),
            u64::BITS,
        ) {
            emu.set_sp(stack_before);
            emu.ssdt_pad_stack.truncate(pad_depth);
            return false;
        }
        emu.set_rip_with_check(addr, false)
    } else {
        if !emu.push_instruction(
            emu.regs().get_eip().wrapping_add(instruction_sz as u64),
            u32::BITS,
        ) {
            return false;
        }
        emu.set_eip(addr, false)
    }
}
