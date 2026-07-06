use crate::color;
use crate::emu::Emu;
use crate::winapi::{winapi32, winapi64};
use crate::windows::constants::{LIBS64_MAX, LIBS64_MIN};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Yellow"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    if ins.op_count() != 1 {
        unimplemented!("weird variant of call");
    }

    let addr = match emu.get_jump_value(ins, 0) {
        Some(a) => a,
        None => return false,
    };

    if emu.cfg.trace_calls {
        let callee_sym = if emu.cfg.is_x64() {
            winapi64::kernel32::guess_api_name(emu, addr)
        } else {
            winapi32::kernel32::guess_api_name(emu, addr as u32)
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
    }

    if emu.regs_mut().rip == addr - 5
        && emu.cfg.verbose >= 1 {
            log::trace!("call next instruction, prolly call/pop");
        }
        //emu.stack_lvl[emu.stack_lvl_idx] -= 1;
     /*else {
    emu.stack_lvl.push(0);
    emu.stack_lvl_idx += 1;
    }*/

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
        if pad_call {
            emu.regs_mut().rsp = emu.regs().rsp.wrapping_sub(0x20);
            let expected_ra = rip + instruction_sz as u64;
            emu.ssdt_pad_stack.push(expected_ra);
        }
        if !emu.stack_push64(emu.regs().rip + instruction_sz as u64) {
            return false;
        }
        emu.set_rip(addr, false)
    } else {
        if !emu.stack_push32(emu.regs().get_eip() as u32 + instruction_sz as u32) {
            return false;
        }
        emu.set_eip(addr, false)
    }
    //true
}
