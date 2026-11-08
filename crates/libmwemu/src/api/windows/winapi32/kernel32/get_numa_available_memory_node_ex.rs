use crate::emu;
use crate::windows::constants;

pub fn GetNumaAvailableMemoryNodeEx(emu: &mut emu::Emu) {
    let _Node = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _AvailableBytes = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    log_red!(emu, "kernel32!GetNumaAvailableMemoryNodeEx");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
