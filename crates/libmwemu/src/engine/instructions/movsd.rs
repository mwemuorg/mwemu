use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    let sz0 = emu.get_operand_sz(ins, 0);
    let sz1 = emu.get_operand_sz(ins, 1);

    // SSE scalar form `MOVSD` (iced shares `Mnemonic::Movsd` with the string
    // MOVSD; the scalar-double form always has an xmm/128-bit operand). The m64
    // memory operand is NOT a 128-bit value: treating it as one made the store
    // write 16 bytes (clobbering the adjacent qword — e.g. a stack GS cookie)
    // and the load pull garbage into the upper 64 bits instead of zeroing them.
    if ins.op_count() == 2 && (sz0 == 128 || sz1 == 128) {
        emu.show_instruction(
            color!("LightCyan"),
            &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
        );

        if sz0 == 128 && sz1 == 128 {
            // movsd xmm0, xmm1 — copy the low 64 bits, leave dst's high 64 intact.
            let src = match emu.get_operand_xmm_value_128(ins, 1, true) {
                Some(v) => v,
                None => return false,
            };
            let dst = match emu.get_operand_xmm_value_128(ins, 0, true) {
                Some(v) => v,
                None => return false,
            };
            let res = (dst & 0xffffffff_ffffffff_00000000_00000000) | (src & 0xffffffff_ffffffff);
            emu.set_operand_xmm_value_128(ins, 0, res);
        } else if sz0 == 128 {
            // movsd xmm0, m64 — load the low 64 bits and ZERO the upper 64 bits.
            let src = match emu.get_operand_value(ins, 1, true) {
                Some(v) => v as u128,
                None => return false,
            };
            emu.set_operand_xmm_value_128(ins, 0, src & 0xffffffff_ffffffff);
        } else {
            // movsd m64, xmm1 — store ONLY the low 64 bits (8 bytes).
            let src = match emu.get_operand_xmm_value_128(ins, 1, true) {
                Some(v) => v,
                None => return false,
            };
            emu.set_operand_value(ins, 0, src as u64);
        }
    } else {
        // legacy mode of movsd

        if emu.rep.is_some() {
            if emu.rep.unwrap() == 0 {
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
            let val = emu
                .maps
                .read_dword(emu.regs().rsi)
                .expect("cannot read memory");

            emu.maps.write_dword(emu.regs().rdi, val);

            if !emu.flag_df() {
                emu.regs_mut().rsi += 4;
                emu.regs_mut().rdi += 4;
            } else {
                emu.regs_mut().rsi -= 4;
                emu.regs_mut().rdi -= 4;
            }
        } else {
            // 32bits

            let val = match emu.maps.read_dword(emu.regs().get_esi()) {
                Some(v) => v,
                None => {
                    log::trace!("cannot read memory at esi");
                    return false;
                }
            };
            emu.maps.write_dword(emu.regs().get_edi(), val);

            if !emu.flag_df() {
                let esi = emu.regs().get_esi() + 4;
                let edi = emu.regs().get_edi() + 4;
                emu.regs_mut().set_esi(esi);
                emu.regs_mut().set_edi(edi);
            } else {
                let esi = emu.regs().get_esi() - 4;
                let edi = emu.regs().get_edi() - 4;
                emu.regs_mut().set_esi(esi);
                emu.regs_mut().set_edi(edi);
            }
        }
    }
    true
}
