use crate::emu;
use crate::windows::constants;

pub fn TerminateProcess(emu: &mut emu::Emu) {
    let hndl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let code = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    log_red!(
        emu,
        "kernel32!TerminateProcess hndl: {} code: {}",
        hndl,
        code
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    if hndl == constants::CURRENT_PROCESS_HANDLE as u32 {
        emu.stop();
        return;
    }
    emu.regs_mut().rax = 1;
}
