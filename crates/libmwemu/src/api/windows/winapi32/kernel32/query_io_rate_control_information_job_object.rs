use crate::emu;
use crate::windows::constants;

pub fn QueryIoRateControlInformationJobObject(emu: &mut emu::Emu) {
    let _hJob = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _VolumeName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _InfoBlocks = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _InfoBlockCount = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    log_red!(emu, "kernel32!QueryIoRateControlInformationJobObject");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
