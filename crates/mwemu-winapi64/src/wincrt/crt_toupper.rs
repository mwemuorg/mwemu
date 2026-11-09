use crate::emu;

pub(super) fn crt_toupper(emu: &mut emu::Emu) {
    let c = emu.regs().rcx as u8;
    emu.regs_mut().rax = (c as char).to_ascii_uppercase() as u64;
}
