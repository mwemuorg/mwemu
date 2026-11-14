use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptSIPLoad(emu: &mut emu::Emu) {
    let pg_subject = WinApiAbi::arg(emu, 0);
    let dw_flags = WinApiAbi::arg(emu, 1);
    let p_sip_dispatch = WinApiAbi::arg(emu, 2);
    log_red!(
        emu,
        "crypt32!CryptSIPLoad pg_subject:0x{:x} dw_flags:0x{:x} p_sip_dispatch:0x{:x}",
        pg_subject,
        dw_flags,
        p_sip_dispatch
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
