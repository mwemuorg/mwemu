use crate::emu;

pub fn strcpy(emu: &mut emu::Emu) {
    let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let s = emu.maps.read_string(src);
    if dst != 0 {
        emu.maps.write_string(dst, &s);
    }
    log_red!(emu, "msvcrt!strcpy dst:0x{:x} '{}'", dst, s);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = dst;
}
