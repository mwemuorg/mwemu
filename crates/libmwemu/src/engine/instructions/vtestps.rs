use crate::arch::x86::regs::U256;
use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// VTESTPS: ZF=((a AND b) sign bits==0), CF=((NOT a AND b) sign bits==0).
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("LightCyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let sm128: u128 = {
        let mut m = 0u128;
        let mut s = 0u32;
        while s < 128 {
            m |= (0x8000_0000u128) << s;
            s += 32;
        }
        m
    };
    let (zf, cf) = if emu.get_operand_sz(ins, 0) == 256 {
        let a = emu
            .get_operand_ymm_value_256(ins, 0, true)
            .unwrap_or(U256::from(0));
        let b = emu
            .get_operand_ymm_value_256(ins, 1, true)
            .unwrap_or(U256::from(0));
        let sm = U256::from_little_endian(&{
            let mut x = [0u8; 32];
            x[0..16].copy_from_slice(&sm128.to_le_bytes());
            x[16..32].copy_from_slice(&sm128.to_le_bytes());
            x
        });
        ((a & b & sm).is_zero(), (!a & b & sm).is_zero())
    } else {
        let a = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
        let b = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
        ((a & b & sm128) == 0, (!a & b & sm128) == 0)
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
