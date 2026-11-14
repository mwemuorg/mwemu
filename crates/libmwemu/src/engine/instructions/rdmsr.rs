use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

/// `rdmsr`: edx:eax = MSR[ecx].
///
/// With a driver loaded the MSR file is modelled (and the read logged as a
/// privileged operation); in user mode only the one MSR shellcode probes is
/// answered, as before.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let index = emu.regs().rcx as u32;
    if emu.kernel.as_ref().is_some_and(|k| !k.ledger_only) {
        let v = emu.kernel_msr_read(index);
        emu.regs_mut().rax = v & 0xffff_ffff;
        emu.regs_mut().rdx = v >> 32;
        return true;
    }

    match index {
        0x176 => {
            emu.regs_mut().rdx = 0;
            emu.regs_mut().rax = emu.cfg.code_base_addr + 0x42;
        }
        _ => {
            log::trace!("/!\\ unimplemented rdmsr with value {}", emu.regs().rcx);
            return false;
        }
    }
    true
}
