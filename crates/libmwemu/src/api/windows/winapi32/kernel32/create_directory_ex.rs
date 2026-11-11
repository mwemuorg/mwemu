use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::errors::*;

pub fn CreateDirectoryExA(emu: &mut emu::Emu) {
    let _lpTemplateDirectory = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpNewDirectory = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpSecurityAttributes = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

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
