use crate::emu;

pub fn lstrcpy(emu: &mut emu::Emu) {
    let dst = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!lstrcpy: error reading dst") as u64;
    let src = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!lstrcpy: error reading src") as u64;
    let s = emu.maps.read_string(src);
    emu.maps.write_string(dst, &s);

    log_red!(emu, "kernel32!lstrcpy 0x{:x} `{}`", dst, s);

    emu.regs_mut().rax = dst;
    emu.stack_pop32(false);
    emu.stack_pop32(false);
}
