use crate::arch::x86::regs::U256;
use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(color!("LightCyan"), &DecodedInstruction::X86(*ins));

    // Only lane sign bits participate in the flag comparisons.
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
        let a = try_operand!(emu.get_operand_ymm_value_256(ins, 0, true));
        let b = try_operand!(emu.get_operand_ymm_value_256(ins, 1, true));
        let sm = U256::from_little_endian(&{
            let mut x = [0u8; 32];
            x[0..16].copy_from_slice(&sm128.to_le_bytes());
            x[16..32].copy_from_slice(&sm128.to_le_bytes());
            x
        });
        ((a & b & sm).is_zero(), (!a & b & sm).is_zero())
    } else {
        let a = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
        let b = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
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
