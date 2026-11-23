use crate::debug::console::Console;
use crate::emu::{Emu, decoded_instruction::DecodedInstruction};
use crate::exception::{
    seh32,
    types::{ExceptionType, MemoryAccess},
};
use crate::maps::mem64::Permission;
use crate::winapi::{winapi32, winapi64};
use crate::windows::constants::memory::*;

use crate::{color, exception::handlers as exception};
use iced_x86::{Code, Instruction, Mnemonic};

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.last_ret = (emu.pos, ins.ip());

    let ret_addr: u64 = if ins.mnemonic() == Mnemonic::Retf {
        let Some(target) = far_return_target(emu, ins) else {
            return false;
        };
        target
    } else if matches!(ins.code(), Code::Retnw | Code::Retnw_imm16) {
        let Some(target) = emu.stack_pop16(false) else {
            return false;
        };
        target as u64
    } else if emu.cfg.is_x64() {
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

    emu.show_instruction_ret(color!("Yellow"), &DecodedInstruction::X86(*ins), ret_addr);

    if emu.break_on_next_return {
        emu.break_on_next_return = false;
        Console::spawn_console(emu);
    }

    if ins.op_count() > 0 {
        let adjustment = ins.immediate16();
        if emu.cfg.is_x64() {
            emu.set_sp(emu.sp().wrapping_add(adjustment as u64));
        } else {
            let sp = (emu.sp() as u32).wrapping_add(adjustment as u32);
            emu.set_sp(sp as u64);
        }
    }

    if emu.eh_ctx() != 0
        && (seh32::handle_return(emu, ret_addr) || exception::handle_return(emu, ret_addr))
    {
        return true;
    }

    emu.call_stack_mut().pop();

    // Undo the SSDT shadow-space padding applied in `call.rs` whenever the
    // matching RET goes back to the PE-side caller. The pad stack only holds
    // RAs for PE→real-DLL transitions, so a hit here is unambiguous.
    if emu.cfg.is_x64()
        && emu.cfg.emulate_winapi
        && ret_addr < LIBS64_MIN
        && let Some(&expected) = emu.ssdt_pad_stack.last()
        && expected == ret_addr
    {
        emu.ssdt_pad_stack.pop();
        emu.regs_mut().rsp = emu.regs().rsp.wrapping_add(0x20);
    }

    // Host calls may return to an unmapped sentinel, including address zero.
    if emu.host_call_return == Some(ret_addr) {
        emu.regs_mut().rip = ret_addr;
        emu.force_reload = true;
        return true;
    }

    if emu.cfg.is_x64() {
        emu.set_rip_with_check(ret_addr, false)
    } else {
        emu.set_eip(ret_addr, false)
    }
}

/// RETF in a flat user code segment. A different selector requires a descriptor-table model.
fn far_return_target(emu: &mut Emu, ins: &Instruction) -> Option<u64> {
    let current_cs = emu.windows_i386_user_cs()?;
    let sp = emu.sp() as u32;
    let slot: u32 = match ins.code() {
        Code::Retfw | Code::Retfw_imm16 => size_of::<u16>() as u32,
        Code::Retfd | Code::Retfd_imm16 => size_of::<u32>() as u32,
        _ => return None,
    };
    let bytes = slot * 2;
    if sp.checked_add(bytes - 1).is_none()
        || !emu
            .maps
            .can_access_range(sp as u64, bytes as usize, Permission::READ)
    {
        emu.memory_exception(
            ExceptionType::PopfCannotReadStack,
            sp as u64,
            bytes as usize,
            MemoryAccess::Read,
        );
        return None;
    }
    let selector_slot = (sp + slot) as u64;
    let target = if slot == size_of::<u16>() as u32 {
        emu.maps.read_word(sp as u64).map(u64::from)?
    } else {
        emu.maps.read_dword(sp as u64).map(u64::from)?
    };
    let cs = emu.maps.read_word(selector_slot)?;
    if cs != current_cs {
        if cs & !3 == 0 {
            emu.exception(ExceptionType::BadAddressDereferencing);
        } else {
            log::warn!("unsupported far return from CS={current_cs:#x} to CS={cs:#x}");
        }
        return None;
    }
    emu.set_sp(sp.wrapping_add(slot * 2) as u64);
    Some(target)
}
