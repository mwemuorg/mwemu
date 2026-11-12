use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn srand(emu: &mut emu::Emu) {
    let seed = WinApiAbi::arg(emu, 0) as u32;
    log_red!(emu, "msvcrt!srand seed:{}", seed);
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
