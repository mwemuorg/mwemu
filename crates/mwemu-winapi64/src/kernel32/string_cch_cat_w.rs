use crate::emu;

pub fn StringCchCatW(emu: &mut emu::Emu) {
    let dest = emu.regs().rcx;
    let cch_dest = emu.regs().rdx;
    let src = emu.regs().r8;

    let existing = emu.maps.read_wide_string(dest);
    let append = emu.maps.read_wide_string(src);

    log_red!(
        emu,
        "kernel32!StringCchCatW dest:0x{:x} cchDest:{} existing:'{}' append:'{}'",
        dest,
        cch_dest,
        existing,
        append
    );

    if dest != 0 {
        let combined = format!("{}{}", existing, append);
        emu.maps.write_wide_string(dest, &combined);
    }
    emu.regs_mut().rax = 0;
}
