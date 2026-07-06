use crate::emu;
use crate::windows::constants;

pub fn TerminateProcess(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!TerminateProcess cannot read the handle");
    let code = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!TerminateProcess cannot read the exit code");

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
