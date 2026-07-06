use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let addr = match emu.get_operand_value(ins, 0, false) {
        Some(v) => v,
        None => return false,
    };

    if emu.cfg.is_x64() {
        let env = emu.fpu_mut().get_env64();

        for i in 0..4 {
            emu.maps.write_qword(addr + (i * 4), env[i as usize]);
        }
    } else {
        let env = emu.fpu_mut().get_env32();
        for i in 0..4 {
            emu.maps.write_dword(addr + (i * 4), env[i as usize]);
        }
    }

    emu.sync_fpu_ip();
    true
}
