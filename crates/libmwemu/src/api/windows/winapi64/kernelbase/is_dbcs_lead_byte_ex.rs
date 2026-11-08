use crate::emu;
use crate::winapi::helper;

pub fn IsDBCSLeadByteEx(emu: &mut emu::Emu) {
    let _codepage = emu.regs().rcx;
    let test_char = emu.regs().rdx;
    let is_lead = (0x81..=0x9f).contains(&test_char) || (0xe0..=0xfc).contains(&test_char);
    let result = if is_lead { 1u64 } else { 0u64 };
    log_red!(
        emu,
        "kernelbase!IsDBCSLeadByteEx 0x{:x} => {}",
        test_char,
        result
    );
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
    emu.regs_mut().rax = result;
}
