use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::exceptions::*;

pub fn UnhandledExceptionFilter(emu: &mut emu::Emu) {
    let exception_info = WinApiAbi::arg(emu, 0) as u32;

    log_red!(
        emu,
        "kernel32!UnhandledExceptionFilter  exception_info: 0x{:x}",
        exception_info
    );

    emu.stack_pop32(false);
    emu.regs_mut().rax = EXCEPTION_EXECUTE_HANDLER as u64;
    // a debugger would had answered EXCEPTION_CONTINUE_SEARCH
}
