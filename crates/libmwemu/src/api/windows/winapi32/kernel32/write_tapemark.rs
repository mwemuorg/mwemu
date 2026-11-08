use crate::emu;
use crate::windows::constants;

pub fn WriteTapemark(emu: &mut emu::Emu) {
    let _hDevice = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _dwTapemarkType = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _dwTapemarkCount = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _bImmediate = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    log_red!(emu, "kernel32!WriteTapemark");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
