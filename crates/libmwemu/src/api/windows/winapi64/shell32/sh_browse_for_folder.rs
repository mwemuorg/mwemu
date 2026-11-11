use crate::emu;
use crate::winapi::helper;

pub fn SHBrowseForFolder(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "shell32!SHBrowseForFolder arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("shbrowseforfolder");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}

pub fn SHBrowseForFolderA(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "shell32!SHBrowseForFolderA arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("shbrowseforfoldera");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}

pub fn SHBrowseForFolderW(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "shell32!SHBrowseForFolderW arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("shbrowseforfolderw");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}
