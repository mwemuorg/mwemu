use crate::{emu, windows::constants};

pub fn UnhandledExceptionFilter(emu: &mut emu::Emu) {
    let exception_info = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!UnhandledExceptionFilter  exception_info: 0x{:x}",
        exception_info
    );

    emu.regs_mut().rax = constants::EXCEPTION_EXECUTE_HANDLER as u64;
    // a debugger would had answered EXCEPTION_CONTINUE_SEARCH
}
