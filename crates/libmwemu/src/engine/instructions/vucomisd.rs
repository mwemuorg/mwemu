use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// VUCOMISD: VEX scalar compare setting ZF/PF/CF; OF/SF/AF cleared.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("LightCyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let v1 = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let v2 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let a = f64::from_bits((v1 & 0xffff_ffff_ffff_ffff) as _);
    let b = f64::from_bits((v2 & 0xffff_ffff_ffff_ffff) as _);
    let f = emu.flags_mut();
    f.f_of = false;
    f.f_sf = false;
    f.f_af = false;
    f.f_zf = false;
    f.f_pf = false;
    f.f_cf = false;
    if a.is_nan() || b.is_nan() {
        let f = emu.flags_mut();
        f.f_zf = true;
        f.f_pf = true;
        f.f_cf = true;
    } else if a == b {
        emu.flags_mut().f_zf = true;
    } else if a < b {
        emu.flags_mut().f_cf = true;
    }
    true
}
