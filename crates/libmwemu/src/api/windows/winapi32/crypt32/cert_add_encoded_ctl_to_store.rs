use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CertAddEncodedCTLToStore(emu: &mut emu::Emu) {
    let cert_ctx = WinApiAbi::arg(emu, 0);
    log_red!(emu, "crypt32!CertAddEncodedCTLToStore ctx:0x{:x}", cert_ctx);
    let handle = helper::handler_create("CertAddEncodedCTLToStore");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 6);
}
