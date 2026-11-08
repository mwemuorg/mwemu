use crate::emu;

pub(super) fn crt_strrchr(emu: &mut emu::Emu) {
    let s = emu.regs().rcx;
    let c = emu.regs().rdx as u8;
    let str_val = emu.maps.read_string(s);
    let result = match str_val.rfind(c as char) {
        Some(pos) => s + pos as u64,
        None => 0,
    };
    emu.regs_mut().rax = result;
}
