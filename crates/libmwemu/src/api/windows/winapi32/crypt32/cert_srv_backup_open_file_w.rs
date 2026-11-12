use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CertSrvBackupOpenFileW(emu: &mut emu::Emu) {
    let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CertSrvBackupOpenFileW ctx:0x{:x}", cert_ctx);
    let handle = helper::handler_create("CertSrvBackupOpenFileW");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
