use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CertGetCertificateChain(emu: &mut emu::Emu) {
    let _h_chain_engine = emu.regs().rcx;
    let _p_cert_context = emu.regs().rdx;
    let _p_time = emu.regs().r8;
    let _h_additional_store = emu.regs().r9;
    let _p_chain_para = WinApiAbi::arg(emu, 4);
    let _dw_flags = WinApiAbi::arg(emu, 5);
    let pp_chain_context = WinApiAbi::arg(emu, 6);

    let chain_h = helper::handler_create("cert_chain");
    log_red!(
        emu,
        "crypt32!CertGetCertificateChain => chain=0x{:x}",
        chain_h
    );
    if pp_chain_context != 0 {
        emu.maps.write_qword(pp_chain_context, chain_h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
