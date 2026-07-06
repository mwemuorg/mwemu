use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.rep.is_some() {
        if emu.rep.unwrap() == 0 || emu.cfg.verbose >= 3 {
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
            .read_word(emu.regs().rsi)
            .expect("cannot read memory");
        emu.maps.write_word(emu.regs().rdi, val);

        if !emu.flag_df() {
            emu.regs_mut().rsi += 2;
            emu.regs_mut().rdi += 2;
        } else {
            emu.regs_mut().rsi -= 2;
            emu.regs_mut().rdi -= 2;
        }
    } else {
        // 32bits
        let val = emu
            .maps
            .read_word(emu.regs().get_esi())
            .expect("cannot read memory");
        emu.maps.write_word(emu.regs().get_edi(), val);

        if !emu.flag_df() {
            let esi = emu.regs().get_esi() + 2;
            let edi = emu.regs().get_edi() + 2;
            emu.regs_mut().set_esi(esi);
            emu.regs_mut().set_edi(edi);
        } else {
            let esi = emu.regs().get_esi() - 2;
            let edi = emu.regs().get_edi() - 2;
            emu.regs_mut().set_esi(esi);
            emu.regs_mut().set_edi(edi);
        }
    }
    true
}
