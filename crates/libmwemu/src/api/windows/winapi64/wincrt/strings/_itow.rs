use crate::emu;

pub fn _itow(emu: &mut emu::Emu) {
    let value = emu.regs().rcx as i32;
    let buf = emu.regs().rdx;
    let radix = emu.regs().r8 as u32;
    let s = if radix == 16 {
        format!("{:x}", value)
    } else {
        format!("{}", value)
    };
    if buf != 0 {
        emu.maps.write_wide_string(buf, &s);
    }
    log_red!(emu, "wincrt!_itow {} radix:{} => '{}'", value, radix, s);
    emu.regs_mut().rax = buf;
}
