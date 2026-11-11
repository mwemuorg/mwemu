use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    // SSE2 CMPSD (scalar-double compare) shares this mnemonic with the string
    // CMPSD; the SSE form is the one with three operands (xmm, xmm/m64, imm8).
    if ins.op_count() == 3 {
        emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
        let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
        let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
        let pred = try_operand!(emu.get_operand_value(ins, 2, true)) as u8 & 7;
        let a = f64::from_bits((dest & 0xffff_ffff_ffff_ffff) as u64);
        let b = f64::from_bits((src & 0xffff_ffff_ffff_ffff) as u64);
        let t = match pred {
            0 => a == b,
            1 => a < b,
            2 => a <= b,
            3 => a.is_nan() || b.is_nan(),
            4 => !(a == b),
            5 => !(a < b),
            6 => !(a <= b),
            _ => !a.is_nan() && !b.is_nan(),
        };
        let low: u128 = if t { 0xffff_ffff_ffff_ffff } else { 0 };
        let result = (dest & !(0xffff_ffff_ffff_ffffu128)) | low;
        return emu.set_operand_xmm_value_128(ins, 0, result);
    }

    let value0: u32;
    let value1: u32;

    if emu.rep.is_some() {
        if emu.rep.unwrap_or(0) == 0 || emu.cfg.verbose >= 3 {
            emu.show_instruction(color!("LightCyan"), &DecodedInstruction::X86(*ins));
        }
    } else {
        emu.show_instruction(color!("LightCyan"), &DecodedInstruction::X86(*ins));
    }

    if emu.cfg.is_x64() {
        value0 = match emu.maps.read_dword(emu.regs().rsi) {
            Some(v) => v,
            None => {
                log::trace!("cannot read rsi");
                return false;
            }
        };
        value1 = match emu.maps.read_dword(emu.regs().rdi) {
            Some(v) => v,
            None => {
                log::trace!("cannot read rdi");
                return false;
            }
        };

        if emu.flag_df() {
            let v = emu.regs().rsi.wrapping_sub(4);
            emu.regs_mut().rsi = v;
            let v = emu.regs().rdi.wrapping_sub(4);
            emu.regs_mut().rdi = v;
        } else {
            let v = emu.regs().rsi.wrapping_add(4);
            emu.regs_mut().rsi = v;
            let v = emu.regs().rdi.wrapping_add(4);
            emu.regs_mut().rdi = v;
        }
    } else {
        value0 = match emu.maps.read_dword(emu.regs().get_esi()) {
            Some(v) => v,
            None => {
                log::trace!("cannot read esi");
                return false;
            }
        };
        value1 = match emu.maps.read_dword(emu.regs().get_edi()) {
            Some(v) => v,
            None => {
                log::trace!("cannot read edi");
                return false;
            }
        };

        if emu.flag_df() {
            let esi = emu.regs().get_esi().wrapping_sub(4);
            let edi = emu.regs().get_edi().wrapping_sub(4);
            emu.regs_mut().set_esi(esi);
            emu.regs_mut().set_edi(edi);
        } else {
            let esi = emu.regs().get_esi().wrapping_add(4);
            let edi = emu.regs().get_edi().wrapping_add(4);
            emu.regs_mut().set_esi(esi);
            emu.regs_mut().set_edi(edi);
        }
    }

    emu.flags_overwrite_mut()
        .sub32(value0 as u64, value1 as u64);

    if emu.cfg.verbose >= 2 {
        if value0 > value1 {
            log::trace!("\tcmp: 0x{:x} > 0x{:x}", value0, value1);
        } else if value0 < value1 {
            log::trace!("\tcmp: 0x{:x} < 0x{:x}", value0, value1);
        } else {
            log::trace!("\tcmp: 0x{:x} == 0x{:x}", value0, value1);
        }
    }
    true
}
