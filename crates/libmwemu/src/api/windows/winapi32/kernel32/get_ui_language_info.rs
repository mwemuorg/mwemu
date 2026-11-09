use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetUILanguageInfo(emu: &mut emu::Emu) {
    let flags = arg(emu, 0);
    let languages_ptr = arg(emu, 1) as u64;
    let fallback_ptr = arg(emu, 2) as u64;
    let fallback_size_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetUILanguageInfo flags:0x{:x} lang:0x{:x}",
        flags,
        languages_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
