use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CertFindCTLInStore(emu: &mut emu::Emu) {
    let h_cert_store = WinApiAbi::arg(emu, 0);
    let dw_msg_and_cert_encoding = WinApiAbi::arg(emu, 1);
    let dw_find_flags = WinApiAbi::arg(emu, 2);
    let dw_find_type = WinApiAbi::arg(emu, 3);
    let pv_find_para = WinApiAbi::arg(emu, 4);
    let p_prev_ctl_ctx = WinApiAbi::arg(emu, 5);
    log_red!(emu, "crypt32!CertFindCTLInStore 0x{:x}", h_cert_store);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
