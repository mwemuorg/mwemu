use crate::emu;
use crate::windows::constants;

pub fn GetNamedPipeClientProcessId(emu: &mut emu::Emu) {
    let _Pipe = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _ClientProcessId = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    log_red!(emu, "kernel32!GetNamedPipeClientProcessId");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
