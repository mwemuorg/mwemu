use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

/// RDRAND — read a hardware-generated random value into the destination
/// register (16/32/64-bit) and report success in CF.
///
/// On real silicon CF=0 means "no entropy available, retry"; guests loop on
/// RDRAND until CF=1, so the emulator always succeeds to avoid a spin. The
/// value comes from the same `rand` source used elsewhere in mwemu.
pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let sz = emu.get_operand_sz(ins, 0);
    let rnd = rand::random::<u64>();
    let value = match sz {
        64 => rnd,
        32 => rnd & 0xffff_ffff,
        16 => rnd & 0xffff,
        _ => return false,
    };

    if !emu.set_operand_value(ins, 0, value) {
        return false;
    }

    // Success: CF=1; OF, SF, ZF, AF, PF are cleared (Intel SDM Vol. 2).
    let flags = emu.flags_mut();
    flags.f_cf = true;
    flags.f_of = false;
    flags.f_sf = false;
    flags.f_zf = false;
    flags.f_af = false;
    flags.f_pf = false;

    true
}
