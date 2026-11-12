use crate::emu;
use crate::windows::constants::errors::*;

pub fn CreateDirectoryTransactedA(emu: &mut emu::Emu) {
    create_directory_transacted(emu, "CreateDirectoryTransactedA");
}

fn create_directory_transacted(emu: &mut emu::Emu, api: &str) {
    let _lpTemplateDirectory = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpNewDirectory = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpSecurityAttributes = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _hTransaction = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub fn CreateDirectoryTransactedW(emu: &mut emu::Emu) {
    create_directory_transacted(emu, "CreateDirectoryTransactedW");
}
