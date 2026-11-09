use crate::emu;

pub(super) fn crt_strftime(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let max_size = emu.regs().rdx;
    let s = "01/01/2024 00:00:00";
    if dst != 0 && max_size > 0 {
        emu.maps.write_string(dst, s);
    }
    log_red!(emu, "wincrt!strftime => '{}'", s);
    emu.regs_mut().rax = s.len() as u64;
}
