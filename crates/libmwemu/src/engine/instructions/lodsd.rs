use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    //TODO: crash if arrive to zero or max value

    if emu.cfg.is_x64() {
        let val = match emu.maps.read_dword(emu.regs().rsi) {
            Some(v) => v,
            None => return false,
        };

        emu.regs_mut().set_eax(val as u64);
        if emu.flag_df() {
            emu.regs_mut().rsi -= 4;
        } else {
            emu.regs_mut().rsi += 4;
        }
    } else {
        let val = match emu.maps.read_dword(emu.regs().get_esi()) {
            Some(v) => v,
            None => return false,
        };

        emu.regs_mut().set_eax(val as u64);
        if emu.flag_df() {
            let esi = emu.regs().get_esi() - 4;
            emu.regs_mut().set_esi(esi);
        } else {
            let esi = emu.regs().get_esi() + 4;
            emu.regs_mut().set_esi(esi);
        }
    }
    true
}
