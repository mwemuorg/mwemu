use crate::emu;

pub(super) fn _lock(emu: &mut emu::Emu) {
    let lock_num = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "msvcrt!_lock {}", lock_num);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
