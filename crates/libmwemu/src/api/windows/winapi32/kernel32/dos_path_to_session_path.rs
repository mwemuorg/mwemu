use crate::emu;
use crate::windows::constants::errors::*;

pub fn DosPathToSessionPathA(emu: &mut emu::Emu) {
    dos_path_to_session_path(emu, "DosPathToSessionPathA");
}

fn dos_path_to_session_path(emu: &mut emu::Emu, api: &str) {
    let _a1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lp = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _a3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn DosPathToSessionPathW(emu: &mut emu::Emu) {
    dos_path_to_session_path(emu, "DosPathToSessionPathW");
}
