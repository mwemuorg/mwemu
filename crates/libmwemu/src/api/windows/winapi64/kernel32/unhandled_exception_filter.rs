use crate::emu;
use crate::windows::constants::errors::*;

pub fn UnhandledExceptionFilter(emu: &mut emu::Emu) {
    let exception_info = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!UnhandledExceptionFilter pExceptionInfo:0x{:x}",
        exception_info
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = EXCEPTION_EXECUTE_HANDLER as u64;
    // a debugger would had answered EXCEPTION_CONTINUE_SEARCH
}
