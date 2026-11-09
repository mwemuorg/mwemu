use crate::emu;
use crate::winapi::helper;

pub fn InternetSetStatusCallback(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let callback = emu.regs().rdx;
    let valid = helper::handler_exist(handle);
    log_red!(
        emu,
        "wininet!InternetSetStatusCallback handle: 0x{:x} callback: 0x{:x} valid: {}",
        handle,
        callback,
        valid
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
