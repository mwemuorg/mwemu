use crate::emu;

pub(super) fn _initterm(emu: &mut emu::Emu) {
    let start_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let end_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    log_red!(emu, "msvcrt!_initterm 0x{:x} - 0x{:x}", start_ptr, end_ptr);

    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
