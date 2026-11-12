use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CertGetEnhancedKeyUsage(emu: &mut emu::Emu) {
    let p_cert_ctx = WinApiAbi::arg(emu, 0);
    let dw_flags = WinApiAbi::arg(emu, 1);
    let p_usage = WinApiAbi::arg(emu, 2);
    let pcb_usage = WinApiAbi::arg(emu, 3);
    log_red!(emu, "crypt32!CertGetEnhancedKeyUsage 0x{:x}", p_cert_ctx);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
