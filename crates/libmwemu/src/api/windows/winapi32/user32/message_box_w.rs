use crate::emu;

pub(super) fn MessageBoxW(emu: &mut emu::Emu) {
    let titleptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let msgptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let msg = emu.maps.read_wide_string(msgptr);
    let title = emu.maps.read_wide_string(titleptr);

    log_red!(emu, "user32!MessageBoxW {} {}", title, msg);

    emu.regs_mut().rax = 0;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
