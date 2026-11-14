use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CertAddCTLContextToStore(emu: &mut emu::Emu) {
    let h_cert_store = WinApiAbi::arg(emu, 0);
    let p_ctl_ctx = WinApiAbi::arg(emu, 1);
    let dw_add_disposition = WinApiAbi::arg(emu, 2);
    let pp_store_ctx = WinApiAbi::arg(emu, 3);
    log_red!(emu, "crypt32!CertAddCTLContextToStore 0x{:x}", h_cert_store);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
