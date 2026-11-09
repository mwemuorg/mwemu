use crate::emu;
pub fn IsDBCSLeadByte(emu: &mut emu::Emu) {
    let test_char = emu.regs().rcx as u8;
    let is_lead = (0x81..=0x9f).contains(&test_char) || (0xe0..=0xfc).contains(&test_char);
    log_red!(
        emu,
        "kernel32!IsDBCSLeadByte char:0x{:02x} => {}",
        test_char,
        is_lead
    );
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
    emu.set_last_error(0);
    emu.regs_mut().rax = if is_lead { 1 } else { 0 };
}
