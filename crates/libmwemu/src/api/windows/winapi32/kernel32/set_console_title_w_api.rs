use crate::emu;

pub fn SetConsoleTitleW_api(emu: &mut emu::Emu) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "kernel32!SetConsoleTitleW '{}'", s);
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
