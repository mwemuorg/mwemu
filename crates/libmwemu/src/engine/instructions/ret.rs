use crate::debug::console::Console;
use crate::emu::Emu;
use crate::winapi::{winapi32, winapi64};
use crate::windows::constants::LIBS64_MIN;
use crate::{color, exception::handlers as exception};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    let ret_addr: u64 = if emu.cfg.is_x64() {
        match emu.stack_pop64(false) {
            Some(v) => v,
            None => return false,
        }
    } else {
        match emu.stack_pop32(false) {
            Some(v) => v as u64,
            None => return false,
        }
    };

    if emu.cfg.trace_calls {
        let dest_sym = if emu.cfg.is_x64() {
            winapi64::kernel32::guess_api_name(emu, ret_addr)
        } else {
            winapi32::kernel32::guess_api_name(emu, ret_addr as u32)
        };
        let dest_note = if dest_sym.is_empty() {
            String::new()
        } else {
            format!(" {}", dest_sym)
        };
        log::trace!(
            "{} 0x{:x} RET 0x{:x}{}",
            emu.pos,
            ins.ip(),
            ret_addr,
            dest_note
        );
    }

    emu.show_instruction_ret(
        color!("Yellow"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
        ret_addr,
    );

    if emu.break_on_next_return {
        emu.break_on_next_return = false;
        Console::spawn_console(emu);
    }

    if ins.op_count() > 0 {
        let arg = emu
            .get_operand_value(ins, 0, true)
            .expect("weird crash on ret");
        // apply stack compensation of ret operand

        if emu.cfg.is_x64() {
            if arg % 8 != 0 {
                log::trace!("weird ret argument!");
                return false;
            }

            emu.regs_mut().rsp += arg;
            //emu.stack_lvl[emu.stack_lvl_idx] -= arg as i32 / 8;
        } else {
            if arg % 4 != 0 {
                log::trace!("weird ret argument!");
                return false;
            }

            let esp = emu.regs().get_esp() + arg;
            emu.regs_mut().set_esp(esp);
            //emu.stack_lvl[emu.stack_lvl_idx] -= arg as i32 / 4;
        }
    }

    emu.call_stack_mut().pop();

    if emu.run_until_ret {
        return true;
    }

    if emu.eh_ctx() != 0 {
        exception::exit(emu);
        return true;
    }

    // Undo the SSDT shadow-space padding applied in `call.rs` whenever the
    // matching RET goes back to the PE-side caller. The pad stack only holds
    // RAs for PE→real-DLL transitions, so a hit here is unambiguous.
    if emu.cfg.is_x64() && emu.cfg.emulate_winapi && ret_addr < LIBS64_MIN {
        if let Some(&expected) = emu.ssdt_pad_stack.last() {
            if expected == ret_addr {
                emu.ssdt_pad_stack.pop();
                emu.regs_mut().rsp = emu.regs().rsp.wrapping_add(0x20);
            }
        }
    }

    if emu.cfg.is_x64() {
        emu.set_rip(ret_addr, false)
    } else {
        emu.set_eip(ret_addr, false)
    }
    //true
}
