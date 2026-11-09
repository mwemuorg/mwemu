use crate::emu;

pub fn StringCchCopyW(emu: &mut emu::Emu) {
    let dest = emu.regs().rcx;
    let cch_dest = emu.regs().rdx;
    let src = emu.regs().r8;

    let s = emu.maps.read_wide_string(src);

    log_red!(
        emu,
        "kernel32!StringCchCopyW dest:0x{:x} cchDest:{} src:'{}'",
        dest,
        cch_dest,
        s
    );

    if dest != 0 && !s.is_empty() {
        emu.maps.write_wide_string(dest, &s);
    }
    emu.regs_mut().rax = 0;
}
