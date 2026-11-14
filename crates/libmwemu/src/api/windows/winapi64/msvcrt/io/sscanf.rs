use crate::emu;

pub fn sscanf(emu: &mut emu::Emu, api: &str) {
    let buf = emu.regs().rcx;
    let fmt_ptr = emu.regs().rdx;
    let s = emu.maps.read_string(buf);
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(emu, "msvcrt!{} buf:'{}' fmt:'{}'", api, s, fmt);
    emu.regs_mut().rax = 1;
}
