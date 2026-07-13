use crate::arch::x86::regs::U256;
use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// VPTEST: ZF=((a AND b)==0), CF=((b AND NOT a)==0), over 128 or 256 bits.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("LightCyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let (zf, cf) = if emu.get_operand_sz(ins, 0) == 256 {
        let a = emu
            .get_operand_ymm_value_256(ins, 0, true)
            .unwrap_or(U256::from(0));
        let b = emu
            .get_operand_ymm_value_256(ins, 1, true)
            .unwrap_or(U256::from(0));
        ((a & b).is_zero(), (b & !a).is_zero())
    } else {
        let a = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
        let b = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
        ((a & b) == 0, (b & !a) == 0)
    };
    let f = emu.flags_mut();
    f.f_zf = zf;
    f.f_cf = cf;
    f.f_of = false;
    f.f_sf = false;
    f.f_af = false;
    f.f_pf = false;
    true
}
