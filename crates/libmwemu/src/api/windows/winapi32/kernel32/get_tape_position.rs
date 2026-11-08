use crate::emu;
use crate::windows::constants;

pub fn GetTapePosition(emu: &mut emu::Emu) {
    let _hDevice = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _dwPositionType = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpdwPartition = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _lpdwOffsetLow = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _lpdwOffsetHigh = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    log_red!(emu, "kernel32!GetTapePosition");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
