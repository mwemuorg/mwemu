use crate::emu;
use crate::windows::constants;

pub fn VerifyVersionInfoA(emu: &mut emu::Emu) {
    let _lpVersionInformation = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _dwTypeMask = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _dwlConditionMask = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    log_red!(emu, "kernel32!VerifyVersionInfoA");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
