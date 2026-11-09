use crate::emu;

pub fn StringCbCopyW(emu: &mut emu::Emu) {
    let dest = emu.regs().rcx;
    let cb_dest = emu.regs().rdx;
    let src = emu.regs().r8;

    let s = emu.maps.read_wide_string(src);

    log_red!(
        emu,
        "kernel32!StringCbCopyW dest:0x{:x} cbDest:{} src:'{}'",
        dest,
        cb_dest,
        s
    );

    if dest != 0 && !s.is_empty() {
        emu.maps.write_wide_string(dest, &s);
    }
    emu.regs_mut().rax = 0;
}
