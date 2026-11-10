use crate::emu;
use crate::windows::constants::errors::*;

pub fn GetNamedPipeServerSessionId(emu: &mut emu::Emu) {
    let _Pipe = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _ServerProcessId = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    log_red!(emu, "kernel32!GetNamedPipeServerSessionId");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
