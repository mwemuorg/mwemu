use crate::emu;

pub fn _get_errno(emu: &mut emu::Emu) {
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    log_red!(emu, "msvcrt!_get_errno => 0");

    emu.regs_mut().rax = 0;
}
