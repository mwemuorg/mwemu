use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CertSrvIsServerOnlineW(emu: &mut emu::Emu) {
    let cert_ctx = WinApiAbi::arg(emu, 0);
    log_red!(emu, "crypt32!CertSrvIsServerOnlineW ctx:0x{:x}", cert_ctx);
    emu.set_last_error(0);
    let h = WinApiAbi::arg(emu, 0);
    emu.regs_mut().rax = 1;

    stdcall_ret(emu, 2);
}
