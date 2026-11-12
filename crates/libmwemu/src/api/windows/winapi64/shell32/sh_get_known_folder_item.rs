use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn SHGetKnownFolderItem(emu: &mut emu::Emu) {
    let rfid = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let htoken = emu.regs().r8;
    let riid = emu.regs().r9;
    let ppv = WinApiAbi::arg(emu, 4);
    let hndl = helper::handler_create("IShellItem://knownfolder");
    if ppv != 0 {
        emu.maps.write_qword(ppv, hndl);
    }
    log_red!(
        emu,
        "shell32!SHGetKnownFolderItem flags:0x{:x} =0x{:x}",
        flags,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
