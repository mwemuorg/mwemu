use crate::emu;
use crate::winapi::helper;

pub(super) fn InternetReadFileExA(emu: &mut emu::Emu) {
    let file_hndl = emu.regs().rcx;
    let buff_ptr = emu.regs().rdx;
    let flags = emu.regs().r8;
    let ctx = emu.regs().r9;

    log_red!(emu, "wininet!InternetReadFileExA buff: 0x{:x}", buff_ptr);

    if !helper::handler_exist(file_hndl) {
        log::trace!("\tinvalid handle.");
    }

    emu.regs_mut().rax = 1; // true
}

pub(super) fn InternetReadFileExW(emu: &mut emu::Emu) {
    let file_hndl = emu.regs().rcx;
    let buff_ptr = emu.regs().rdx;
    let _flags = emu.regs().r8;
    let _ctx = emu.regs().r9;

    log_red!(emu, "wininet!InternetReadFileExW buff: 0x{:x}", buff_ptr);

    if !helper::handler_exist(file_hndl) {
        log::trace!("\tinvalid handle.");
    }

    emu.regs_mut().rax = 1; // true
}
