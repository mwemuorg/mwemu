use crate::emu;

pub(super) fn crt_islower(emu: &mut emu::Emu) {
    let c = emu.regs().rcx as u8;
    emu.regs_mut().rax = if (c as char).is_ascii_lowercase() {
        1
    } else {
        0
    };
}
