use crate::emu;
use crate::windows::constants;

pub fn BackupSeek(emu: &mut emu::Emu) {
    let _hFile = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _dwLowBytesToSeek = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _dwHighBytesToSeek = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _lpdwLowByteSeeked = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _lpdwHighByteSeeked = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let _lpContext = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);

    log_red!(emu, "kernel32!BackupSeek");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
