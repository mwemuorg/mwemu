use crate::emu;
use crate::windows::constants;

pub fn GetDefaultCommConfigW(emu: &mut emu::Emu) {
    let lpszName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpCC = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpdwSize = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    let lpszName = emu.maps.read_wide_string(lpszName as u64);

    log_red!(emu, "kernel32!GetDefaultCommConfigW {}", lpszName);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
