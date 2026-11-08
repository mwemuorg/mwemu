use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn FreeLibraryAndExitThread(emu: &mut emu::Emu) {
    let module = arg(emu, 0) as u64;
    let exit_code = arg(emu, 1) as u64 as u32;
    log_red!(
        emu,
        "kernel32!FreeLibraryAndExitThread module:0x{:x} exitCode:{}",
        module,
        exit_code
    );
    helper::handler_close(module);
    stdcall_ret(emu, 2);
}
