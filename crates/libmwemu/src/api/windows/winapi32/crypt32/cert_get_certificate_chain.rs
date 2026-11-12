use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CertGetCertificateChain(emu: &mut emu::Emu) {
    let engine = WinApiAbi::arg(emu, 0);
    let cert_ctx = WinApiAbi::arg(emu, 1);
    let time = WinApiAbi::arg(emu, 2);
    let add_store = WinApiAbi::arg(emu, 3);
    let chain_para = WinApiAbi::arg(emu, 4);
    let flags = WinApiAbi::arg(emu, 5);
    let _reserved = WinApiAbi::arg(emu, 6);
    let chain_ctx_ptr = WinApiAbi::arg(emu, 7);
    let hndl = helper::handler_create("cert_chain");
    if chain_ctx_ptr != 0 {
        emu.maps.write_dword(chain_ctx_ptr, hndl as u32);
    }
    log_red!(
        emu,
        "crypt32!CertGetCertificateChain cert:0x{:x} => 0x{:x}",
        cert_ctx,
        hndl
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 8);
}
