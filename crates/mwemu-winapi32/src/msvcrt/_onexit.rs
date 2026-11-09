use crate::emu;

pub(super) fn _onexit(emu: &mut emu::Emu) {
    let fptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    log_red!(emu, "msvcrt!_onexit 0x{:x}", fptr);

    emu.set_last_error(0);
    emu.regs_mut().rax = fptr;
}
