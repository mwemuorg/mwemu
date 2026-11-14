use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptSIPGetCaps(emu: &mut emu::Emu) {
    let p_subject_info = WinApiAbi::arg(emu, 0);
    let p_caps = WinApiAbi::arg(emu, 1);
    log_red!(
        emu,
        "crypt32!CryptSIPGetCaps p_subject_info:0x{:x} p_caps:0x{:x}",
        p_subject_info,
        p_caps
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
