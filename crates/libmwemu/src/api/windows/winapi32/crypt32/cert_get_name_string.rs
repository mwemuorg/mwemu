use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CertGetNameString(emu: &mut emu::Emu) {
    let cert_ctx = WinApiAbi::arg(emu, 0);
    log_red!(emu, "crypt32!CertGetNameString ctx:0x{:x}", cert_ctx);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
