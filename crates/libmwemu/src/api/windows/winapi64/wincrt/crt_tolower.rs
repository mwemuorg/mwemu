use crate::emu;

pub(super) fn crt_tolower(emu: &mut emu::Emu) {
    let c = emu.regs().rcx as u8;
    emu.regs_mut().rax = (c as char).to_ascii_lowercase() as u64;
}
