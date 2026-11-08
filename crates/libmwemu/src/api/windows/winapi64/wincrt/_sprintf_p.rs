use crate::emu;

pub fn _sprintf_p(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let buf_size = emu.regs().rdx as usize;
    let fmt_ptr = emu.regs().r8;
    let fmt = emu.maps.read_string(fmt_ptr);
    if buf != 0 {
        emu.maps.write_string(buf, &fmt);
    }
    log_red!(emu, "wincrt!_sprintf_p buf:0x{:x} fmt:'{}'", buf, fmt);
    emu.regs_mut().rax = fmt.len() as u64;
}
