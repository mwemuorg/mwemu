use crate::emu;

pub fn _get_errno(emu: &mut emu::Emu) {
    let out_ptr = emu.regs().rcx;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    log_red!(emu, "msvcrt!_get_errno => 0");
    emu.regs_mut().rax = 0;
}
