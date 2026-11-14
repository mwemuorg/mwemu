use crate::emu;

pub(super) fn crt_towupper(emu: &mut emu::Emu) {
    let c = emu.regs().rcx as u16;
    let ch = char::from_u32(c as u32).unwrap_or('\0');
    let upper = ch.to_ascii_uppercase() as u64;
    emu.regs_mut().rax = upper;
}
