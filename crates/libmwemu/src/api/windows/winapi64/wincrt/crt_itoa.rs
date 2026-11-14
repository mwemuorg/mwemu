use crate::emu;

pub(super) fn crt_itoa(emu: &mut emu::Emu) {
    let val = emu.regs().rcx as i32;
    let buf = emu.regs().rdx;
    let radix = emu.regs().r8 as u32;
    let s = match radix {
        16 => format!("{:x}", val),
        8 => format!("{:o}", val),
        _ => format!("{}", val),
    };
    if buf != 0 {
        emu.maps.write_string(buf, &s);
    }
    log_red!(emu, "wincrt!_itoa {} radix:{} => '{}'", val, radix, s);
    emu.regs_mut().rax = buf;
}
