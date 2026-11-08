use crate::emu;

pub fn sscanf_s(emu: &mut emu::Emu) {
    let buf_ptr = emu.regs().rcx;
    let fmt_ptr = emu.regs().rdx;
    let s = emu.maps.read_string(buf_ptr);
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(emu, "wincrt!sscanf_s buf:'{}' fmt:'{}'", s, fmt);
    emu.regs_mut().rax = 1;
}
