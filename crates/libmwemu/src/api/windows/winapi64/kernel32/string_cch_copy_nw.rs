use crate::emu;

pub fn StringCchCopyNW(emu: &mut emu::Emu) {
    let dest = emu.regs().rcx;
    let cch_dest = emu.regs().rdx;
    let src = emu.regs().r8;
    let cch_to_copy = emu.regs().r9 as usize;

    let s = emu.maps.read_wide_string(src);

    log_red!(
        emu,
        "kernel32!StringCchCopyNW dest:0x{:x} cchDest:{} src:'{}' cchToCopy:{}",
        dest,
        cch_dest,
        s,
        cch_to_copy
    );

    if dest != 0 && !s.is_empty() {
        let truncated: String = s.chars().take(cch_to_copy).collect();
        emu.maps.write_wide_string(dest, &truncated);
    }
    emu.regs_mut().rax = 0;
}
