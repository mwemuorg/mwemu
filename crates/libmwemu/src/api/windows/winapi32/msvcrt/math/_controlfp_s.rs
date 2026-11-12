use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn _controlfp_s(emu: &mut emu::Emu) {
    let p_current = WinApiAbi::arg(emu, 0);
    if p_current != 0 {
        emu.maps.write_dword(p_current, 0x0008_001f);
    }
    log_red!(emu, "msvcrt!_controlfp_s");
    emu.regs_mut().rax = 0;
}
