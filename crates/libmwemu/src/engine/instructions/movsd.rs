use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    let sz0 = emu.get_operand_sz(ins, 0);
    let sz1 = emu.get_operand_sz(ins, 1);

    // Iced shares this mnemonic between scalar-double and string forms.
    // Scalar memory accesses use 64 bits; memory loads also clear the upper half.
    if ins.op_count() == 2 && (sz0 == 128 || sz1 == 128) {
        emu.show_instruction(
            color!("LightCyan"),
            &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
        );

        if sz0 == 128 && sz1 == 128 {
            let src = match emu.get_operand_xmm_value_128(ins, 1, true) {
                Some(v) => v,
                None => return false,
            };
            let dst = match emu.get_operand_xmm_value_128(ins, 0, true) {
                Some(v) => v,
                None => return false,
            };
            let res = (dst & 0xffffffff_ffffffff_00000000_00000000) | (src & 0xffffffff_ffffffff);
            emu.set_operand_xmm_value_128(ins, 0, res)
        } else if sz0 == 128 {
            let src = match emu.get_operand_value(ins, 1, true) {
                Some(v) => v as u128,
                None => return false,
            };
            emu.set_operand_xmm_value_128(ins, 0, src & 0xffffffff_ffffffff)
        } else {
            let src = match emu.get_operand_xmm_value_128(ins, 1, true) {
                Some(v) => v,
                None => return false,
            };
            emu.set_operand_value(ins, 0, src as u64)
        }
    } else {
        // String form.

        if emu.rep.is_some() {
            if emu.rep.unwrap_or(0) == 0 {
                emu.show_instruction(
                    color!("LightCyan"),
                    &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
                );
            }
        } else {
            emu.show_instruction(
                color!("LightCyan"),
                &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
            );
        }

        if emu.cfg.is_x64() {
            let val = match emu.maps.read_dword(emu.regs().rsi) {
                Some(v) => v,
                None => {
                    log::trace!("movsd: cannot read memory at rsi=0x{:x}", emu.regs().rsi);
                    return false;
                }
            };

            let addr = emu.regs().rdi;
            let value = emu.call_memory_write_hook(addr, 32, val.into()) as u32;
            if !emu.maps.write_dword(addr, value) {
                return false;
            }

            if !emu.flag_df() {
                let v = emu.regs().rsi.wrapping_add(4);
                emu.regs_mut().rsi = v;
                let v = emu.regs().rdi.wrapping_add(4);
                emu.regs_mut().rdi = v;
            } else {
                let v = emu.regs().rsi.wrapping_sub(4);
                emu.regs_mut().rsi = v;
                let v = emu.regs().rdi.wrapping_sub(4);
                emu.regs_mut().rdi = v;
            }
        } else {
            let val = match emu.maps.read_dword(emu.regs().get_esi()) {
                Some(v) => v,
                None => {
                    log::trace!("cannot read memory at esi");
                    return false;
                }
            };
            let addr = emu.regs().get_edi();
            let value = emu.call_memory_write_hook(addr, 32, val.into()) as u32;
            if !emu.maps.write_dword(addr, value) {
                return false;
            }

            if !emu.flag_df() {
                let esi = emu.regs().get_esi().wrapping_add(4);
                let edi = emu.regs().get_edi().wrapping_add(4);
                emu.regs_mut().set_esi(esi);
                emu.regs_mut().set_edi(edi);
            } else {
                let esi = emu.regs().get_esi().wrapping_sub(4);
                let edi = emu.regs().get_edi().wrapping_sub(4);
                emu.regs_mut().set_esi(esi);
                emu.regs_mut().set_edi(edi);
            }
        }
        true
    }
}
