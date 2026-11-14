use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn SHBrowseForFolder(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(
        emu,
        "shell32!SHBrowseForFolder arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("shbrowseforfolder");
    emu.regs_mut().rax = handle;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}

pub fn SHBrowseForFolderA(emu: &mut emu::Emu) {
    sh_browse_for_folder(emu, "SHBrowseForFolderA", false);
}

pub fn SHBrowseForFolderW(emu: &mut emu::Emu) {
    sh_browse_for_folder(emu, "SHBrowseForFolderW", true);
}

fn sh_browse_for_folder(emu: &mut emu::Emu, api: &str, wide: bool) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    if wide {
        let handle = helper::handler_create("shbrowseforfolderw");
        emu.regs_mut().rax = handle;
        stdcall_ret(emu, 1);
    } else {
        let handle = helper::handler_create("shbrowseforfoldera");
        emu.regs_mut().rax = handle;
        stdcall_ret(emu, 1);
    }
}
