use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn CreateDirectoryTransactedA(emu: &mut emu::Emu) {
    create_directory_transacted(emu, "CreateDirectoryTransactedA");
}

fn create_directory_transacted(emu: &mut emu::Emu, api: &str) {
    let _lpTemplateDirectory = WinApiAbi::arg(emu, 0) as u32;
    let _lpNewDirectory = WinApiAbi::arg(emu, 1) as u32;
    let _lpSecurityAttributes = WinApiAbi::arg(emu, 2) as u32;
    let _hTransaction = WinApiAbi::arg(emu, 3) as u32;

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub fn CreateDirectoryTransactedW(emu: &mut emu::Emu) {
    create_directory_transacted(emu, "CreateDirectoryTransactedW");
}
