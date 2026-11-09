use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn InternetSetStatusCallback(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    let callback = arg(emu, 1) as u64;
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
    stdcall_ret(emu, 2);
}
