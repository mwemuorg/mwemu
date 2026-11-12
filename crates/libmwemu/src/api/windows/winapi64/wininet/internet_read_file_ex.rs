use crate::emu;
use crate::winapi::helper;

pub(super) fn InternetReadFileExA(emu: &mut emu::Emu) {
    internet_read_file_ex(emu, "InternetReadFileExA");
}

pub(super) fn InternetReadFileExW(emu: &mut emu::Emu) {
    internet_read_file_ex(emu, "InternetReadFileExW");
}

fn internet_read_file_ex(emu: &mut emu::Emu, api: &str) {
    let file_hndl = emu.regs().rcx;
    let buff_ptr = emu.regs().rdx;
    log_red!(emu, "wininet!{api} buff: 0x{:x}", buff_ptr);
    if !helper::handler_exist(file_hndl) {
        log::trace!("\tinvalid handle.");
    }
    emu.regs_mut().rax = 1;
}
