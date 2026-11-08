use crate::emu;
use crate::windows::constants;

pub fn GlobalAddAtomExA(emu: &mut emu::Emu) {
    let lpString = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _Flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let lpString = emu.maps.read_string(lpString as u64);

    log_red!(emu, "kernel32!GlobalAddAtomExA {}", lpString);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
