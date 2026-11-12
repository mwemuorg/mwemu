use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn _get_errno(emu: &mut emu::Emu) {
    let out_ptr = WinApiAbi::arg(emu, 0);
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    log_red!(emu, "msvcrt!_get_errno => 0");

    emu.regs_mut().rax = 0;
}
