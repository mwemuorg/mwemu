use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    assert!(ins.op_count() == 1);

    let value0 = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let value1;
    let sz = emu.get_operand_sz(ins, 0);

    if sz == 32 {
        value1 = (value0 & 0x00000000_000000ff) << 24
            | (value0 & 0x00000000_0000ff00) << 8
            | (value0 & 0x00000000_00ff0000) >> 8
            | (value0 & 0x00000000_ff000000) >> 24
            | (value0 & 0xffffffff_00000000);
    } else if sz == 64 {
        value1 = (value0 & 0xff000000_00000000) >> 56
            | (value0 & 0x00ff0000_00000000) >> 40
            | (value0 & 0x0000ff00_00000000) >> 24
            | (value0 & 0x000000ff_00000000) >> 8
            | (value0 & 0x00000000_ff000000) << 8
            | (value0 & 0x00000000_00ff0000) << 24
            | (value0 & 0x00000000_0000ff00) << 40
            | (value0 & 0x00000000_000000ff) << 56;
    } else if sz == 16 {
        value1 = 0;
        if emu.cfg.verbose >= 1 {
            log::trace!("/!\\ bswap of 16bits has undefined behaviours");
        }
    } else {
        unimplemented!("bswap <16bits makes no sense, isn't it?");
    }

    /*
    for i in 0..sz {
        let bit = get_bit!(value0, i);
        set_bit!(value1, sz-i-1, bit);
    }*/

    if !emu.set_operand_value(ins, 0, value1) {
        return false;
    }
    true
}
