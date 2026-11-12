use crate::emu;

pub fn wcscpy(emu: &mut emu::Emu) {
    let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = emu.maps.read_wide_string(src);
    if dst != 0 {
        emu.maps.write_wide_string(dst, &s);
    }
    log_red!(emu, "msvcrt!wcscpy dst:0x{:x} '{}'", dst, s);

    emu.regs_mut().rax = dst;
}
