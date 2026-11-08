use crate::emu;

pub fn IsCharAlphaNumericA(emu: &mut emu::Emu) {
    let c = emu.regs().rcx as u8 as char;

    log_red!(emu, "kernelbase!IsCharAlphaNumericA char: {}", c);

    emu.regs_mut().rax = if c.is_ascii_alphanumeric() { 1 } else { 0 };
}
