use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn CreateDirectoryExA(emu: &mut emu::Emu) {
    let _lpTemplateDirectory = WinApiAbi::arg(emu, 0) as u32;
    let _lpNewDirectory = WinApiAbi::arg(emu, 1) as u32;
    let _lpSecurityAttributes = WinApiAbi::arg(emu, 2) as u32;

    log_red!(emu, "kernel32!CreateDirectoryExA");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn CreateDirectoryExW(emu: &mut emu::Emu) {
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateDirectoryExW");
    emu.regs_mut().rax = helper::handler_create("CreateDirectoryExW");
}
