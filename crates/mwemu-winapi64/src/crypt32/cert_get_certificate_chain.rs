use crate::emu;
use crate::winapi::helper;

pub fn CertGetCertificateChain(emu: &mut emu::Emu) {
    let _h_chain_engine = emu.regs().rcx;
    let _p_cert_context = emu.regs().rdx;
    let _p_time = emu.regs().r8;
    let _h_additional_store = emu.regs().r9;
    let _p_chain_para = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _dw_flags = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let pp_chain_context = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);

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
