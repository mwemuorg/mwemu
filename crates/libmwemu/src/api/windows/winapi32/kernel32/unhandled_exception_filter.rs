use crate::emu;
use crate::windows::constants;

pub fn UnhandledExceptionFilter(emu: &mut emu::Emu) {
    let exception_info = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!UnhandledExceptionFilter cannot read exception_info");

    log_red!(
        emu,
        "kernel32!UnhandledExceptionFilter  exception_info: 0x{:x}",
        exception_info
    );

    emu.stack_pop32(false);
    emu.regs_mut().rax = constants::EXCEPTION_EXECUTE_HANDLER as u64;
    // a debugger would had answered EXCEPTION_CONTINUE_SEARCH
}
