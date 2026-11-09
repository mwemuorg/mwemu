use crate::emu;

pub fn uaw_wcscpy(emu: &mut emu::Emu) {
    let dest = emu.regs().rcx;
    let src = emu.regs().rdx;

    let s = emu.maps.read_wide_string(src);
    log_red!(emu, "kernel32!uaw_wcscpy dest:0x{:x} src:'{}'", dest, s);

    if dest != 0 {
        emu.maps.write_wide_string(dest, &s);
    }
    emu.regs_mut().rax = dest;
}
