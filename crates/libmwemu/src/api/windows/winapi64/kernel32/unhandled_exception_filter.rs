use crate::emu;
use crate::windows::constants;

pub fn UnhandledExceptionFilter(emu: &mut emu::Emu) {
    let exception_info = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!UnhandledExceptionFilter pExceptionInfo:0x{:x}",
        exception_info
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::EXCEPTION_EXECUTE_HANDLER as u64;
    // a debugger would had answered EXCEPTION_CONTINUE_SEARCH
}
