use crate::emu;
use crate::windows::constants;

pub fn DebugSetProcessKillOnExit(emu: &mut emu::Emu) {
    let _KillOnExit = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");

    log_red!(emu, "kernel32!DebugSetProcessKillOnExit");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
