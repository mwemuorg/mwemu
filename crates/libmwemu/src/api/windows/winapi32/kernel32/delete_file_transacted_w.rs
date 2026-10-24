use crate::emu;
use crate::windows::constants;

pub fn DeleteFileTransactedW(emu: &mut emu::Emu) {
    let lpFileName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _hTransaction = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let lpFileName = emu.maps.read_wide_string(lpFileName as u64);

    log_red!(emu, "kernel32!DeleteFileTransactedW {}", lpFileName);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
